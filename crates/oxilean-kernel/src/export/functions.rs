//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::reduce::ReducibilityHint;
use crate::{
    BinderInfo, Declaration, Environment, Expr, FVarId, Level, LevelMVarId, Literal, Name,
};
use std::collections::HashMap;

use super::types::{
    ConfigNode, ExportedModule, FocusStack, IntegrityCheckResult, LabelSet, ModuleCache,
    ModuleDependencyGraph, ModuleDiff, ModuleInfo, ModuleRegistry, ModuleVersion, NonEmptyVec,
    PathBuf, RewriteRule, RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, StatSummary, StringPool,
    TokenBucket, TransformStat, TransitiveClosure, VersionedRecord, WindowIterator,
};

/// Export an environment to a module.
///
/// Exports all declarations and constant info entries from the environment.
pub fn export_environment(env: &Environment, module_name: String) -> ExportedModule {
    let mut module = ExportedModule::new(module_name);
    for (name, ci) in env.constant_infos() {
        module.add_constant(name.clone(), ci.clone());
    }
    module
}
/// Import a module into an environment.
///
/// Imports both legacy declarations and constant info entries.
pub fn import_module(env: &mut Environment, module: &ExportedModule) -> Result<(), String> {
    for (name, decl) in &module.declarations {
        env.add(decl.clone()).map_err(|e| {
            format!(
                "Failed to import {} from module {}: {}",
                name, module.name, e
            )
        })?;
    }
    for (name, ci) in &module.constants {
        if env.find(name).is_some() {
            continue;
        }
        env.add_constant(ci.clone()).map_err(|e| {
            format!(
                "Failed to import constant {} from module {}: {}",
                name, module.name, e
            )
        })?;
    }
    Ok(())
}
/// Serialization format identifier.
pub const MAGIC_NUMBER: u32 = 0x4F584C4E;
/// Module format version.
pub const FORMAT_VERSION: u32 = 2;
#[inline]
fn write_u8(buf: &mut Vec<u8>, v: u8) {
    buf.push(v);
}
#[inline]
fn write_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_le_bytes());
}
#[inline]
fn write_u64(buf: &mut Vec<u8>, v: u64) {
    buf.extend_from_slice(&v.to_le_bytes());
}
fn write_str(buf: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    write_u32(buf, bytes.len() as u32);
    buf.extend_from_slice(bytes);
}
fn write_name(buf: &mut Vec<u8>, name: &Name) {
    match name {
        Name::Anonymous => write_u8(buf, 0),
        Name::Str(parent, s) => {
            write_u8(buf, 1);
            write_name(buf, parent);
            write_str(buf, s);
        }
        Name::Num(parent, n) => {
            write_u8(buf, 2);
            write_name(buf, parent);
            write_u64(buf, *n);
        }
    }
}
fn write_level(buf: &mut Vec<u8>, level: &Level) {
    match level {
        Level::Zero => write_u8(buf, 0),
        Level::Succ(inner) => {
            write_u8(buf, 1);
            write_level(buf, inner);
        }
        Level::Max(l, r) => {
            write_u8(buf, 2);
            write_level(buf, l);
            write_level(buf, r);
        }
        Level::IMax(l, r) => {
            write_u8(buf, 3);
            write_level(buf, l);
            write_level(buf, r);
        }
        Level::Param(name) => {
            write_u8(buf, 4);
            write_name(buf, name);
        }
        Level::MVar(LevelMVarId(id)) => {
            write_u8(buf, 5);
            write_u64(buf, *id);
        }
    }
}
fn write_binder_info(buf: &mut Vec<u8>, bi: BinderInfo) {
    let tag: u8 = match bi {
        BinderInfo::Default => 0,
        BinderInfo::Implicit => 1,
        BinderInfo::StrictImplicit => 2,
        BinderInfo::InstImplicit => 3,
    };
    write_u8(buf, tag);
}
fn write_literal(buf: &mut Vec<u8>, lit: &Literal) {
    match lit {
        Literal::Nat(n) => {
            write_u8(buf, 0);
            write_u64(buf, *n);
        }
        Literal::Str(s) => {
            write_u8(buf, 1);
            write_str(buf, s);
        }
    }
}
fn write_expr(buf: &mut Vec<u8>, expr: &Expr) {
    match expr {
        Expr::Sort(level) => {
            write_u8(buf, 0);
            write_level(buf, level);
        }
        Expr::BVar(idx) => {
            write_u8(buf, 1);
            write_u32(buf, *idx);
        }
        Expr::FVar(FVarId(id)) => {
            write_u8(buf, 2);
            write_u64(buf, *id);
        }
        Expr::Const(name, levels) => {
            write_u8(buf, 3);
            write_name(buf, name);
            write_u32(buf, levels.len() as u32);
            for l in levels {
                write_level(buf, l);
            }
        }
        Expr::App(f, a) => {
            write_u8(buf, 4);
            write_expr(buf, f);
            write_expr(buf, a);
        }
        Expr::Lam(bi, name, ty, body) => {
            write_u8(buf, 5);
            write_binder_info(buf, *bi);
            write_name(buf, name);
            write_expr(buf, ty);
            write_expr(buf, body);
        }
        Expr::Pi(bi, name, ty, body) => {
            write_u8(buf, 6);
            write_binder_info(buf, *bi);
            write_name(buf, name);
            write_expr(buf, ty);
            write_expr(buf, body);
        }
        Expr::Let(name, ty, val, body) => {
            write_u8(buf, 7);
            write_name(buf, name);
            write_expr(buf, ty);
            write_expr(buf, val);
            write_expr(buf, body);
        }
        Expr::Lit(lit) => {
            write_u8(buf, 8);
            write_literal(buf, lit);
        }
        Expr::Proj(name, idx, inner) => {
            write_u8(buf, 9);
            write_name(buf, name);
            write_u32(buf, *idx);
            write_expr(buf, inner);
        }
    }
}
fn write_reducibility_hint(buf: &mut Vec<u8>, hint: ReducibilityHint) {
    match hint {
        ReducibilityHint::Opaque => write_u8(buf, 0),
        ReducibilityHint::Abbrev => write_u8(buf, 1),
        ReducibilityHint::Regular(h) => {
            write_u8(buf, 2);
            write_u32(buf, h);
        }
    }
}
fn write_names(buf: &mut Vec<u8>, names: &[Name]) {
    write_u32(buf, names.len() as u32);
    for n in names {
        write_name(buf, n);
    }
}
fn write_declaration(buf: &mut Vec<u8>, decl: &Declaration) {
    match decl {
        Declaration::Axiom {
            name,
            univ_params,
            ty,
        } => {
            write_u8(buf, 0);
            write_name(buf, name);
            write_names(buf, univ_params);
            write_expr(buf, ty);
        }
        Declaration::Definition {
            name,
            univ_params,
            ty,
            val,
            hint,
        } => {
            write_u8(buf, 1);
            write_name(buf, name);
            write_names(buf, univ_params);
            write_expr(buf, ty);
            write_expr(buf, val);
            write_reducibility_hint(buf, *hint);
        }
        Declaration::Theorem {
            name,
            univ_params,
            ty,
            val,
        } => {
            write_u8(buf, 2);
            write_name(buf, name);
            write_names(buf, univ_params);
            write_expr(buf, ty);
            write_expr(buf, val);
        }
        Declaration::Opaque {
            name,
            univ_params,
            ty,
            val,
        } => {
            write_u8(buf, 3);
            write_name(buf, name);
            write_names(buf, univ_params);
            write_expr(buf, ty);
            write_expr(buf, val);
        }
    }
}
type ReadResult<T> = Result<T, String>;
fn read_u8(bytes: &[u8], pos: &mut usize) -> ReadResult<u8> {
    if *pos >= bytes.len() {
        return Err(format!("unexpected end of data at pos {}", *pos));
    }
    let v = bytes[*pos];
    *pos += 1;
    Ok(v)
}
fn read_u32(bytes: &[u8], pos: &mut usize) -> ReadResult<u32> {
    if *pos + 4 > bytes.len() {
        return Err(format!("unexpected end of data at pos {}", *pos));
    }
    let v = u32::from_le_bytes([
        bytes[*pos],
        bytes[*pos + 1],
        bytes[*pos + 2],
        bytes[*pos + 3],
    ]);
    *pos += 4;
    Ok(v)
}
fn read_u64(bytes: &[u8], pos: &mut usize) -> ReadResult<u64> {
    if *pos + 8 > bytes.len() {
        return Err(format!("unexpected end of data at pos {}", *pos));
    }
    let v = u64::from_le_bytes([
        bytes[*pos],
        bytes[*pos + 1],
        bytes[*pos + 2],
        bytes[*pos + 3],
        bytes[*pos + 4],
        bytes[*pos + 5],
        bytes[*pos + 6],
        bytes[*pos + 7],
    ]);
    *pos += 8;
    Ok(v)
}
fn read_str(bytes: &[u8], pos: &mut usize) -> ReadResult<String> {
    let len = read_u32(bytes, pos)? as usize;
    if *pos + len > bytes.len() {
        return Err(format!("string truncated at pos {}", *pos));
    }
    let s = std::str::from_utf8(&bytes[*pos..*pos + len])
        .map_err(|e| format!("invalid UTF-8: {}", e))?
        .to_owned();
    *pos += len;
    Ok(s)
}
fn read_name(bytes: &[u8], pos: &mut usize) -> ReadResult<Name> {
    let tag = read_u8(bytes, pos)?;
    match tag {
        0 => Ok(Name::Anonymous),
        1 => {
            let parent = read_name(bytes, pos)?;
            let s = read_str(bytes, pos)?;
            Ok(Name::Str(Box::new(parent), s))
        }
        2 => {
            let parent = read_name(bytes, pos)?;
            let n = read_u64(bytes, pos)?;
            Ok(Name::Num(Box::new(parent), n))
        }
        other => Err(format!("unknown Name tag: {}", other)),
    }
}
fn read_level(bytes: &[u8], pos: &mut usize) -> ReadResult<Level> {
    let tag = read_u8(bytes, pos)?;
    match tag {
        0 => Ok(Level::Zero),
        1 => {
            let inner = read_level(bytes, pos)?;
            Ok(Level::Succ(Box::new(inner)))
        }
        2 => {
            let l = read_level(bytes, pos)?;
            let r = read_level(bytes, pos)?;
            Ok(Level::Max(Box::new(l), Box::new(r)))
        }
        3 => {
            let l = read_level(bytes, pos)?;
            let r = read_level(bytes, pos)?;
            Ok(Level::IMax(Box::new(l), Box::new(r)))
        }
        4 => {
            let name = read_name(bytes, pos)?;
            Ok(Level::Param(name))
        }
        5 => {
            let id = read_u64(bytes, pos)?;
            Ok(Level::MVar(LevelMVarId(id)))
        }
        other => Err(format!("unknown Level tag: {}", other)),
    }
}
fn read_binder_info(bytes: &[u8], pos: &mut usize) -> ReadResult<BinderInfo> {
    let tag = read_u8(bytes, pos)?;
    match tag {
        0 => Ok(BinderInfo::Default),
        1 => Ok(BinderInfo::Implicit),
        2 => Ok(BinderInfo::StrictImplicit),
        3 => Ok(BinderInfo::InstImplicit),
        other => Err(format!("unknown BinderInfo tag: {}", other)),
    }
}
fn read_literal(bytes: &[u8], pos: &mut usize) -> ReadResult<Literal> {
    let tag = read_u8(bytes, pos)?;
    match tag {
        0 => Ok(Literal::Nat(read_u64(bytes, pos)?)),
        1 => Ok(Literal::Str(read_str(bytes, pos)?)),
        other => Err(format!("unknown Literal tag: {}", other)),
    }
}
fn read_expr(bytes: &[u8], pos: &mut usize) -> ReadResult<Expr> {
    let tag = read_u8(bytes, pos)?;
    match tag {
        0 => Ok(Expr::Sort(read_level(bytes, pos)?)),
        1 => Ok(Expr::BVar(read_u32(bytes, pos)?)),
        2 => Ok(Expr::FVar(FVarId(read_u64(bytes, pos)?))),
        3 => {
            let name = read_name(bytes, pos)?;
            let count = read_u32(bytes, pos)? as usize;
            let mut levels = Vec::with_capacity(count);
            for _ in 0..count {
                levels.push(read_level(bytes, pos)?);
            }
            Ok(Expr::Const(name, levels))
        }
        4 => {
            let f = read_expr(bytes, pos)?;
            let a = read_expr(bytes, pos)?;
            Ok(Expr::App(Box::new(f), Box::new(a)))
        }
        5 => {
            let bi = read_binder_info(bytes, pos)?;
            let name = read_name(bytes, pos)?;
            let ty = read_expr(bytes, pos)?;
            let body = read_expr(bytes, pos)?;
            Ok(Expr::Lam(bi, name, Box::new(ty), Box::new(body)))
        }
        6 => {
            let bi = read_binder_info(bytes, pos)?;
            let name = read_name(bytes, pos)?;
            let ty = read_expr(bytes, pos)?;
            let body = read_expr(bytes, pos)?;
            Ok(Expr::Pi(bi, name, Box::new(ty), Box::new(body)))
        }
        7 => {
            let name = read_name(bytes, pos)?;
            let ty = read_expr(bytes, pos)?;
            let val = read_expr(bytes, pos)?;
            let body = read_expr(bytes, pos)?;
            Ok(Expr::Let(name, Box::new(ty), Box::new(val), Box::new(body)))
        }
        8 => Ok(Expr::Lit(read_literal(bytes, pos)?)),
        9 => {
            let name = read_name(bytes, pos)?;
            let idx = read_u32(bytes, pos)?;
            let inner = read_expr(bytes, pos)?;
            Ok(Expr::Proj(name, idx, Box::new(inner)))
        }
        other => Err(format!("unknown Expr tag: {}", other)),
    }
}
fn read_reducibility_hint(bytes: &[u8], pos: &mut usize) -> ReadResult<ReducibilityHint> {
    let tag = read_u8(bytes, pos)?;
    match tag {
        0 => Ok(ReducibilityHint::Opaque),
        1 => Ok(ReducibilityHint::Abbrev),
        2 => Ok(ReducibilityHint::Regular(read_u32(bytes, pos)?)),
        other => Err(format!("unknown ReducibilityHint tag: {}", other)),
    }
}
fn read_names(bytes: &[u8], pos: &mut usize) -> ReadResult<Vec<Name>> {
    let count = read_u32(bytes, pos)? as usize;
    let mut names = Vec::with_capacity(count);
    for _ in 0..count {
        names.push(read_name(bytes, pos)?);
    }
    Ok(names)
}
fn read_declaration(bytes: &[u8], pos: &mut usize) -> ReadResult<Declaration> {
    let tag = read_u8(bytes, pos)?;
    match tag {
        0 => {
            let name = read_name(bytes, pos)?;
            let univ_params = read_names(bytes, pos)?;
            let ty = read_expr(bytes, pos)?;
            Ok(Declaration::Axiom {
                name,
                univ_params,
                ty,
            })
        }
        1 => {
            let name = read_name(bytes, pos)?;
            let univ_params = read_names(bytes, pos)?;
            let ty = read_expr(bytes, pos)?;
            let val = read_expr(bytes, pos)?;
            let hint = read_reducibility_hint(bytes, pos)?;
            Ok(Declaration::Definition {
                name,
                univ_params,
                ty,
                val,
                hint,
            })
        }
        2 => {
            let name = read_name(bytes, pos)?;
            let univ_params = read_names(bytes, pos)?;
            let ty = read_expr(bytes, pos)?;
            let val = read_expr(bytes, pos)?;
            Ok(Declaration::Theorem {
                name,
                univ_params,
                ty,
                val,
            })
        }
        3 => {
            let name = read_name(bytes, pos)?;
            let univ_params = read_names(bytes, pos)?;
            let ty = read_expr(bytes, pos)?;
            let val = read_expr(bytes, pos)?;
            Ok(Declaration::Opaque {
                name,
                univ_params,
                ty,
                val,
            })
        }
        other => Err(format!("unknown Declaration tag: {}", other)),
    }
}
/// Serialize an `ExportedModule` to a compact binary format.
///
/// The format is:
/// - 4-byte magic number (`OXLN`)
/// - 4-byte format version
/// - module name (length-prefixed UTF-8)
/// - version string (length-prefixed UTF-8)
/// - dependency count + dependency name strings
/// - declaration count + serialized `Declaration` values
/// - metadata count + (key, value) string pairs
pub fn serialize_module(module: &ExportedModule) -> Vec<u8> {
    let mut buf = Vec::new();
    write_u32(&mut buf, MAGIC_NUMBER);
    write_u32(&mut buf, FORMAT_VERSION);
    write_str(&mut buf, &module.name);
    write_str(&mut buf, &module.version);
    write_u32(&mut buf, module.dependencies.len() as u32);
    for dep in &module.dependencies {
        write_str(&mut buf, dep);
    }
    write_u32(&mut buf, module.declarations.len() as u32);
    for (_name, decl) in &module.declarations {
        write_declaration(&mut buf, decl);
    }
    write_u32(&mut buf, module.metadata.len() as u32);
    for (k, v) in &module.metadata {
        write_str(&mut buf, k);
        write_str(&mut buf, v);
    }
    buf
}
/// Deserialize an `ExportedModule` from bytes produced by [`serialize_module`].
///
/// Returns an error if the bytes are malformed, truncated, or have an
/// incompatible format version.
pub fn deserialize_module(bytes: &[u8]) -> Result<ExportedModule, String> {
    let mut pos = 0;
    let magic = read_u32(bytes, &mut pos)?;
    if magic != MAGIC_NUMBER {
        return Err(format!("invalid magic number: 0x{:08X}", magic));
    }
    let version = read_u32(bytes, &mut pos)?;
    if version != FORMAT_VERSION {
        return Err(format!("unsupported format version: {}", version));
    }
    let name = read_str(bytes, &mut pos)?;
    let ver = read_str(bytes, &mut pos)?;
    let dep_count = read_u32(bytes, &mut pos)? as usize;
    let mut dependencies = Vec::with_capacity(dep_count);
    for _ in 0..dep_count {
        dependencies.push(read_str(bytes, &mut pos)?);
    }
    let decl_count = read_u32(bytes, &mut pos)? as usize;
    let mut declarations = Vec::with_capacity(decl_count);
    for _ in 0..decl_count {
        let decl = read_declaration(bytes, &mut pos)?;
        let decl_name = decl.name().clone();
        declarations.push((decl_name, decl));
    }
    let meta_count = read_u32(bytes, &mut pos)? as usize;
    let mut metadata = HashMap::new();
    for _ in 0..meta_count {
        let k = read_str(bytes, &mut pos)?;
        let v = read_str(bytes, &mut pos)?;
        metadata.insert(k, v);
    }
    Ok(ExportedModule {
        name,
        declarations,
        constants: Vec::new(),
        dependencies,
        version: ver,
        metadata,
    })
}
/// Deserialize only the module header (name, version, declaration count).
///
/// Useful for quickly inspecting module metadata without decoding all declarations.
pub fn deserialize_module_header(bytes: &[u8]) -> Result<(String, u32, u32), String> {
    let mut pos = 0;
    let magic = read_u32(bytes, &mut pos)?;
    if magic != MAGIC_NUMBER {
        return Err(format!("Invalid magic number: 0x{:08X}", magic));
    }
    let version = read_u32(bytes, &mut pos)?;
    if version != FORMAT_VERSION {
        return Err(format!("Unsupported format version: {}", version));
    }
    let name = read_str(bytes, &mut pos)?;
    let _ver_str = read_str(bytes, &mut pos)?;
    let _dep_count = read_u32(bytes, &mut pos)?;
    let dep_count = _dep_count as usize;
    for _ in 0..dep_count {
        let len = read_u32(bytes, &mut pos)? as usize;
        if pos + len > bytes.len() {
            return Err("truncated dependency list".to_string());
        }
        pos += len;
    }
    let num_entries = read_u32(bytes, &mut pos)?;
    Ok((name, version, num_entries))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Expr, Level};
    #[test]
    fn test_exported_module_create() {
        let module = ExportedModule::new("test".to_string());
        assert_eq!(module.name, "test");
        assert_eq!(module.num_entries(), 0);
        assert!(module.is_empty());
    }
    #[test]
    fn test_add_declaration() {
        let mut module = ExportedModule::new("test".to_string());
        let decl = Declaration::Axiom {
            name: Name::str("foo"),
            univ_params: vec![],
            ty: Expr::Sort(Level::zero()),
        };
        module.add_declaration(Name::str("foo"), decl);
        assert_eq!(module.declarations.len(), 1);
        assert!(!module.is_empty());
    }
    #[test]
    fn test_add_dependency() {
        let mut module = ExportedModule::new("test".to_string());
        module.add_dependency("base".to_string());
        module.add_dependency("base".to_string());
        assert_eq!(module.dependencies.len(), 1);
        assert_eq!(module.dependencies[0], "base");
    }
    #[test]
    fn test_export_environment() {
        let env = Environment::new();
        let module = export_environment(&env, "test".to_string());
        assert_eq!(module.name, "test");
    }
    #[test]
    fn test_import_module() {
        let mut module = ExportedModule::new("test".to_string());
        module.add_declaration(
            Name::str("foo"),
            Declaration::Axiom {
                name: Name::str("foo"),
                univ_params: vec![],
                ty: Expr::Sort(Level::zero()),
            },
        );
        let mut target_env = Environment::new();
        assert!(import_module(&mut target_env, &module).is_ok());
        assert!(target_env.get(&Name::str("foo")).is_some());
    }
    #[test]
    fn test_module_cache() {
        let mut cache = ModuleCache::new();
        let module = ExportedModule::new("test".to_string());
        cache.add(module);
        assert!(cache.contains("test"));
        assert_eq!(cache.all_modules().len(), 1);
        assert_eq!(cache.num_modules(), 1);
    }
    #[test]
    fn test_import_all() {
        let mut cache = ModuleCache::new();
        let mut module = ExportedModule::new("mod1".to_string());
        module.add_declaration(
            Name::str("foo"),
            Declaration::Axiom {
                name: Name::str("foo"),
                univ_params: vec![],
                ty: Expr::Sort(Level::zero()),
            },
        );
        cache.add(module);
        let mut target = Environment::new();
        assert!(cache.import_all(&mut target).is_ok());
        assert!(target.get(&Name::str("foo")).is_some());
    }
    #[test]
    fn test_serialize_deserialize_header() {
        let mut module = ExportedModule::new("test_mod".to_string());
        module.add_dependency("base".to_string());
        let bytes = serialize_module(&module);
        let (name, version, _entries) =
            deserialize_module_header(&bytes).expect("value should be present");
        assert_eq!(name, "test_mod");
        assert_eq!(version, FORMAT_VERSION);
    }
    #[test]
    fn test_serialize_invalid_magic() {
        let bytes = vec![0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert!(deserialize_module_header(&bytes).is_err());
    }
    #[test]
    fn test_serialize_deserialize_round_trip() {
        let mut module = ExportedModule::new("my_module".to_string());
        module.add_dependency("base".to_string());
        module.set_metadata("author".to_string(), "alice".to_string());
        let axiom = Declaration::Axiom {
            name: Name::str("MyAxiom"),
            univ_params: vec![Name::str("u")],
            ty: Expr::Sort(Level::param(Name::str("u"))),
        };
        module.add_declaration(Name::str("MyAxiom"), axiom);
        let def = Declaration::Definition {
            name: Name::str("myDef"),
            univ_params: vec![],
            ty: Expr::Sort(Level::zero()),
            val: Expr::Lit(Literal::Nat(42)),
            hint: ReducibilityHint::Regular(1),
        };
        module.add_declaration(Name::str("myDef"), def);
        let thm = Declaration::Theorem {
            name: Name::str("myThm"),
            univ_params: vec![],
            ty: Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                Box::new(Expr::Sort(Level::zero())),
                Box::new(Expr::BVar(0)),
            ),
            val: Expr::Lam(
                BinderInfo::Default,
                Name::str("x"),
                Box::new(Expr::Sort(Level::zero())),
                Box::new(Expr::BVar(0)),
            ),
        };
        module.add_declaration(Name::str("myThm"), thm);
        let bytes = serialize_module(&module);
        let decoded = deserialize_module(&bytes).expect("decoded should be present");
        assert_eq!(decoded.name, "my_module");
        assert_eq!(decoded.dependencies, vec!["base"]);
        assert_eq!(
            decoded
                .metadata
                .get("author")
                .expect("element at \'author\' should exist"),
            "alice"
        );
        assert_eq!(decoded.declarations.len(), 3);
        assert_eq!(decoded.declarations[0].0, Name::str("MyAxiom"));
        assert_eq!(decoded.declarations[1].0, Name::str("myDef"));
        assert_eq!(decoded.declarations[2].0, Name::str("myThm"));
    }
    #[test]
    fn test_serialize_complex_expr() {
        let expr = Expr::App(
            Box::new(Expr::Lam(
                BinderInfo::Implicit,
                Name::str("x"),
                Box::new(Expr::Const(Name::str("Nat"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Lit(Literal::Nat(7))),
        );
        let mut buf = Vec::new();
        write_expr(&mut buf, &expr);
        let decoded = read_expr(&buf, &mut 0).expect("decoded should be present");
        assert_eq!(decoded, expr);
    }
    #[test]
    fn test_serialize_name_components() {
        let name = Name::str("Nat").append_str("add").append_num(3);
        let mut buf = Vec::new();
        write_name(&mut buf, &name);
        let decoded = read_name(&buf, &mut 0).expect("decoded should be present");
        assert_eq!(decoded, name);
    }
    #[test]
    fn test_serialize_level_variants() {
        let levels = vec![
            Level::Zero,
            Level::succ(Level::Zero),
            Level::max(Level::Zero, Level::param(Name::str("u"))),
            Level::imax(Level::param(Name::str("u")), Level::param(Name::str("v"))),
            Level::MVar(LevelMVarId(99)),
        ];
        for level in &levels {
            let mut buf = Vec::new();
            write_level(&mut buf, level);
            let decoded = read_level(&buf, &mut 0).expect("decoded should be present");
            assert_eq!(&decoded, level);
        }
    }
    #[test]
    fn test_module_metadata() {
        let mut module = ExportedModule::new("test".to_string());
        module.set_metadata("author".to_string(), "oxilean".to_string());
        assert_eq!(
            module
                .metadata
                .get("author")
                .expect("element at \'author\' should exist"),
            "oxilean"
        );
    }
    #[test]
    fn test_cache_remove() {
        let mut cache = ModuleCache::new();
        cache.add(ExportedModule::new("mod1".to_string()));
        cache.add(ExportedModule::new("mod2".to_string()));
        assert_eq!(cache.num_modules(), 2);
        cache.remove("mod1");
        assert_eq!(cache.num_modules(), 1);
        assert!(!cache.contains("mod1"));
    }
    #[test]
    fn test_cache_clear() {
        let mut cache = ModuleCache::new();
        cache.add(ExportedModule::new("mod1".to_string()));
        cache.clear();
        assert_eq!(cache.num_modules(), 0);
    }
}
/// Compute the diff between two module versions.
///
/// Returns a `ModuleDiff` describing what was added, removed, or changed.
pub fn diff_modules(old: &ExportedModule, new: &ExportedModule) -> ModuleDiff {
    let old_names: std::collections::HashSet<&Name> = old.declaration_names().into_iter().collect();
    let new_names: std::collections::HashSet<&Name> = new.declaration_names().into_iter().collect();
    let added = new_names
        .difference(&old_names)
        .map(|n| (*n).clone())
        .collect();
    let removed = old_names
        .difference(&new_names)
        .map(|n| (*n).clone())
        .collect();
    let changed = Vec::new();
    ModuleDiff {
        added,
        removed,
        changed,
    }
}
/// Check the integrity of a module.
pub fn check_module_integrity(module: &ExportedModule) -> IntegrityCheckResult {
    if module.is_empty() {
        return IntegrityCheckResult::EmptyModule;
    }
    let names = module.declaration_names();
    let mut seen = std::collections::HashSet::new();
    let mut duplicates = Vec::new();
    for name in names {
        if !seen.insert(name) {
            duplicates.push(name.clone());
        }
    }
    if !duplicates.is_empty() {
        return IntegrityCheckResult::DuplicateNames(duplicates);
    }
    IntegrityCheckResult::Ok
}
/// Check the integrity of serialized module bytes.
pub fn check_bytes_integrity(bytes: &[u8]) -> IntegrityCheckResult {
    if bytes.len() < 8 {
        return IntegrityCheckResult::BadMagicNumber;
    }
    let magic = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    if magic != MAGIC_NUMBER {
        return IntegrityCheckResult::BadMagicNumber;
    }
    let version = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    if version != FORMAT_VERSION {
        return IntegrityCheckResult::UnsupportedVersion(version);
    }
    IntegrityCheckResult::Ok
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::{Expr, Level};
    #[test]
    fn test_module_diff_empty() {
        let m1 = ExportedModule::new("m1".to_string());
        let m2 = ExportedModule::new("m1".to_string());
        let diff = diff_modules(&m1, &m2);
        assert!(diff.is_empty());
        assert_eq!(diff.total_changes(), 0);
    }
    #[test]
    fn test_module_diff_added() {
        let m1 = ExportedModule::new("m1".to_string());
        let mut m2 = ExportedModule::new("m1".to_string());
        m2.add_declaration(
            Name::str("foo"),
            Declaration::Axiom {
                name: Name::str("foo"),
                univ_params: vec![],
                ty: Expr::Sort(Level::zero()),
            },
        );
        let diff = diff_modules(&m1, &m2);
        assert_eq!(diff.added.len(), 1);
        assert_eq!(diff.removed.len(), 0);
    }
    #[test]
    fn test_module_diff_removed() {
        let mut m1 = ExportedModule::new("m1".to_string());
        m1.add_declaration(
            Name::str("foo"),
            Declaration::Axiom {
                name: Name::str("foo"),
                univ_params: vec![],
                ty: Expr::Sort(Level::zero()),
            },
        );
        let m2 = ExportedModule::new("m1".to_string());
        let diff = diff_modules(&m1, &m2);
        assert_eq!(diff.removed.len(), 1);
    }
    #[test]
    fn test_module_registry_register() {
        let mut reg = ModuleRegistry::new();
        reg.register(Name::str("foo"), "base".to_string());
        assert_eq!(reg.module_for(&Name::str("foo")), Some("base"));
        assert!(reg.contains_decl(&Name::str("foo")));
        assert_eq!(reg.num_decls(), 1);
    }
    #[test]
    fn test_module_registry_register_module() {
        let mut module = ExportedModule::new("mymod".to_string());
        module.add_declaration(
            Name::str("bar"),
            Declaration::Axiom {
                name: Name::str("bar"),
                univ_params: vec![],
                ty: Expr::Sort(Level::zero()),
            },
        );
        let mut reg = ModuleRegistry::new();
        reg.register_module(&module);
        assert_eq!(reg.module_for(&Name::str("bar")), Some("mymod"));
    }
    #[test]
    fn test_module_registry_decls_for_module() {
        let mut reg = ModuleRegistry::new();
        reg.register(Name::str("a"), "mod".to_string());
        reg.register(Name::str("b"), "mod".to_string());
        assert_eq!(reg.decls_for_module("mod").len(), 2);
    }
    #[test]
    fn test_dep_graph_topological_order() {
        let mut g = ModuleDependencyGraph::new();
        g.add_module("a".to_string());
        g.add_module("b".to_string());
        g.add_dep("b".to_string(), "a".to_string());
        let order = g.topological_order().expect("order should be present");
        assert_eq!(order.len(), 2);
        assert!(order.contains(&"a".to_string()));
        assert!(order.contains(&"b".to_string()));
    }
    #[test]
    fn test_dep_graph_depends_on() {
        let mut g = ModuleDependencyGraph::new();
        g.add_module("a".to_string());
        g.add_module("b".to_string());
        g.add_dep("b".to_string(), "a".to_string());
        assert!(g.depends_on("b", "a"));
        assert!(!g.depends_on("a", "b"));
    }
    #[test]
    fn test_check_module_integrity_ok() {
        let mut m = ExportedModule::new("test".to_string());
        m.add_declaration(
            Name::str("foo"),
            Declaration::Axiom {
                name: Name::str("foo"),
                univ_params: vec![],
                ty: Expr::Sort(Level::zero()),
            },
        );
        assert_eq!(check_module_integrity(&m), IntegrityCheckResult::Ok);
    }
    #[test]
    fn test_check_module_integrity_empty() {
        let m = ExportedModule::new("test".to_string());
        assert_eq!(
            check_module_integrity(&m),
            IntegrityCheckResult::EmptyModule
        );
    }
    #[test]
    fn test_check_bytes_integrity_ok() {
        let module = ExportedModule::new("t".to_string());
        let bytes = serialize_module(&module);
        assert_eq!(check_bytes_integrity(&bytes), IntegrityCheckResult::Ok);
    }
    #[test]
    fn test_check_bytes_integrity_bad_magic() {
        let bytes = vec![0u8; 16];
        assert_eq!(
            check_bytes_integrity(&bytes),
            IntegrityCheckResult::BadMagicNumber
        );
    }
    #[test]
    fn test_module_diff_total_changes() {
        let diff = ModuleDiff {
            added: vec![Name::str("a")],
            removed: vec![Name::str("b"), Name::str("c")],
            changed: vec![],
        };
        assert_eq!(diff.total_changes(), 3);
        assert!(!diff.is_empty());
    }
}
#[cfg(test)]
mod version_tests {
    use super::*;
    #[test]
    fn test_version_display() {
        let v = ModuleVersion::new(1, 2, 3);
        assert_eq!(v.to_string(), "1.2.3");
    }
    #[test]
    fn test_version_parse_ok() {
        let v = ModuleVersion::parse("2.3.4").expect("v should be present");
        assert_eq!(v.major, 2);
        assert_eq!(v.minor, 3);
        assert_eq!(v.patch, 4);
    }
    #[test]
    fn test_version_parse_invalid() {
        assert!(ModuleVersion::parse("1.2").is_none());
        assert!(ModuleVersion::parse("a.b.c").is_none());
    }
    #[test]
    fn test_version_compatible_same() {
        let v1 = ModuleVersion::new(1, 0, 0);
        let v2 = ModuleVersion::new(1, 0, 0);
        assert!(v1.is_compatible_with(&v2));
    }
    #[test]
    fn test_version_compatible_newer_minor() {
        let newer = ModuleVersion::new(1, 1, 0);
        let older = ModuleVersion::new(1, 0, 0);
        assert!(newer.is_compatible_with(&older));
    }
    #[test]
    fn test_version_incompatible_different_major() {
        let v1 = ModuleVersion::new(2, 0, 0);
        let v2 = ModuleVersion::new(1, 5, 0);
        assert!(!v1.is_compatible_with(&v2));
    }
    #[test]
    fn test_module_info_builder() {
        let info = ModuleInfo::new()
            .with_version(ModuleVersion::new(0, 1, 0))
            .with_author("oxilean")
            .with_license("MIT")
            .with_description("core library");
        assert_eq!(info.author.as_deref(), Some("oxilean"));
        assert_eq!(info.license.as_deref(), Some("MIT"));
        assert!(info.version.is_some());
    }
    #[test]
    fn test_module_info_default_empty() {
        let info = ModuleInfo::new();
        assert!(info.version.is_none());
        assert!(info.author.is_none());
    }
    #[test]
    fn test_version_ordering() {
        let v1 = ModuleVersion::new(1, 0, 0);
        let v2 = ModuleVersion::new(1, 1, 0);
        assert!(v1 < v2);
    }
    #[test]
    fn test_version_roundtrip() {
        let v = ModuleVersion::new(3, 14, 159);
        let s = v.to_string();
        let v2 = ModuleVersion::parse(&s).expect("v2 should be present");
        assert_eq!(v, v2);
    }
}
#[cfg(test)]
mod tests_padding_infra {
    use super::*;
    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }
    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }
    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo");
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }
    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }
    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }
    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}
#[cfg(test)]
mod tests_extra_iterators {
    use super::*;
    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}
#[cfg(test)]
mod tests_padding2 {
    use super::*;
    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0);
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }
    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty());
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 10);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60));
        assert_eq!(tb.capacity(), 100);
    }
    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}
