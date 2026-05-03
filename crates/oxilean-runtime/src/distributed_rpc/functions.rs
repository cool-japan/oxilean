// distributed_rpc/functions.rs — Distributed RPC network implementation

use super::types::{
    CircuitBreaker, CircuitState, NodeId, RpcNetwork, RpcRequest, RpcResponse, RpcResult,
    ServiceName,
};

// ── RpcNetwork ────────────────────────────────────────────────────────────────

impl RpcNetwork {
    /// Register `methods` for `service` on `node`.
    pub fn register(&mut self, node: NodeId, service: ServiceName, methods: Vec<String>) {
        self.registry.register(node, service, methods);
    }

    /// Simulate an RPC call, respecting node availability and latency.
    ///
    /// - If the destination node is down → `RpcResult::NodeDown`.
    /// - If the service/method is not registered on the target → `RpcResult::Err`.
    /// - Otherwise → `RpcResult::Ok` with a simulated echo response.
    pub fn call(&mut self, req: RpcRequest) -> RpcResponse {
        let latency = self
            .latency_ms
            .get(&(req.from, req.to))
            .copied()
            .unwrap_or(0);
        self.clock_ms += latency;

        let result = if self.downed_nodes.contains(&req.to) {
            RpcResult::NodeDown
        } else if !self.registry.has_method(req.to, &req.service, &req.method) {
            RpcResult::Err(format!(
                "method '{}' not found on service '{}' at {}",
                req.method, req.service, req.to
            ))
        } else {
            // Simulated successful response: echo the args as a comma-joined string
            let echo = req.args.join(",");
            RpcResult::Ok(echo)
        };

        let resp = RpcResponse {
            request_id: req.id,
            from: req.to,
            to: req.from,
            result,
        };
        self.message_log.push((req, resp.clone()));
        resp
    }

    /// Build a new `RpcRequest` and immediately call it, advancing the internal
    /// request counter.
    pub fn make_call(
        &mut self,
        from: NodeId,
        to: NodeId,
        service: ServiceName,
        method: impl Into<String>,
        args: Vec<String>,
    ) -> RpcResponse {
        let req = RpcRequest {
            id: self.next_req_id,
            from,
            to,
            service,
            method: method.into(),
            args,
        };
        self.next_req_id += 1;
        self.call(req)
    }

    /// Set the simulated one-way latency from `from` to `to`.
    pub fn set_latency(&mut self, from: NodeId, to: NodeId, ms: u64) {
        self.latency_ms.insert((from, to), ms);
    }

    /// Mark a node as unreachable.  Any call targeting it will return `NodeDown`.
    pub fn take_down(&mut self, node: NodeId) {
        self.downed_nodes.insert(node);
    }

    /// Restore a previously downed node to availability.
    pub fn bring_up(&mut self, node: NodeId) {
        self.downed_nodes.remove(&node);
    }

    /// Return all nodes that currently expose the given service.
    pub fn find_service(&self, service: &ServiceName) -> Vec<NodeId> {
        let mut nodes = self.registry.nodes_for(service);
        // Exclude downed nodes from discovery results
        nodes.retain(|n| !self.downed_nodes.contains(n));
        nodes.sort();
        nodes
    }

    /// Return the current simulated clock value in ms.
    pub fn clock(&self) -> u64 {
        self.clock_ms
    }
}

// ── CircuitBreaker ────────────────────────────────────────────────────────────

impl CircuitBreaker {
    /// Record the outcome of one call attempt at `current_time`.
    ///
    /// - **Closed**: failures accumulate; trip to `Open` when `threshold` is reached.
    /// - **Open**: auto-transitions to `HalfOpen` after `open_duration`; rejects calls.
    /// - **HalfOpen**: one allowed probe; success → `Closed`, failure → `Open`.
    ///
    /// Returns `true` if the call **should proceed** (circuit is closed or in probe mode),
    /// `false` if the call should be rejected.
    pub fn attempt(&mut self, success: bool, current_time: u64) -> bool {
        self.last_attempt = current_time;
        match &self.state {
            CircuitState::Closed => {
                if success {
                    self.failures = 0;
                    true
                } else {
                    self.failures += 1;
                    if self.failures >= self.threshold {
                        self.state = CircuitState::Open;
                    }
                    true // the call already happened; we report it
                }
            }
            CircuitState::Open => {
                // Check whether enough time has elapsed to probe
                let open_since = self.last_attempt.saturating_sub(self.open_duration);
                if current_time >= open_since + self.open_duration {
                    self.state = CircuitState::HalfOpen;
                    // Allow the probe through
                    if success {
                        self.failures = 0;
                        self.state = CircuitState::Closed;
                    } else {
                        self.state = CircuitState::Open;
                    }
                    true
                } else {
                    false // still open; reject
                }
            }
            CircuitState::HalfOpen => {
                if success {
                    self.failures = 0;
                    self.state = CircuitState::Closed;
                } else {
                    self.failures += 1;
                    self.state = CircuitState::Open;
                }
                true
            }
        }
    }

    /// Return a reference to the current circuit state.
    pub fn state(&self) -> &CircuitState {
        &self.state
    }
}

// ── retry_with_backoff ────────────────────────────────────────────────────────

/// Compute an exponential back-off delay sequence.
///
/// Returns a `Vec` of `attempts` values: `[base_ms, base_ms*2, base_ms*4, …]`.
/// Each value is capped at `u64::MAX / 2` to prevent overflow.
pub fn retry_with_backoff(attempts: u32, base_ms: u64) -> Vec<u64> {
    (0..attempts)
        .map(|i| {
            // 2^i saturating multiply
            // 2^i via repeated multiplication, capped at u64::MAX
            let factor = if i < 64 { 1u64 << i } else { u64::MAX };
            base_ms.saturating_mul(factor)
        })
        .collect()
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::super::types::{
        CircuitBreaker, CircuitState, NodeId, RpcNetwork, RpcRequest, RpcResult, ServiceName,
    };
    use super::retry_with_backoff;

    fn make_network() -> RpcNetwork {
        let nodes = vec![NodeId(1), NodeId(2), NodeId(3)];
        RpcNetwork::new(nodes)
    }

    fn svc(s: &str) -> ServiceName {
        ServiceName::new(s)
    }

    // ── registration / discovery ──────────────────────────────────────────────

    #[test]
    fn test_register_and_find_service() {
        let mut net = make_network();
        net.register(
            NodeId(1),
            svc("calc"),
            vec!["add".to_owned(), "mul".to_owned()],
        );
        let found = net.find_service(&svc("calc"));
        assert_eq!(found, vec![NodeId(1)]);
    }

    #[test]
    fn test_find_service_multiple_nodes() {
        let mut net = make_network();
        net.register(NodeId(1), svc("auth"), vec!["login".to_owned()]);
        net.register(NodeId(2), svc("auth"), vec!["login".to_owned()]);
        let mut found = net.find_service(&svc("auth"));
        found.sort();
        assert_eq!(found, vec![NodeId(1), NodeId(2)]);
    }

    #[test]
    fn test_find_service_excludes_downed_nodes() {
        let mut net = make_network();
        net.register(NodeId(1), svc("storage"), vec!["read".to_owned()]);
        net.register(NodeId(2), svc("storage"), vec!["read".to_owned()]);
        net.take_down(NodeId(1));
        let found = net.find_service(&svc("storage"));
        assert_eq!(found, vec![NodeId(2)]);
    }

    #[test]
    fn test_find_service_unknown_returns_empty() {
        let net = make_network();
        assert!(net.find_service(&svc("unknown")).is_empty());
    }

    // ── call mechanics ────────────────────────────────────────────────────────

    #[test]
    fn test_call_success() {
        let mut net = make_network();
        net.register(NodeId(2), svc("echo"), vec!["say".to_owned()]);
        let req = RpcRequest {
            id: 0,
            from: NodeId(1),
            to: NodeId(2),
            service: svc("echo"),
            method: "say".to_owned(),
            args: vec!["hello".to_owned()],
        };
        let resp = net.call(req);
        assert_eq!(resp.result, RpcResult::Ok("hello".to_owned()));
    }

    #[test]
    fn test_call_node_down() {
        let mut net = make_network();
        net.register(NodeId(2), svc("echo"), vec!["say".to_owned()]);
        net.take_down(NodeId(2));
        let req = RpcRequest {
            id: 1,
            from: NodeId(1),
            to: NodeId(2),
            service: svc("echo"),
            method: "say".to_owned(),
            args: vec![],
        };
        let resp = net.call(req);
        assert_eq!(resp.result, RpcResult::NodeDown);
    }

    #[test]
    fn test_call_unknown_method_returns_err() {
        let mut net = make_network();
        net.register(NodeId(2), svc("calc"), vec!["add".to_owned()]);
        let req = RpcRequest {
            id: 2,
            from: NodeId(1),
            to: NodeId(2),
            service: svc("calc"),
            method: "div".to_owned(),
            args: vec![],
        };
        let resp = net.call(req);
        assert!(matches!(resp.result, RpcResult::Err(_)));
    }

    #[test]
    fn test_call_logs_message() {
        let mut net = make_network();
        net.register(NodeId(2), svc("log"), vec!["write".to_owned()]);
        let req = RpcRequest {
            id: 0,
            from: NodeId(1),
            to: NodeId(2),
            service: svc("log"),
            method: "write".to_owned(),
            args: vec!["entry".to_owned()],
        };
        net.call(req);
        assert_eq!(net.message_log.len(), 1);
    }

    #[test]
    fn test_call_args_echo() {
        let mut net = make_network();
        net.register(NodeId(2), svc("join"), vec!["concat".to_owned()]);
        let req = RpcRequest {
            id: 0,
            from: NodeId(1),
            to: NodeId(2),
            service: svc("join"),
            method: "concat".to_owned(),
            args: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
        };
        let resp = net.call(req);
        assert_eq!(resp.result, RpcResult::Ok("a,b,c".to_owned()));
    }

    // ── latency ───────────────────────────────────────────────────────────────

    #[test]
    fn test_latency_advances_clock() {
        let mut net = make_network();
        net.register(NodeId(2), svc("ping"), vec!["pong".to_owned()]);
        net.set_latency(NodeId(1), NodeId(2), 50);
        let req = RpcRequest {
            id: 0,
            from: NodeId(1),
            to: NodeId(2),
            service: svc("ping"),
            method: "pong".to_owned(),
            args: vec![],
        };
        net.call(req);
        assert_eq!(net.clock(), 50);
    }

    #[test]
    fn test_latency_accumulates_across_calls() {
        let mut net = make_network();
        net.register(NodeId(2), svc("ping"), vec!["pong".to_owned()]);
        net.set_latency(NodeId(1), NodeId(2), 30);
        for _ in 0..3 {
            let req = RpcRequest {
                id: 0,
                from: NodeId(1),
                to: NodeId(2),
                service: svc("ping"),
                method: "pong".to_owned(),
                args: vec![],
            };
            net.call(req);
        }
        assert_eq!(net.clock(), 90);
    }

    // ── take_down / bring_up ──────────────────────────────────────────────────

    #[test]
    fn test_bring_up_restores_node() {
        let mut net = make_network();
        net.register(NodeId(2), svc("kv"), vec!["get".to_owned()]);
        net.take_down(NodeId(2));
        net.bring_up(NodeId(2));
        let req = RpcRequest {
            id: 0,
            from: NodeId(1),
            to: NodeId(2),
            service: svc("kv"),
            method: "get".to_owned(),
            args: vec!["key".to_owned()],
        };
        let resp = net.call(req);
        assert_eq!(resp.result, RpcResult::Ok("key".to_owned()));
    }

    #[test]
    fn test_take_down_then_find_service_empty() {
        let mut net = make_network();
        net.register(NodeId(1), svc("q"), vec!["push".to_owned()]);
        net.take_down(NodeId(1));
        assert!(net.find_service(&svc("q")).is_empty());
    }

    // ── make_call helper ──────────────────────────────────────────────────────

    #[test]
    fn test_make_call_increments_request_id() {
        let mut net = make_network();
        net.register(NodeId(2), svc("s"), vec!["m".to_owned()]);
        let r1 = net.make_call(NodeId(1), NodeId(2), svc("s"), "m", vec![]);
        let r2 = net.make_call(NodeId(1), NodeId(2), svc("s"), "m", vec![]);
        assert_ne!(r1.request_id, r2.request_id);
    }

    // ── CircuitBreaker ────────────────────────────────────────────────────────

    #[test]
    fn test_circuit_breaker_starts_closed() {
        let cb = CircuitBreaker::new(3, 100);
        assert_eq!(cb.state(), &CircuitState::Closed);
    }

    #[test]
    fn test_circuit_breaker_trips_after_threshold() {
        let mut cb = CircuitBreaker::new(3, 100);
        cb.attempt(false, 0);
        cb.attempt(false, 1);
        cb.attempt(false, 2);
        assert_eq!(cb.state(), &CircuitState::Open);
    }

    #[test]
    fn test_circuit_breaker_resets_on_success() {
        let mut cb = CircuitBreaker::new(3, 100);
        cb.attempt(false, 0);
        cb.attempt(false, 1);
        cb.attempt(true, 2);
        assert_eq!(cb.state(), &CircuitState::Closed);
        assert_eq!(cb.failures, 0);
    }

    #[test]
    fn test_circuit_breaker_open_rejects() {
        let mut cb = CircuitBreaker::new(2, 1000);
        cb.attempt(false, 0);
        cb.attempt(false, 1);
        // Should now be Open; immediate retry should be rejected (clock not advanced)
        let allow = cb.attempt(true, 2);
        // open_duration=1000, current_time=2: 2 < 0+1000 → still open → false
        assert!(!allow);
    }

    #[test]
    fn test_circuit_breaker_half_open_success_closes() {
        let mut cb = CircuitBreaker::new(2, 10);
        cb.attempt(false, 0);
        cb.attempt(false, 1);
        // Open. After open_duration=10 the breaker moves to HalfOpen then probes.
        let allowed = cb.attempt(true, 20);
        assert!(allowed);
        assert_eq!(cb.state(), &CircuitState::Closed);
    }

    #[test]
    fn test_circuit_breaker_half_open_failure_reopens() {
        let mut cb = CircuitBreaker::new(2, 10);
        cb.attempt(false, 0);
        cb.attempt(false, 1);
        cb.attempt(false, 20); // probe fails → back to Open
        assert_eq!(cb.state(), &CircuitState::Open);
    }

    // ── retry_with_backoff ────────────────────────────────────────────────────

    #[test]
    fn test_retry_with_backoff_empty() {
        let delays = retry_with_backoff(0, 100);
        assert!(delays.is_empty());
    }

    #[test]
    fn test_retry_with_backoff_sequence() {
        let delays = retry_with_backoff(4, 10);
        assert_eq!(delays, vec![10, 20, 40, 80]);
    }

    #[test]
    fn test_retry_with_backoff_single() {
        let delays = retry_with_backoff(1, 500);
        assert_eq!(delays, vec![500]);
    }

    #[test]
    fn test_retry_with_backoff_saturates() {
        // With a large base and many attempts this must not panic/overflow
        let delays = retry_with_backoff(64, u64::MAX / 2);
        assert!(!delays.is_empty());
        // All values are valid u64 (saturation prevents overflow)
        assert!(delays.iter().all(|d| *d > 0));
    }

    #[test]
    fn test_retry_with_backoff_base_zero() {
        let delays = retry_with_backoff(5, 0);
        assert_eq!(delays, vec![0, 0, 0, 0, 0]);
    }
}
