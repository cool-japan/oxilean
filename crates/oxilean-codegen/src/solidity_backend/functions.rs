//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    ContractKind, SolAnalysisCache, SolConstantFoldingHelper, SolDepGraph, SolDominatorTree,
    SolExtCache, SolExtConstFolder, SolExtDepGraph, SolExtDomTree, SolExtLiveness,
    SolExtPassConfig, SolExtPassPhase, SolExtPassRegistry, SolExtPassStats, SolExtWorklist,
    SolLivenessInfo, SolPassConfig, SolPassPhase, SolPassRegistry, SolPassStats, SolWorklist,
    SolidityBackend, SolidityContract, SolidityEnum, SolidityError, SolidityEvent, SolidityExpr,
    SolidityFunction, SolidityParam, SolidityStateVar, SolidityStmt, SolidityStruct, SolidityType,
    StateMutability, Visibility,
};

/// Standard library constants available in every generated Solidity contract.
/// These are injected as comments / utility fragments for the emitter.
pub const SOLIDITY_RUNTIME: &str = r#"// SPDX-License-Identifier: MIT
// OxiLean Solidity Runtime Library (inlined)

/// @dev SafeMath-equivalent operations (Solidity 0.8+ has overflow checks built in)
library OxiLeanMath {
    /// @notice Returns the minimum of two values.
    function min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }

    /// @notice Returns the maximum of two values.
    function max(uint256 a, uint256 b) internal pure returns (uint256) {
        return a > b ? a : b;
    }

    /// @notice Returns the absolute difference.
    function absDiff(uint256 a, uint256 b) internal pure returns (uint256) {
        return a >= b ? a - b : b - a;
    }

    /// @notice Saturating addition (clamps to uint256 max).
    function saturatingAdd(uint256 a, uint256 b) internal pure returns (uint256) {
        unchecked {
            uint256 c = a + b;
            return c < a ? type(uint256).max : c;
        }
    }

    /// @notice Saturating subtraction (clamps to 0).
    function saturatingSub(uint256 a, uint256 b) internal pure returns (uint256) {
        return a > b ? a - b : 0;
    }

    /// @notice Integer square root (floor).
    function sqrt(uint256 x) internal pure returns (uint256 r) {
        if (x == 0) return 0;
        r = 1;
        uint256 xAux = x;
        if (xAux >= 0x100000000000000000000000000000000) { r <<= 64; xAux >>= 128; }
        if (xAux >= 0x10000000000000000) { r <<= 32; xAux >>= 64; }
        if (xAux >= 0x100000000) { r <<= 16; xAux >>= 32; }
        if (xAux >= 0x10000) { r <<= 8; xAux >>= 16; }
        if (xAux >= 0x100) { r <<= 4; xAux >>= 8; }
        if (xAux >= 0x10) { r <<= 2; xAux >>= 4; }
        if (xAux >= 0x4) { r <<= 1; }
        r = (r + x / r) >> 1;
        r = (r + x / r) >> 1;
        r = (r + x / r) >> 1;
        r = (r + x / r) >> 1;
        r = (r + x / r) >> 1;
        r = (r + x / r) >> 1;
        r = (r + x / r) >> 1;
        return r > x / r ? r - 1 : r;
    }
}

/// @dev Address utilities
library OxiLeanAddress {
    /// @notice Returns true if the address is the zero address.
    function isZero(address addr) internal pure returns (bool) {
        return addr == address(0);
    }

    /// @notice Converts an address to uint256.
    function toUint256(address addr) internal pure returns (uint256) {
        return uint256(uint160(addr));
    }

    /// @notice Converts uint256 to address.
    function fromUint256(uint256 n) internal pure returns (address) {
        return address(uint160(n));
    }
}

/// @dev Bytes utilities
library OxiLeanBytes {
    /// @notice Converts bytes32 to bytes.
    function toBytes(bytes32 b) internal pure returns (bytes memory) {
        bytes memory result = new bytes(32);
        assembly { mstore(add(result, 32), b) }
        return result;
    }

    /// @notice Concatenates two byte arrays.
    function concat(bytes memory a, bytes memory b) internal pure returns (bytes memory c) {
        uint256 la = a.length;
        uint256 lb = b.length;
        c = new bytes(la + lb);
        for (uint256 i = 0; i < la; i++) c[i] = a[i];
        for (uint256 j = 0; j < lb; j++) c[la + j] = b[j];
    }
}
"#;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_uint256_display() {
        assert_eq!(SolidityType::Uint256.to_string(), "uint256");
    }
    #[test]
    pub(super) fn test_mapping_display() {
        let ty = SolidityType::Mapping(
            Box::new(SolidityType::Address),
            Box::new(SolidityType::Uint256),
        );
        assert_eq!(ty.to_string(), "mapping(address => uint256)");
    }
    #[test]
    pub(super) fn test_dyn_array_display() {
        let ty = SolidityType::DynArray(Box::new(SolidityType::Uint256));
        assert_eq!(ty.to_string(), "uint256[]");
    }
    #[test]
    pub(super) fn test_fixed_array_display() {
        let ty = SolidityType::FixedArray(Box::new(SolidityType::Bool), 10);
        assert_eq!(ty.to_string(), "bool[10]");
    }
    #[test]
    pub(super) fn test_string_type_display() {
        assert_eq!(SolidityType::StringTy.to_string(), "string");
    }
    #[test]
    pub(super) fn test_tuple_abi_canonical() {
        let ty = SolidityType::Tuple(vec![
            SolidityType::Uint256,
            SolidityType::Address,
            SolidityType::Bool,
        ]);
        assert_eq!(ty.abi_canonical(), "(uint256,address,bool)");
    }
    #[test]
    pub(super) fn test_is_reference_type_string() {
        assert!(SolidityType::StringTy.is_reference_type());
    }
    #[test]
    pub(super) fn test_is_reference_type_uint256() {
        assert!(!SolidityType::Uint256.is_reference_type());
    }
    #[test]
    pub(super) fn test_is_reference_type_mapping() {
        let ty = SolidityType::Mapping(
            Box::new(SolidityType::Address),
            Box::new(SolidityType::Uint256),
        );
        assert!(ty.is_reference_type());
    }
    #[test]
    pub(super) fn test_param_new_value_type() {
        let p = SolidityParam::new(SolidityType::Uint256, "amount");
        assert!(p.location.is_none());
        assert_eq!(p.to_string(), "uint256 amount");
    }
    #[test]
    pub(super) fn test_param_new_reference_type() {
        let p = SolidityParam::new(SolidityType::StringTy, "name");
        assert_eq!(p.location.as_deref(), Some("memory"));
        assert_eq!(p.to_string(), "string memory name");
    }
    #[test]
    pub(super) fn test_param_calldata() {
        let p = SolidityParam::calldata(SolidityType::Bytes, "data");
        assert_eq!(p.location.as_deref(), Some("calldata"));
        assert_eq!(p.to_string(), "bytes calldata data");
    }
    #[test]
    pub(super) fn test_abi_signature() {
        let mut func = SolidityFunction::new("transfer");
        func.params
            .push(SolidityParam::new(SolidityType::Address, "to"));
        func.params
            .push(SolidityParam::new(SolidityType::Uint256, "amount"));
        assert_eq!(func.abi_signature(), "transfer(address,uint256)");
    }
    #[test]
    pub(super) fn test_selector_length() {
        let func = SolidityFunction::new("balanceOf");
        let sel = func.selector();
        assert_eq!(sel.len(), 4);
    }
    #[test]
    pub(super) fn test_selector_deterministic() {
        let f1 = SolidityFunction::new("approve");
        let f2 = SolidityFunction::new("approve");
        assert_eq!(f1.selector(), f2.selector());
    }
    #[test]
    pub(super) fn test_selector_canonical_ethereum_values() {
        // transfer(address,uint256) → 0xa9059cbb
        let mut transfer = SolidityFunction::new("transfer");
        transfer
            .params
            .push(SolidityParam::new(SolidityType::Address, "to"));
        transfer
            .params
            .push(SolidityParam::new(SolidityType::Uint256, "amount"));
        assert_eq!(
            transfer.selector(),
            [0xa9, 0x05, 0x9c, 0xbb],
            "transfer(address,uint256) selector mismatch"
        );

        // balanceOf(address) → 0x70a08231
        let mut balance_of = SolidityFunction::new("balanceOf");
        balance_of
            .params
            .push(SolidityParam::new(SolidityType::Address, "account"));
        assert_eq!(
            balance_of.selector(),
            [0x70, 0xa0, 0x82, 0x31],
            "balanceOf(address) selector mismatch"
        );

        // approve(address,uint256) → 0x095ea7b3
        let mut approve = SolidityFunction::new("approve");
        approve
            .params
            .push(SolidityParam::new(SolidityType::Address, "spender"));
        approve
            .params
            .push(SolidityParam::new(SolidityType::Uint256, "amount"));
        assert_eq!(
            approve.selector(),
            [0x09, 0x5e, 0xa7, 0xb3],
            "approve(address,uint256) selector mismatch"
        );

        // allowance(address,address) → 0xdd62ed3e
        let mut allowance = SolidityFunction::new("allowance");
        allowance
            .params
            .push(SolidityParam::new(SolidityType::Address, "owner"));
        allowance
            .params
            .push(SolidityParam::new(SolidityType::Address, "spender"));
        assert_eq!(
            allowance.selector(),
            [0xdd, 0x62, 0xed, 0x3e],
            "allowance(address,address) selector mismatch"
        );

        // totalSupply() → 0x18160ddd
        let total_supply = SolidityFunction::new("totalSupply");
        assert_eq!(
            total_supply.selector(),
            [0x18, 0x16, 0x0d, 0xdd],
            "totalSupply() selector mismatch"
        );

        // Additional well-known selector: decimals() → 0x313ce567
        let decimals = SolidityFunction::new("decimals");
        assert_eq!(
            decimals.selector(),
            [0x31, 0x3c, 0xe5, 0x67],
            "decimals() selector mismatch"
        );
    }
    #[test]
    pub(super) fn test_msg_sender_display() {
        assert_eq!(SolidityExpr::MsgSender.to_string(), "msg.sender");
    }
    #[test]
    pub(super) fn test_binop_display() {
        let expr = SolidityExpr::BinOp(
            "+".into(),
            Box::new(SolidityExpr::Var("a".into())),
            Box::new(SolidityExpr::IntLit(1)),
        );
        assert_eq!(expr.to_string(), "(a + 1)");
    }
    #[test]
    pub(super) fn test_ternary_display() {
        let expr = SolidityExpr::Ternary(
            Box::new(SolidityExpr::BoolLit(true)),
            Box::new(SolidityExpr::IntLit(1)),
            Box::new(SolidityExpr::IntLit(0)),
        );
        assert_eq!(expr.to_string(), "(true ? 1 : 0)");
    }
    #[test]
    pub(super) fn test_keccak256_display() {
        let expr = SolidityExpr::Keccak256(Box::new(SolidityExpr::Var("data".into())));
        assert_eq!(expr.to_string(), "keccak256(data)");
    }
    #[test]
    pub(super) fn test_type_max_display() {
        let expr = SolidityExpr::TypeMax(SolidityType::Uint256);
        assert_eq!(expr.to_string(), "type(uint256).max");
    }
    #[test]
    pub(super) fn test_emit_empty_contract() {
        let mut backend = SolidityBackend::new();
        backend.add_contract(SolidityContract::new("Empty", ContractKind::Contract));
        let src = backend.emit_contract();
        assert!(src.contains("pragma solidity"));
        assert!(src.contains("contract Empty {"));
        assert!(src.contains("SPDX-License-Identifier: MIT"));
    }
    #[test]
    pub(super) fn test_emit_interface() {
        let mut backend = SolidityBackend::new();
        let mut iface = SolidityContract::new("IERC20", ContractKind::Interface);
        let mut func = SolidityFunction::new("totalSupply");
        func.returns
            .push(SolidityParam::new(SolidityType::Uint256, ""));
        func.mutability = StateMutability::View;
        func.body = vec![];
        iface.functions.push(func);
        backend.add_contract(iface);
        let src = backend.emit_contract();
        assert!(src.contains("interface IERC20 {"));
        assert!(src.contains("function totalSupply()"));
    }
    #[test]
    pub(super) fn test_emit_contract_with_state_var() {
        let mut backend = SolidityBackend::new();
        let mut contract = SolidityContract::new("Token", ContractKind::Contract);
        contract.state_vars.push(SolidityStateVar {
            ty: SolidityType::Mapping(
                Box::new(SolidityType::Address),
                Box::new(SolidityType::Uint256),
            ),
            name: "_balances".into(),
            visibility: Visibility::Private,
            is_immutable: false,
            is_constant: false,
            init: None,
            doc: None,
        });
        backend.add_contract(contract);
        let src = backend.emit_contract();
        assert!(src.contains("mapping(address => uint256)"));
        assert!(src.contains("_balances"));
    }
    #[test]
    pub(super) fn test_emit_event() {
        let mut backend = SolidityBackend::new();
        let mut contract = SolidityContract::new("Token", ContractKind::Contract);
        contract.events.push(SolidityEvent {
            name: "Transfer".into(),
            fields: vec![
                (SolidityType::Address, true, "from".into()),
                (SolidityType::Address, true, "to".into()),
                (SolidityType::Uint256, false, "value".into()),
            ],
            anonymous: false,
            doc: None,
        });
        backend.add_contract(contract);
        let src = backend.emit_contract();
        assert!(src.contains("event Transfer("));
        assert!(src.contains("indexed from"));
        assert!(src.contains("indexed to"));
    }
    #[test]
    pub(super) fn test_emit_custom_error() {
        let mut backend = SolidityBackend::new();
        let mut contract = SolidityContract::new("Token", ContractKind::Contract);
        contract.errors.push(SolidityError {
            name: "InsufficientBalance".into(),
            params: vec![
                SolidityParam::new(SolidityType::Uint256, "available"),
                SolidityParam::new(SolidityType::Uint256, "required"),
            ],
            doc: None,
        });
        backend.add_contract(contract);
        let src = backend.emit_contract();
        assert!(src.contains("error InsufficientBalance("));
    }
    #[test]
    pub(super) fn test_emit_require_stmt() {
        let stmt = SolidityStmt::Require(
            SolidityExpr::BinOp(
                ">".into(),
                Box::new(SolidityExpr::Var("amount".into())),
                Box::new(SolidityExpr::IntLit(0)),
            ),
            Some("Amount must be positive".into()),
        );
        let out = SolidityBackend::emit_stmt(&stmt, 2);
        assert!(out.contains("require("));
        assert!(out.contains("Amount must be positive"));
    }
    #[test]
    pub(super) fn test_compile_decl() {
        let mut backend = SolidityBackend::new();
        let sv = backend.compile_decl("owner", SolidityType::Address);
        assert_eq!(sv.name, "owner");
        assert!(matches!(sv.ty, SolidityType::Address));
    }
    #[test]
    pub(super) fn test_runtime_constant_not_empty() {
        assert!(!SOLIDITY_RUNTIME.is_empty());
        assert!(SOLIDITY_RUNTIME.contains("OxiLeanMath"));
    }
    #[test]
    pub(super) fn test_visibility_display() {
        assert_eq!(Visibility::Public.to_string(), "public");
        assert_eq!(Visibility::External.to_string(), "external");
        assert_eq!(Visibility::Private.to_string(), "private");
        assert_eq!(Visibility::Internal.to_string(), "internal");
    }
    #[test]
    pub(super) fn test_state_mutability_display() {
        assert_eq!(StateMutability::View.to_string(), "view");
        assert_eq!(StateMutability::Pure.to_string(), "pure");
        assert_eq!(StateMutability::Payable.to_string(), "payable");
        assert_eq!(StateMutability::NonPayable.to_string(), "");
    }
    #[test]
    pub(super) fn test_emit_struct() {
        let s = SolidityStruct {
            name: "Position".into(),
            fields: vec![
                (SolidityType::Uint256, "x".into()),
                (SolidityType::Uint256, "y".into()),
            ],
            doc: None,
        };
        let out = SolidityBackend::emit_struct(&s, 1);
        assert!(out.contains("struct Position {"));
        assert!(out.contains("uint256 x;"));
        assert!(out.contains("uint256 y;"));
    }
    #[test]
    pub(super) fn test_emit_enum() {
        let e = SolidityEnum {
            name: "State".into(),
            variants: vec!["Created".into(), "Active".into(), "Closed".into()],
            doc: None,
        };
        let out = SolidityBackend::emit_enum(&e, 1);
        assert!(out.contains("enum State {"));
        assert!(out.contains("Created"));
        assert!(out.contains("Closed"));
    }
    #[test]
    pub(super) fn test_emit_with_runtime() {
        let mut backend = SolidityBackend::new().with_runtime();
        backend.add_contract(SolidityContract::new("X", ContractKind::Contract));
        let src = backend.emit_contract();
        assert!(src.contains("OxiLeanMath"));
        assert!(src.contains("saturatingAdd"));
    }
    #[test]
    pub(super) fn test_emit_inheritance() {
        let mut backend = SolidityBackend::new();
        let mut contract = SolidityContract::new("MyToken", ContractKind::Contract);
        contract.bases.push("ERC20".into());
        contract.bases.push("Ownable".into());
        backend.add_contract(contract);
        let src = backend.emit_contract();
        assert!(src.contains("contract MyToken is ERC20, Ownable {"));
    }
    #[test]
    pub(super) fn test_address_payable_display() {
        assert_eq!(SolidityType::AddressPayable.to_string(), "address payable");
    }
    #[test]
    pub(super) fn test_payable_expr_display() {
        let expr = SolidityExpr::Payable(Box::new(SolidityExpr::MsgSender));
        assert_eq!(expr.to_string(), "payable(msg.sender)");
    }
}
#[cfg(test)]
mod Sol_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = SolPassConfig::new("test_pass", SolPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = SolPassStats::new();
        stats.record_run(10, 100, 3);
        stats.record_run(20, 200, 5);
        assert_eq!(stats.total_runs, 2);
        assert!((stats.average_changes_per_run() - 15.0).abs() < 0.01);
        assert!((stats.success_rate() - 1.0).abs() < 0.01);
        let s = stats.format_summary();
        assert!(s.contains("Runs: 2/2"));
    }
    #[test]
    pub(super) fn test_pass_registry() {
        let mut reg = SolPassRegistry::new();
        reg.register(SolPassConfig::new("pass_a", SolPassPhase::Analysis));
        reg.register(SolPassConfig::new("pass_b", SolPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = SolAnalysisCache::new(10);
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
        cache.invalidate("key1");
        assert!(!cache.entries["key1"].valid);
        assert_eq!(cache.size(), 1);
    }
    #[test]
    pub(super) fn test_worklist() {
        let mut wl = SolWorklist::new();
        assert!(wl.push(1));
        assert!(wl.push(2));
        assert!(!wl.push(1));
        assert_eq!(wl.len(), 2);
        assert_eq!(wl.pop(), Some(1));
        assert!(!wl.contains(1));
        assert!(wl.contains(2));
    }
    #[test]
    pub(super) fn test_dominator_tree() {
        let mut dt = SolDominatorTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 3));
        assert!(!dt.dominates(2, 3));
        assert!(dt.dominates(3, 3));
    }
    #[test]
    pub(super) fn test_liveness() {
        let mut liveness = SolLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(SolConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(SolConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(SolConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            SolConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(SolConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = SolDepGraph::new();
        g.add_dep(1, 2);
        g.add_dep(2, 3);
        g.add_dep(1, 3);
        assert_eq!(g.dependencies_of(2), vec![1]);
        let topo = g.topological_sort();
        assert_eq!(topo.len(), 3);
        assert!(!g.has_cycle());
        let pos: std::collections::HashMap<u32, usize> =
            topo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos[&1] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }
}
#[cfg(test)]
mod solext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_solext_phase_order() {
        assert_eq!(SolExtPassPhase::Early.order(), 0);
        assert_eq!(SolExtPassPhase::Middle.order(), 1);
        assert_eq!(SolExtPassPhase::Late.order(), 2);
        assert_eq!(SolExtPassPhase::Finalize.order(), 3);
        assert!(SolExtPassPhase::Early.is_early());
        assert!(!SolExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_solext_config_builder() {
        let c = SolExtPassConfig::new("p")
            .with_phase(SolExtPassPhase::Late)
            .with_max_iter(50)
            .with_debug(1);
        assert_eq!(c.name, "p");
        assert_eq!(c.max_iterations, 50);
        assert!(c.is_debug_enabled());
        assert!(c.enabled);
        let c2 = c.disabled();
        assert!(!c2.enabled);
    }
    #[test]
    pub(super) fn test_solext_stats() {
        let mut s = SolExtPassStats::new();
        s.visit();
        s.visit();
        s.modify();
        s.iterate();
        assert_eq!(s.nodes_visited, 2);
        assert_eq!(s.nodes_modified, 1);
        assert!(s.changed);
        assert_eq!(s.iterations, 1);
        let e = s.efficiency();
        assert!((e - 0.5).abs() < 1e-9);
    }
    #[test]
    pub(super) fn test_solext_registry() {
        let mut r = SolExtPassRegistry::new();
        r.register(SolExtPassConfig::new("a").with_phase(SolExtPassPhase::Early));
        r.register(SolExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&SolExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_solext_cache() {
        let mut c = SolExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_solext_worklist() {
        let mut w = SolExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_solext_dom_tree() {
        let mut dt = SolExtDomTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        dt.set_idom(4, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 4));
        assert!(!dt.dominates(2, 3));
        assert_eq!(dt.depth_of(3), 2);
    }
    #[test]
    pub(super) fn test_solext_liveness() {
        let mut lv = SolExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_solext_const_folder() {
        let mut cf = SolExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_solext_dep_graph() {
        let mut g = SolExtDepGraph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(!g.has_cycle());
        assert_eq!(g.topo_sort(), Some(vec![0, 1, 2, 3]));
        assert_eq!(g.reachable(0).len(), 4);
        let sccs = g.scc();
        assert_eq!(sccs.len(), 4);
    }
}
