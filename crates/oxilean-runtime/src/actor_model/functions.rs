// actor_model/functions.rs — Actor system implementation

use super::types::{
    ActorBehavior, ActorId, ActorInfo, ActorState, ActorSystem, Message, MessageKind,
};
use std::collections::HashMap;

impl ActorSystem {
    /// Create a new, empty actor system.
    pub fn new() -> Self {
        Self::default()
    }

    /// Spawn a new actor, returning its assigned `ActorId`.
    pub fn spawn(&mut self, name: &str, behavior: ActorBehavior) -> ActorId {
        let id = ActorId(self.next_id);
        self.next_id += 1;
        let info = ActorInfo::new(id, name.to_owned(), behavior);
        self.actors.insert(id, info);
        self.mailboxes.insert(id, Vec::new());
        id
    }

    /// Enqueue a message from `from` to `to`.
    ///
    /// Returns `true` if the destination actor exists and the message was delivered,
    /// `false` if the destination is unknown or stopped/failed.
    pub fn send(&mut self, from: ActorId, to: ActorId, kind: MessageKind) -> bool {
        // Validate destination exists and is reachable
        let reachable = self.actors.get(&to).map_or(false, |info| {
            !matches!(info.state, ActorState::Stopped | ActorState::Failed(_))
        });
        if !reachable {
            return false;
        }

        let seq = self.seq;
        self.seq += 1;

        let msg = Message {
            from,
            to,
            kind,
            seq,
        };

        if let Some(mailbox) = self.mailboxes.get_mut(&to) {
            mailbox.push(msg);
            // Increment message_count on the destination actor
            if let Some(info) = self.actors.get_mut(&to) {
                info.message_count += 1;
                if info.state == ActorState::Idle {
                    info.state = ActorState::Waiting;
                }
            }
            true
        } else {
            false
        }
    }

    /// Dequeue the oldest message from `id`'s mailbox.
    ///
    /// Returns `None` if the mailbox is empty or the actor does not exist.
    pub fn receive(&mut self, id: ActorId) -> Option<Message> {
        let mailbox = self.mailboxes.get_mut(&id)?;
        if mailbox.is_empty() {
            // Transition back to Idle if no messages remain
            if let Some(info) = self.actors.get_mut(&id) {
                if info.state == ActorState::Waiting {
                    info.state = ActorState::Idle;
                }
            }
            return None;
        }
        let msg = mailbox.remove(0);
        // Update actor state after consuming
        if let Some(info) = self.actors.get_mut(&id) {
            if mailbox.is_empty() {
                info.state = ActorState::Idle;
            } else {
                info.state = ActorState::Running;
            }
        }
        Some(msg)
    }

    /// Gracefully stop an actor: drain its mailbox and mark it `Stopped`.
    pub fn stop(&mut self, id: ActorId) {
        if let Some(info) = self.actors.get_mut(&id) {
            info.state = ActorState::Stopped;
        }
        // Clear pending messages
        if let Some(mailbox) = self.mailboxes.get_mut(&id) {
            mailbox.clear();
        }
        // Notify linked actors of the stop via an Error message (best-effort)
        let linked: Vec<ActorId> = self
            .actors
            .get(&id)
            .map(|info| info.links.clone())
            .unwrap_or_default();
        for peer in linked {
            let seq = self.seq;
            self.seq += 1;
            let notice = Message {
                from: id,
                to: peer,
                kind: MessageKind::Error(format!("linked actor {} stopped", id)),
                seq,
            };
            if let Some(mailbox) = self.mailboxes.get_mut(&peer) {
                mailbox.push(notice);
                if let Some(peer_info) = self.actors.get_mut(&peer) {
                    peer_info.message_count += 1;
                }
            }
        }
    }

    /// Create a bidirectional failure-propagation link between actors `a` and `b`.
    pub fn link(&mut self, a: ActorId, b: ActorId) {
        if let Some(info_a) = self.actors.get_mut(&a) {
            if !info_a.links.contains(&b) {
                info_a.links.push(b);
            }
        }
        if let Some(info_b) = self.actors.get_mut(&b) {
            if !info_b.links.contains(&a) {
                info_b.links.push(a);
            }
        }
    }

    /// Return a map of state name → count of actors in that state.
    pub fn stats(&self) -> HashMap<String, usize> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for info in self.actors.values() {
            let key = match &info.state {
                ActorState::Idle => "Idle".to_owned(),
                ActorState::Running => "Running".to_owned(),
                ActorState::Waiting => "Waiting".to_owned(),
                ActorState::Stopped => "Stopped".to_owned(),
                ActorState::Failed(r) => format!("Failed({})", r),
            };
            *counts.entry(key).or_insert(0) += 1;
        }
        counts
    }

    /// Return the number of pending messages in `id`'s mailbox.
    pub fn mailbox_size(&self, id: ActorId) -> usize {
        self.mailboxes.get(&id).map_or(0, |v| v.len())
    }

    /// Send `kind` from `from` to every other actor in the system.
    ///
    /// Returns the number of actors that successfully received the message.
    pub fn broadcast(&mut self, from: ActorId, kind: MessageKind) -> usize {
        let targets: Vec<ActorId> = self
            .actors
            .keys()
            .copied()
            .filter(|&id| id != from)
            .collect();

        let mut delivered = 0usize;
        for to in targets {
            if self.send(from, to, kind.clone()) {
                delivered += 1;
            }
        }
        delivered
    }

    /// Return all link-graph edges as `(a, b)` pairs (each undirected edge appears once).
    pub fn topology(&self) -> Vec<(ActorId, ActorId)> {
        let mut edges: Vec<(ActorId, ActorId)> = Vec::new();
        for info in self.actors.values() {
            for &peer in &info.links {
                // Emit each edge only once: smaller id first
                if info.id < peer {
                    edges.push((info.id, peer));
                }
            }
        }
        edges.sort();
        edges
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::{
        ActorBehavior, ActorId, ActorState, ActorSystem, ControlMsg, MessageKind,
        SupervisorStrategy,
    };

    fn make_system() -> ActorSystem {
        ActorSystem::new()
    }

    // ── spawn / basic lifecycle ────────────────────────────────────────────────

    #[test]
    fn test_spawn_returns_unique_ids() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        assert_ne!(a, b);
        assert_eq!(a, ActorId(0));
        assert_eq!(b, ActorId(1));
    }

    #[test]
    fn test_spawn_actor_starts_idle() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        assert_eq!(sys.actors[&a].state, ActorState::Idle);
    }

    #[test]
    fn test_spawn_creates_empty_mailbox() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        assert_eq!(sys.mailbox_size(a), 0);
    }

    #[test]
    fn test_spawn_stateful_behavior() {
        let mut sys = make_system();
        let a = sys.spawn(
            "counter",
            ActorBehavior::Stateful {
                state: "0".to_owned(),
            },
        );
        assert!(matches!(
            sys.actors[&a].behavior,
            ActorBehavior::Stateful { .. }
        ));
    }

    #[test]
    fn test_spawn_supervisor_behavior() {
        let mut sys = make_system();
        let a = sys.spawn(
            "sup",
            ActorBehavior::Supervisor {
                strategy: SupervisorStrategy::OneForOne,
            },
        );
        assert!(matches!(
            sys.actors[&a].behavior,
            ActorBehavior::Supervisor { .. }
        ));
    }

    // ── send / receive ─────────────────────────────────────────────────────────

    #[test]
    fn test_send_and_receive_basic() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        let sent = sys.send(a, b, MessageKind::Data("hello".to_owned()));
        assert!(sent);
        let msg = sys.receive(b);
        assert!(msg.is_some());
        let msg = msg.unwrap();
        assert_eq!(msg.from, a);
        assert_eq!(msg.to, b);
        assert_eq!(msg.kind, MessageKind::Data("hello".to_owned()));
    }

    #[test]
    fn test_send_increments_message_count() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        sys.send(a, b, MessageKind::Ping);
        sys.send(a, b, MessageKind::Pong);
        assert_eq!(sys.actors[&b].message_count, 2);
    }

    #[test]
    fn test_send_to_unknown_actor_returns_false() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let ghost = ActorId(999);
        assert!(!sys.send(a, ghost, MessageKind::Ping));
    }

    #[test]
    fn test_send_to_stopped_actor_returns_false() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        sys.stop(b);
        assert!(!sys.send(a, b, MessageKind::Ping));
    }

    #[test]
    fn test_receive_fifo_order() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        sys.send(a, b, MessageKind::Data("first".to_owned()));
        sys.send(a, b, MessageKind::Data("second".to_owned()));
        let m1 = sys.receive(b).unwrap();
        let m2 = sys.receive(b).unwrap();
        assert_eq!(m1.kind, MessageKind::Data("first".to_owned()));
        assert_eq!(m2.kind, MessageKind::Data("second".to_owned()));
    }

    #[test]
    fn test_receive_empty_returns_none() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        assert!(sys.receive(a).is_none());
    }

    #[test]
    fn test_receive_unknown_actor_returns_none() {
        let mut sys = make_system();
        assert!(sys.receive(ActorId(42)).is_none());
    }

    #[test]
    fn test_seq_numbers_monotone() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        sys.send(a, b, MessageKind::Ping);
        sys.send(a, b, MessageKind::Pong);
        let m1 = sys.receive(b).unwrap();
        let m2 = sys.receive(b).unwrap();
        assert!(m1.seq < m2.seq);
    }

    // ── stop ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_stop_marks_actor_stopped() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        sys.stop(a);
        assert_eq!(sys.actors[&a].state, ActorState::Stopped);
    }

    #[test]
    fn test_stop_clears_mailbox() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        sys.send(a, b, MessageKind::Ping);
        sys.stop(b);
        assert_eq!(sys.mailbox_size(b), 0);
    }

    #[test]
    fn test_stop_notifies_linked_peers() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        sys.link(a, b);
        sys.stop(a);
        // bob should have received an error notice
        assert!(sys.mailbox_size(b) > 0);
        let msg = sys.receive(b).unwrap();
        assert!(matches!(msg.kind, MessageKind::Error(_)));
    }

    // ── link / topology ───────────────────────────────────────────────────────

    #[test]
    fn test_link_is_bidirectional() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        sys.link(a, b);
        assert!(sys.actors[&a].links.contains(&b));
        assert!(sys.actors[&b].links.contains(&a));
    }

    #[test]
    fn test_link_idempotent() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        sys.link(a, b);
        sys.link(a, b);
        assert_eq!(sys.actors[&a].links.iter().filter(|&&x| x == b).count(), 1);
    }

    #[test]
    fn test_topology_edges_unique() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        let c = sys.spawn("carol", ActorBehavior::Stateless);
        sys.link(a, b);
        sys.link(b, c);
        sys.link(a, c);
        let edges = sys.topology();
        assert_eq!(edges.len(), 3);
    }

    #[test]
    fn test_topology_empty_when_no_links() {
        let mut sys = make_system();
        sys.spawn("alice", ActorBehavior::Stateless);
        sys.spawn("bob", ActorBehavior::Stateless);
        assert!(sys.topology().is_empty());
    }

    // ── stats ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_stats_counts_states() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        sys.stop(b);
        let _ = a;
        let stats = sys.stats();
        assert_eq!(stats.get("Idle").copied().unwrap_or(0), 1);
        assert_eq!(stats.get("Stopped").copied().unwrap_or(0), 1);
    }

    // ── broadcast ────────────────────────────────────────────────────────────

    #[test]
    fn test_broadcast_reaches_all_others() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        let c = sys.spawn("carol", ActorBehavior::Stateless);
        let count = sys.broadcast(a, MessageKind::Ping);
        assert_eq!(count, 2);
        assert_eq!(sys.mailbox_size(b), 1);
        assert_eq!(sys.mailbox_size(c), 1);
    }

    #[test]
    fn test_broadcast_skips_stopped_actors() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        let c = sys.spawn("carol", ActorBehavior::Stateless);
        sys.stop(c);
        let count = sys.broadcast(a, MessageKind::Ping);
        assert_eq!(count, 1);
        assert_eq!(sys.mailbox_size(b), 1);
    }

    // ── control messages ──────────────────────────────────────────────────────

    #[test]
    fn test_send_control_msg() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        sys.send(a, b, MessageKind::Control(ControlMsg::Stop));
        let msg = sys.receive(b).unwrap();
        assert_eq!(msg.kind, MessageKind::Control(ControlMsg::Stop));
    }

    #[test]
    fn test_send_control_link_msg() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        sys.send(a, b, MessageKind::Control(ControlMsg::Link(a)));
        let msg = sys.receive(b).unwrap();
        assert_eq!(msg.kind, MessageKind::Control(ControlMsg::Link(a)));
    }

    // ── mailbox_size ──────────────────────────────────────────────────────────

    #[test]
    fn test_mailbox_size_unknown_actor_zero() {
        let sys = make_system();
        assert_eq!(sys.mailbox_size(ActorId(777)), 0);
    }

    // ── state transitions ─────────────────────────────────────────────────────

    #[test]
    fn test_actor_becomes_waiting_on_send() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        sys.send(a, b, MessageKind::Ping);
        assert_eq!(sys.actors[&b].state, ActorState::Waiting);
    }

    #[test]
    fn test_actor_returns_idle_after_drain() {
        let mut sys = make_system();
        let a = sys.spawn("alice", ActorBehavior::Stateless);
        let b = sys.spawn("bob", ActorBehavior::Stateless);
        sys.send(a, b, MessageKind::Ping);
        sys.receive(b);
        assert_eq!(sys.actors[&b].state, ActorState::Idle);
    }

    // ── supervisor strategy ───────────────────────────────────────────────────

    #[test]
    fn test_supervisor_strategies_exist() {
        let _ = SupervisorStrategy::OneForOne;
        let _ = SupervisorStrategy::OneForAll;
        let _ = SupervisorStrategy::RestForOne;
    }
}
