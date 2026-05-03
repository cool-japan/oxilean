// actor_model/types.rs — Actor-based concurrency model types

use std::collections::HashMap;

/// Unique actor identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActorId(pub u64);

impl std::fmt::Display for ActorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actor({})", self.0)
    }
}

/// Control messages for actor lifecycle management
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlMsg {
    /// Stop the target actor
    Stop,
    /// Restart the target actor
    Restart,
    /// Link sender to the given actor (bidirectional failure propagation)
    Link(ActorId),
    /// Remove link between sender and given actor
    Unlink(ActorId),
    /// Monitor another actor (unidirectional observation)
    Monitor(ActorId),
    /// Remove a monitor
    Demonitor(ActorId),
}

/// The kind of a message flowing through the system
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageKind {
    /// Arbitrary data payload
    Data(String),
    /// Actor lifecycle control
    Control(ControlMsg),
    /// Error notification
    Error(String),
    /// Liveness probe
    Ping,
    /// Liveness reply
    Pong,
}

/// A message envelope
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub from: ActorId,
    pub to: ActorId,
    pub kind: MessageKind,
    /// Monotonically increasing sequence number assigned by the system
    pub seq: u64,
}

/// Lifecycle state of an actor
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActorState {
    Idle,
    Running,
    Waiting,
    Stopped,
    Failed(String),
}

impl std::fmt::Display for ActorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActorState::Idle => write!(f, "Idle"),
            ActorState::Running => write!(f, "Running"),
            ActorState::Waiting => write!(f, "Waiting"),
            ActorState::Stopped => write!(f, "Stopped"),
            ActorState::Failed(reason) => write!(f, "Failed({})", reason),
        }
    }
}

/// Supervisor restart strategy
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupervisorStrategy {
    /// Restart only the failed child
    OneForOne,
    /// Restart all children when one fails
    OneForAll,
    /// Restart the failed child and all children started after it
    RestForOne,
}

/// Describes how an actor processes messages
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorBehavior {
    /// No persistent state between messages
    Stateless,
    /// Carries serialized state string
    Stateful { state: String },
    /// Supervises child actors
    Supervisor { strategy: SupervisorStrategy },
}

/// Metadata stored per registered actor
#[derive(Debug, Clone)]
pub struct ActorInfo {
    pub id: ActorId,
    pub name: String,
    pub state: ActorState,
    pub behavior: ActorBehavior,
    /// Total messages ever processed or enqueued for this actor
    pub message_count: u64,
    /// Bidirectional links (failure-propagation peers)
    pub links: Vec<ActorId>,
}

impl ActorInfo {
    pub fn new(id: ActorId, name: String, behavior: ActorBehavior) -> Self {
        Self {
            id,
            name,
            state: ActorState::Idle,
            behavior,
            message_count: 0,
            links: Vec::new(),
        }
    }
}

/// The top-level actor system: registry + mailboxes
#[derive(Debug, Default)]
pub struct ActorSystem {
    pub actors: HashMap<ActorId, ActorInfo>,
    pub mailboxes: HashMap<ActorId, Vec<Message>>,
    /// Counter used to assign the next unique `ActorId`
    pub next_id: u64,
    /// Global message sequence counter
    pub(super) seq: u64,
}
