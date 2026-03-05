//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    VypAnalysisCache, VypConstantFoldingHelper, VypDepGraph, VypDominatorTree, VypLivenessInfo,
    VypPassConfig, VypPassPhase, VypPassRegistry, VypPassStats, VypWorklist, VyperBackend,
    VyperConstant, VyperContract, VyperDecorator, VyperEvent, VyperExpr, VyperExtCache,
    VyperExtConstFolder, VyperExtDepGraph, VyperExtDomTree, VyperExtLiveness, VyperExtPassConfig,
    VyperExtPassPhase, VyperExtPassRegistry, VyperExtPassStats, VyperExtWorklist, VyperFlagDef,
    VyperFunction, VyperInterface, VyperParam, VyperStmt, VyperStorageVar, VyperStruct, VyperType,
};

/// Standard runtime helpers emitted as Vyper module-level utilities.
pub const VYPER_RUNTIME: &str = r#"# OxiLean Vyper Runtime Helpers
# These @external functions expose common mathematical utilities.

@external
@view
def oxilean_min(a: uint256, b: uint256) -> uint256:
    """
    @dev Returns the minimum of two uint256 values.
    @param a First value.
    @param b Second value.
    @return Minimum of a and b.
    """
    if a < b:
        return a
    return b

@external
@view
def oxilean_max(a: uint256, b: uint256) -> uint256:
    """
    @dev Returns the maximum of two uint256 values.
    @param a First value.
    @param b Second value.
    @return Maximum of a and b.
    """
    if a > b:
        return a
    return b

@external
@pure
def oxilean_abs_diff(a: uint256, b: uint256) -> uint256:
    """
    @dev Returns |a - b|.
    """
    if a >= b:
        return a - b
    return b - a

@external
@pure
def oxilean_saturating_add(a: uint256, b: uint256) -> uint256:
    """
    @dev Saturating addition — returns max_value(uint256) on overflow.
    """
    result: uint256 = a + b
    if result < a:
        return max_value(uint256)
    return result

@external
@pure
def oxilean_saturating_sub(a: uint256, b: uint256) -> uint256:
    """
    @dev Saturating subtraction — clamps to 0 on underflow.
    """
    if a > b:
        return a - b
    return 0

@external
@pure
def oxilean_clamp(value: uint256, lo: uint256, hi: uint256) -> uint256:
    """
    @dev Clamps value to [lo, hi].
    """
    if value < lo:
        return lo
    if value > hi:
        return hi
    return value

@external
@pure
def oxilean_is_pow2(n: uint256) -> bool:
    """
    @dev Returns True iff n is a power of two (n > 0).
    """
    if n == 0:
        return False
    return (n & (n - 1)) == 0

@external
@pure
def oxilean_log2_floor(n: uint256) -> uint256:
    """
    @dev Floor log base 2 of n (n must be > 0).
    """
    assert n > 0, "log2 of zero"
    result: uint256 = 0
    x: uint256 = n
    for _: uint256 in range(256):
        if x <= 1:
            break
        x = x >> 1
        result += 1
    return result

@external
@pure
def oxilean_count_ones(n: uint256) -> uint256:
    """
    @dev Population count — number of set bits in n.
    """
    count: uint256 = 0
    x: uint256 = n
    for _: uint256 in range(256):
        if x == 0:
            break
        count += x & 1
        x = x >> 1
    return count

@external
@view
def oxilean_addr_to_uint(a: address) -> uint256:
    """
    @dev Converts an address to uint256.
    """
    return convert(a, uint256)

@external
@pure
def oxilean_bytes32_to_bool(b: bytes32) -> bool:
    """
    @dev Returns True if any byte in b is non-zero.
    """
    return b != empty(bytes32)

@external
@view
def oxilean_self_balance() -> uint256:
    """
    @dev Returns the current contract ETH balance.
    """
    return self.balance

@external
@view
def oxilean_code_size(a: address) -> uint256:
    """
    @dev Returns the code size of an address (0 for EOA).
    """
    return a.codesize

@external
@pure
def oxilean_pack_two_uint128(hi: uint128, lo: uint128) -> uint256:
    """
    @dev Packs two uint128 values into a single uint256.
    """
    return (convert(hi, uint256) << 128) | convert(lo, uint256)

@external
@pure
def oxilean_unpack_hi(packed: uint256) -> uint128:
    """
    @dev Extracts the high 128 bits from a packed uint256.
    """
    return convert(packed >> 128, uint128)

@external
@pure
def oxilean_unpack_lo(packed: uint256) -> uint128:
    """
    @dev Extracts the low 128 bits from a packed uint256.
    """
    return convert(packed & convert(max_value(uint128), uint256), uint128)
"#;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub(super) fn test_uint256_display() {
        assert_eq!(VyperType::Uint256.to_string(), "uint256");
    }
    #[test]
    pub(super) fn test_hashmap_display() {
        let ty = VyperType::HashMap(Box::new(VyperType::Address), Box::new(VyperType::Uint256));
        assert_eq!(ty.to_string(), "HashMap[address, uint256]");
    }
    #[test]
    pub(super) fn test_dyn_array_display() {
        let ty = VyperType::DynArray(Box::new(VyperType::Uint256), 128);
        assert_eq!(ty.to_string(), "DynArray[uint256, 128]");
    }
    #[test]
    pub(super) fn test_fixed_array_display() {
        let ty = VyperType::FixedArray(Box::new(VyperType::Bool), 10);
        assert_eq!(ty.to_string(), "bool[10]");
    }
    #[test]
    pub(super) fn test_bytes_bounded_display() {
        assert_eq!(VyperType::Bytes(32).to_string(), "Bytes[32]");
    }
    #[test]
    pub(super) fn test_string_bounded_display() {
        assert_eq!(VyperType::StringTy(100).to_string(), "String[100]");
    }
    #[test]
    pub(super) fn test_decimal_display() {
        assert_eq!(VyperType::Decimal.to_string(), "decimal");
    }
    #[test]
    pub(super) fn test_abi_canonical_dyn_array() {
        let ty = VyperType::DynArray(Box::new(VyperType::Address), 10);
        assert_eq!(ty.abi_canonical(), "address[]");
    }
    #[test]
    pub(super) fn test_abi_canonical_string() {
        assert_eq!(VyperType::StringTy(50).abi_canonical(), "string");
    }
    #[test]
    pub(super) fn test_external_decorator() {
        assert_eq!(VyperDecorator::External.to_string(), "@external");
    }
    #[test]
    pub(super) fn test_nonreentrant_decorator() {
        let d = VyperDecorator::NonReentrant("lock".into());
        assert_eq!(d.to_string(), "@nonreentrant(\"lock\")");
    }
    #[test]
    pub(super) fn test_view_decorator() {
        assert_eq!(VyperDecorator::View.to_string(), "@view");
    }
    #[test]
    pub(super) fn test_param_display_no_default() {
        let p = VyperParam::new("amount", VyperType::Uint256);
        assert_eq!(p.to_string(), "amount: uint256");
    }
    #[test]
    pub(super) fn test_param_display_with_default() {
        let p = VyperParam::new("decimals", VyperType::Uint8).with_default(VyperExpr::IntLit(18));
        assert_eq!(p.to_string(), "decimals: uint8 = 18");
    }
    #[test]
    pub(super) fn test_abi_signature() {
        let mut func = VyperFunction::new("transfer");
        func.params.push(VyperParam::new("to", VyperType::Address));
        func.params
            .push(VyperParam::new("amount", VyperType::Uint256));
        assert_eq!(func.abi_signature(), "transfer(address,uint256)");
    }
    #[test]
    pub(super) fn test_selector_length() {
        let func = VyperFunction::new("balanceOf");
        assert_eq!(func.selector().len(), 4);
    }
    #[test]
    pub(super) fn test_selector_deterministic() {
        let f1 = VyperFunction::new("approve");
        let f2 = VyperFunction::new("approve");
        assert_eq!(f1.selector(), f2.selector());
    }
    #[test]
    pub(super) fn test_is_external() {
        let func = VyperFunction::new("foo").external();
        assert!(func.is_external());
    }
    #[test]
    pub(super) fn test_is_read_only() {
        let func = VyperFunction::new("foo").view();
        assert!(func.is_read_only());
        let func2 = VyperFunction::new("bar").pure_fn();
        assert!(func2.is_read_only());
    }
    #[test]
    pub(super) fn test_bool_lit_true() {
        assert_eq!(VyperExpr::BoolLit(true).to_string(), "True");
    }
    #[test]
    pub(super) fn test_bool_lit_false() {
        assert_eq!(VyperExpr::BoolLit(false).to_string(), "False");
    }
    #[test]
    pub(super) fn test_self_field() {
        assert_eq!(
            VyperExpr::SelfField("owner".into()).to_string(),
            "self.owner"
        );
    }
    #[test]
    pub(super) fn test_if_expr_display() {
        let expr = VyperExpr::IfExpr(
            Box::new(VyperExpr::IntLit(1)),
            Box::new(VyperExpr::BoolLit(true)),
            Box::new(VyperExpr::IntLit(0)),
        );
        assert_eq!(expr.to_string(), "1 if True else 0");
    }
    #[test]
    pub(super) fn test_convert_display() {
        let expr = VyperExpr::Convert(Box::new(VyperExpr::MsgSender), VyperType::Uint256);
        assert_eq!(expr.to_string(), "convert(msg.sender, uint256)");
    }
    #[test]
    pub(super) fn test_empty_display() {
        let expr = VyperExpr::Empty(VyperType::Uint256);
        assert_eq!(expr.to_string(), "empty(uint256)");
    }
    #[test]
    pub(super) fn test_max_value_display() {
        let expr = VyperExpr::MaxValue(VyperType::Uint256);
        assert_eq!(expr.to_string(), "max_value(uint256)");
    }
    #[test]
    pub(super) fn test_binop_display() {
        let expr = VyperExpr::BinOp(
            "+".into(),
            Box::new(VyperExpr::Var("a".into())),
            Box::new(VyperExpr::IntLit(1)),
        );
        assert_eq!(expr.to_string(), "(a + 1)");
    }
    #[test]
    pub(super) fn test_not_display() {
        let expr = VyperExpr::UnaryOp("not".into(), Box::new(VyperExpr::BoolLit(false)));
        assert_eq!(expr.to_string(), "not False");
    }
    #[test]
    pub(super) fn test_emit_empty_contract() {
        let mut backend = VyperBackend::new();
        backend.set_contract(VyperContract::new("Empty"));
        let src = backend.emit_contract();
        assert!(src.contains("# @version"));
        assert!(src.contains("0.3.10"));
    }
    #[test]
    pub(super) fn test_emit_storage_var_public() {
        let sv = VyperStorageVar {
            name: "owner".into(),
            ty: VyperType::Address,
            is_public: true,
            doc: None,
        };
        let out = VyperBackend::emit_storage_var(&sv);
        assert!(out.contains("owner: address(public)"));
    }
    #[test]
    pub(super) fn test_emit_event() {
        let ev = VyperEvent {
            name: "Transfer".into(),
            fields: vec![
                ("sender".into(), VyperType::Address, true),
                ("receiver".into(), VyperType::Address, true),
                ("value".into(), VyperType::Uint256, false),
            ],
            doc: None,
        };
        let out = VyperBackend::emit_event(&ev);
        assert!(out.contains("event Transfer:"));
        assert!(out.contains("sender: indexed(address)"));
        assert!(out.contains("value: uint256"));
    }
    #[test]
    pub(super) fn test_emit_struct() {
        let s = VyperStruct {
            name: "Point".into(),
            fields: vec![
                ("x".into(), VyperType::Int256),
                ("y".into(), VyperType::Int256),
            ],
            doc: None,
        };
        let out = VyperBackend::emit_struct(&s);
        assert!(out.contains("struct Point:"));
        assert!(out.contains("x: int256"));
    }
    #[test]
    pub(super) fn test_emit_constant() {
        let c = VyperConstant {
            name: "MAX_SUPPLY".into(),
            ty: VyperType::Uint256,
            value: VyperExpr::IntLit(1_000_000),
            doc: None,
        };
        let out = VyperBackend::emit_constant(&c);
        assert!(out.contains("MAX_SUPPLY: constant(uint256) = 1000000"));
    }
    #[test]
    pub(super) fn test_emit_function_with_body() {
        let mut func = VyperFunction::new("get_balance").external().view();
        func.params
            .push(VyperParam::new("addr", VyperType::Address));
        func.return_ty = Some(VyperType::Uint256);
        func.body.push(VyperStmt::Return(Some(VyperExpr::Index(
            Box::new(VyperExpr::SelfField("balances".into())),
            Box::new(VyperExpr::Var("addr".into())),
        ))));
        let out = VyperBackend::emit_function(&func, false);
        assert!(out.contains("@external"));
        assert!(out.contains("@view"));
        assert!(out.contains("def get_balance(addr: address) -> uint256:"));
        assert!(out.contains("return self.balances[addr]"));
    }
    #[test]
    pub(super) fn test_emit_assert_stmt() {
        let stmt = VyperStmt::Assert(
            VyperExpr::BinOp(
                ">".into(),
                Box::new(VyperExpr::Var("amount".into())),
                Box::new(VyperExpr::IntLit(0)),
            ),
            Some("Amount must be positive".into()),
        );
        let out = VyperBackend::emit_stmt(&stmt, 1);
        assert!(out.contains("assert (amount > 0)"));
        assert!(out.contains("Amount must be positive"));
    }
    #[test]
    pub(super) fn test_emit_for_range_stmt() {
        let stmt = VyperStmt::ForRange(
            "i".into(),
            VyperType::Uint256,
            VyperExpr::IntLit(10),
            vec![VyperStmt::Pass],
        );
        let out = VyperBackend::emit_stmt(&stmt, 1);
        assert!(out.contains("for i: uint256 in range(10):"));
        assert!(out.contains("pass"));
    }
    #[test]
    pub(super) fn test_emit_log_stmt() {
        let stmt = VyperStmt::Log(
            "Transfer".into(),
            vec![
                VyperExpr::MsgSender,
                VyperExpr::Var("to".into()),
                VyperExpr::Var("amount".into()),
            ],
        );
        let out = VyperBackend::emit_stmt(&stmt, 1);
        assert!(out.contains("log Transfer(msg.sender, to, amount)"));
    }
    #[test]
    pub(super) fn test_compile_decl() {
        let backend = VyperBackend::new();
        let sv = backend.compile_decl("total_supply", VyperType::Uint256);
        assert_eq!(sv.name, "total_supply");
        assert!(matches!(sv.ty, VyperType::Uint256));
        assert!(!sv.is_public);
    }
    #[test]
    pub(super) fn test_runtime_constant_not_empty() {
        assert!(!VYPER_RUNTIME.is_empty());
        assert!(VYPER_RUNTIME.contains("@external"));
        assert!(VYPER_RUNTIME.contains("oxilean_min"));
    }
    #[test]
    pub(super) fn test_emit_with_runtime() {
        let mut backend = VyperBackend::new().with_runtime();
        backend.set_contract(VyperContract::new("X"));
        let src = backend.emit_contract();
        assert!(src.contains("oxilean_min"));
        assert!(src.contains("oxilean_saturating_add"));
    }
    #[test]
    pub(super) fn test_with_version() {
        let mut backend = VyperBackend::new().with_version("0.4.0");
        backend.set_contract(VyperContract::new("X"));
        let src = backend.emit_contract();
        assert!(src.contains("# @version 0.4.0"));
    }
    #[test]
    pub(super) fn test_emit_flag() {
        let fl = VyperFlagDef {
            name: "Roles".into(),
            variants: vec!["ADMIN".into(), "MINTER".into(), "BURNER".into()],
            doc: None,
        };
        let out = VyperBackend::emit_flag(&fl);
        assert!(out.contains("flag Roles:"));
        assert!(out.contains("ADMIN"));
        assert!(out.contains("MINTER"));
    }
    #[test]
    pub(super) fn test_emit_interface() {
        let mut iface = VyperInterface {
            name: "IERC20".into(),
            functions: Vec::new(),
            doc: None,
        };
        let mut func = VyperFunction::new("transfer");
        func.decorators.push(VyperDecorator::External);
        func.params.push(VyperParam::new("to", VyperType::Address));
        func.params
            .push(VyperParam::new("amount", VyperType::Uint256));
        func.return_ty = Some(VyperType::Bool);
        iface.functions.push(func);
        let out = VyperBackend::emit_interface(&iface);
        assert!(out.contains("interface IERC20:"));
        assert!(out.contains("def transfer(to: address, amount: uint256) -> bool:"));
    }
    #[test]
    pub(super) fn test_needs_init_hashmap() {
        let ty = VyperType::HashMap(Box::new(VyperType::Address), Box::new(VyperType::Uint256));
        assert!(ty.needs_init());
    }
    #[test]
    pub(super) fn test_needs_init_uint256() {
        assert!(!VyperType::Uint256.needs_init());
    }
    #[test]
    pub(super) fn test_raw_call_display() {
        let expr = VyperExpr::RawCall {
            addr: Box::new(VyperExpr::Var("target".into())),
            data: Box::new(VyperExpr::HexLit("0x".into())),
            value: Some(Box::new(VyperExpr::IntLit(0))),
            gas: None,
        };
        let s = expr.to_string();
        assert!(s.contains("raw_call(target, 0x, value=0)"));
    }
    #[test]
    pub(super) fn test_send_stmt() {
        let stmt = VyperStmt::Send(VyperExpr::MsgSender, VyperExpr::IntLit(1000));
        let out = VyperBackend::emit_stmt(&stmt, 1);
        assert!(out.contains("send(msg.sender, 1000)"));
    }
    #[test]
    pub(super) fn test_full_erc20_token_shape() {
        let mut backend = VyperBackend::new();
        let mut contract = VyperContract::new("MyToken");
        contract.doc = Some("A simple ERC20 token".into());
        contract.storage.push(VyperStorageVar {
            name: "balanceOf".into(),
            ty: VyperType::HashMap(Box::new(VyperType::Address), Box::new(VyperType::Uint256)),
            is_public: true,
            doc: None,
        });
        contract.storage.push(VyperStorageVar {
            name: "totalSupply".into(),
            ty: VyperType::Uint256,
            is_public: true,
            doc: None,
        });
        contract.events.push(VyperEvent {
            name: "Transfer".into(),
            fields: vec![
                ("sender".into(), VyperType::Address, true),
                ("receiver".into(), VyperType::Address, true),
                ("value".into(), VyperType::Uint256, false),
            ],
            doc: None,
        });
        let mut transfer = VyperFunction::new("transfer")
            .external()
            .nonreentrant("lock");
        transfer
            .params
            .push(VyperParam::new("receiver", VyperType::Address));
        transfer
            .params
            .push(VyperParam::new("amount", VyperType::Uint256));
        transfer.return_ty = Some(VyperType::Bool);
        transfer.body.push(VyperStmt::Assert(
            VyperExpr::BinOp(
                ">=".into(),
                Box::new(VyperExpr::Index(
                    Box::new(VyperExpr::SelfField("balanceOf".into())),
                    Box::new(VyperExpr::MsgSender),
                )),
                Box::new(VyperExpr::Var("amount".into())),
            ),
            Some("Insufficient balance".into()),
        ));
        transfer.body.push(VyperStmt::AugAssign(
            "-".into(),
            VyperExpr::Index(
                Box::new(VyperExpr::SelfField("balanceOf".into())),
                Box::new(VyperExpr::MsgSender),
            ),
            VyperExpr::Var("amount".into()),
        ));
        transfer.body.push(VyperStmt::AugAssign(
            "+".into(),
            VyperExpr::Index(
                Box::new(VyperExpr::SelfField("balanceOf".into())),
                Box::new(VyperExpr::Var("receiver".into())),
            ),
            VyperExpr::Var("amount".into()),
        ));
        transfer.body.push(VyperStmt::Log(
            "Transfer".into(),
            vec![
                VyperExpr::MsgSender,
                VyperExpr::Var("receiver".into()),
                VyperExpr::Var("amount".into()),
            ],
        ));
        transfer
            .body
            .push(VyperStmt::Return(Some(VyperExpr::BoolLit(true))));
        contract.functions.push(transfer);
        backend.set_contract(contract);
        let src = backend.emit_contract();
        assert!(src.contains("@version"));
        assert!(src.contains("event Transfer:"));
        assert!(src.contains("balanceOf: HashMap[address, uint256](public)"));
        assert!(src.contains("@external"));
        assert!(src.contains("@nonreentrant(\"lock\")"));
        assert!(src.contains("def transfer(receiver: address, amount: uint256) -> bool:"));
        assert!(src.contains("assert"));
        assert!(src.contains("log Transfer("));
        assert!(src.contains("return True"));
    }
}
#[cfg(test)]
mod Vyp_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = VypPassConfig::new("test_pass", VypPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = VypPassStats::new();
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
        let mut reg = VypPassRegistry::new();
        reg.register(VypPassConfig::new("pass_a", VypPassPhase::Analysis));
        reg.register(VypPassConfig::new("pass_b", VypPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = VypAnalysisCache::new(10);
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
        let mut wl = VypWorklist::new();
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
        let mut dt = VypDominatorTree::new(5);
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
        let mut liveness = VypLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(VypConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(VypConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(VypConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            VypConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(VypConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = VypDepGraph::new();
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
mod vyperext_pass_tests {
    use super::*;
    #[test]
    pub(super) fn test_vyperext_phase_order() {
        assert_eq!(VyperExtPassPhase::Early.order(), 0);
        assert_eq!(VyperExtPassPhase::Middle.order(), 1);
        assert_eq!(VyperExtPassPhase::Late.order(), 2);
        assert_eq!(VyperExtPassPhase::Finalize.order(), 3);
        assert!(VyperExtPassPhase::Early.is_early());
        assert!(!VyperExtPassPhase::Early.is_late());
    }
    #[test]
    pub(super) fn test_vyperext_config_builder() {
        let c = VyperExtPassConfig::new("p")
            .with_phase(VyperExtPassPhase::Late)
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
    pub(super) fn test_vyperext_stats() {
        let mut s = VyperExtPassStats::new();
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
    pub(super) fn test_vyperext_registry() {
        let mut r = VyperExtPassRegistry::new();
        r.register(VyperExtPassConfig::new("a").with_phase(VyperExtPassPhase::Early));
        r.register(VyperExtPassConfig::new("b").disabled());
        assert_eq!(r.len(), 2);
        assert_eq!(r.enabled_passes().len(), 1);
        assert_eq!(r.passes_in_phase(&VyperExtPassPhase::Early).len(), 1);
    }
    #[test]
    pub(super) fn test_vyperext_cache() {
        let mut c = VyperExtCache::new(4);
        assert!(c.get(99).is_none());
        c.put(99, vec![1, 2, 3]);
        let v = c.get(99).expect("v should be present in map");
        assert_eq!(v, &[1u8, 2, 3]);
        assert!(c.hit_rate() > 0.0);
        assert_eq!(c.live_count(), 1);
    }
    #[test]
    pub(super) fn test_vyperext_worklist() {
        let mut w = VyperExtWorklist::new(10);
        w.push(5);
        w.push(3);
        w.push(5);
        assert_eq!(w.len(), 2);
        assert!(w.contains(5));
        let first = w.pop().expect("first should be available to pop");
        assert!(!w.contains(first));
    }
    #[test]
    pub(super) fn test_vyperext_dom_tree() {
        let mut dt = VyperExtDomTree::new(5);
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
    pub(super) fn test_vyperext_liveness() {
        let mut lv = VyperExtLiveness::new(3);
        lv.add_def(0, 1);
        lv.add_use(1, 1);
        assert!(lv.var_is_def_in_block(0, 1));
        assert!(lv.var_is_used_in_block(1, 1));
        assert!(!lv.var_is_def_in_block(1, 1));
    }
    #[test]
    pub(super) fn test_vyperext_const_folder() {
        let mut cf = VyperExtConstFolder::new();
        assert_eq!(cf.add_i64(3, 4), Some(7));
        assert_eq!(cf.div_i64(10, 0), None);
        assert_eq!(cf.mul_i64(6, 7), Some(42));
        assert_eq!(cf.and_i64(0b1100, 0b1010), 0b1000);
        assert_eq!(cf.fold_count(), 3);
        assert_eq!(cf.failure_count(), 1);
    }
    #[test]
    pub(super) fn test_vyperext_dep_graph() {
        let mut g = VyperExtDepGraph::new(4);
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
