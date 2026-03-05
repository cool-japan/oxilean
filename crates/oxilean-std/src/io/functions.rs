//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    AsyncTaskQueue, BufferedReader, BufferedWriter, Capability, DelimiterSplitter, EnvRegistry,
    FileMetadata, HoareVerifier, IoAction, IoActionPipeline, IoError, IoErrorKind, MockFs,
    RecordWriter, SessionAction, SessionChannel, StmLog,
};

/// Build IO type in the environment.
pub fn build_io_env(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    let io_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(type2.clone()),
    );
    env.add(Declaration::Axiom {
        name: Name::str("IO"),
        univ_params: vec![],
        ty: io_ty,
    })
    .map_err(|e| e.to_string())?;
    let pure_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(Expr::BVar(0)),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("IO"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("IO.pure"),
        univ_params: vec![],
        ty: pure_ty,
    })
    .map_err(|e| e.to_string())?;
    let bind_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("ma"),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("IO"), vec![])),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("f"),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(Expr::BVar(2)),
                        Box::new(Expr::App(
                            Box::new(Expr::Const(Name::str("IO"), vec![])),
                            Box::new(Expr::BVar(2)),
                        )),
                    )),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("IO"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("IO.bind"),
        univ_params: vec![],
        ty: bind_ty,
    })
    .map_err(|e| e.to_string())?;
    let println_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(Expr::Const(Name::str("String"), vec![])),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("IO"), vec![])),
            Box::new(Expr::Const(Name::str("Unit"), vec![])),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("IO.println"),
        univ_params: vec![],
        ty: println_ty,
    })
    .map_err(|e| e.to_string())?;
    let readline_ty = Expr::App(
        Box::new(Expr::Const(Name::str("IO"), vec![])),
        Box::new(Expr::Const(Name::str("String"), vec![])),
    );
    env.add(Declaration::Axiom {
        name: Name::str("IO.readLine"),
        univ_params: vec![],
        ty: readline_ty,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
/// Build `IO.throw : String → IO α` for the environment.
pub fn build_io_throw(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let throw_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("msg"),
            Box::new(Expr::Const(Name::str("String"), vec![])),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("IO"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("IO.throw"),
        univ_params: vec![],
        ty: throw_ty,
    })
    .map_err(|e| e.to_string())
}
/// Build `IO.catch : IO α → (String → IO α) → IO α` for the environment.
pub fn build_io_catch(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let catch_ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("action"),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("IO"), vec![])),
                Box::new(Expr::BVar(0)),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("handler"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("err"),
                    Box::new(Expr::Const(Name::str("String"), vec![])),
                    Box::new(Expr::App(
                        Box::new(Expr::Const(Name::str("IO"), vec![])),
                        Box::new(Expr::BVar(2)),
                    )),
                )),
                Box::new(Expr::App(
                    Box::new(Expr::Const(Name::str("IO"), vec![])),
                    Box::new(Expr::BVar(2)),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("IO.catch"),
        univ_params: vec![],
        ty: catch_ty,
    })
    .map_err(|e| e.to_string())
}
/// Build `IO.getEnv : String → IO (Option String)` for the environment.
pub fn build_io_getenv(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let type2 = Expr::Sort(Level::succ(Level::succ(Level::zero())));
    let opt_present = env.get(&Name::str("Option")).is_some();
    if !opt_present {
        let option_ty = Expr::Pi(
            BinderInfo::Default,
            Name::str("α"),
            Box::new(type1.clone()),
            Box::new(type2.clone()),
        );
        env.add(Declaration::Axiom {
            name: Name::str("Option"),
            univ_params: vec![],
            ty: option_ty,
        })
        .map_err(|e| e.to_string())?;
    }
    let getenv_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("key"),
        Box::new(Expr::Const(Name::str("String"), vec![])),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("IO"), vec![])),
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Option"), vec![])),
                Box::new(Expr::Const(Name::str("String"), vec![])),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("IO.getEnv"),
        univ_params: vec![],
        ty: getenv_ty,
    })
    .map_err(|e| e.to_string())
}
/// Build all extended IO operations into the environment.
pub fn build_io_extended(env: &mut Environment) -> Result<(), String> {
    build_io_env(env)?;
    build_io_throw(env)?;
    build_io_catch(env)?;
    build_io_getenv(env)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn setup_env() -> Environment {
        let mut env = Environment::new();
        let type1 = Expr::Sort(Level::succ(Level::zero()));
        env.add(Declaration::Axiom {
            name: Name::str("String"),
            univ_params: vec![],
            ty: type1.clone(),
        })
        .expect("operation should succeed");
        env.add(Declaration::Axiom {
            name: Name::str("Unit"),
            univ_params: vec![],
            ty: type1,
        })
        .expect("operation should succeed");
        env
    }
    #[test]
    fn test_build_io_env() {
        let mut env = setup_env();
        assert!(build_io_env(&mut env).is_ok());
        assert!(env.get(&Name::str("IO")).is_some());
        assert!(env.get(&Name::str("IO.pure")).is_some());
        assert!(env.get(&Name::str("IO.bind")).is_some());
    }
    #[test]
    fn test_io_println() {
        let mut env = setup_env();
        build_io_env(&mut env).expect("build_io_env should succeed");
        let decl = env
            .get(&Name::str("IO.println"))
            .expect("declaration 'IO.println' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_io_readline() {
        let mut env = setup_env();
        build_io_env(&mut env).expect("build_io_env should succeed");
        let decl = env
            .get(&Name::str("IO.readLine"))
            .expect("declaration 'IO.readLine' should exist in env");
        assert!(matches!(decl, Declaration::Axiom { .. }));
    }
    #[test]
    fn test_buffered_reader_read_line() {
        let mut reader = BufferedReader::from_str("hello\nworld\n");
        assert_eq!(reader.read_line(), Some("hello\n".to_string()));
        assert_eq!(reader.read_line(), Some("world\n".to_string()));
        assert_eq!(reader.read_line(), None);
    }
    #[test]
    fn test_buffered_reader_read_exact() {
        let mut reader = BufferedReader::from_str("abcdef");
        let chunk = reader.read_exact(3).expect("read_exact should succeed");
        assert_eq!(chunk, b"abc");
        assert_eq!(reader.remaining(), 3);
    }
    #[test]
    fn test_buffered_reader_eof() {
        let mut reader = BufferedReader::from_str("x");
        assert!(!reader.is_eof());
        reader.read_byte();
        assert!(reader.is_eof());
    }
    #[test]
    fn test_buffered_reader_lines() {
        let mut reader = BufferedReader::from_str("a\nb\nc");
        let lines = reader.lines();
        assert_eq!(lines.len(), 3);
    }
    #[test]
    fn test_buffered_reader_peek() {
        let reader = BufferedReader::from_str("hello");
        assert_eq!(reader.peek(), Some(b'h'));
    }
    #[test]
    fn test_buffered_reader_skip() {
        let mut reader = BufferedReader::from_str("abcdef");
        reader.skip(3);
        assert_eq!(reader.peek(), Some(b'd'));
    }
    #[test]
    fn test_buffered_reader_reset() {
        let mut reader = BufferedReader::from_str("abc");
        reader.read_byte();
        reader.reset();
        assert_eq!(reader.peek(), Some(b'a'));
    }
    #[test]
    fn test_buffered_writer_writeln() {
        let mut writer = BufferedWriter::new(1024);
        writer.writeln("hello");
        assert_eq!(writer.buffered_len(), 6);
        assert!(!writer.is_empty());
    }
    #[test]
    fn test_buffered_writer_total_written() {
        let mut writer = BufferedWriter::new(10);
        writer.write_str("hello");
        writer.write_str("world");
        assert_eq!(writer.total_written(), 10);
    }
    #[test]
    fn test_buffered_writer_auto_flush() {
        let mut writer = BufferedWriter::new(5);
        writer.write_str("hello world");
        assert!(writer.is_empty());
    }
    #[test]
    fn test_io_error_display() {
        let err = IoError::not_found("foo.lean");
        let msg = format!("{}", err);
        assert!(msg.contains("not found"));
    }
    #[test]
    fn test_io_error_kinds() {
        assert_eq!(
            format!("{}", IoErrorKind::PermissionDenied),
            "permission denied"
        );
        assert_eq!(format!("{}", IoErrorKind::TimedOut), "timed out");
        assert_eq!(
            format!("{}", IoErrorKind::UnexpectedEof),
            "unexpected end of file"
        );
    }
    #[test]
    fn test_file_metadata_regular() {
        let meta = FileMetadata::regular_file("/tmp/foo.lean", 1024);
        assert!(meta.is_file);
        assert!(!meta.is_dir);
        assert_eq!(meta.size, 1024);
    }
    #[test]
    fn test_file_metadata_dir() {
        let meta = FileMetadata::directory("/tmp/mydir");
        assert!(meta.is_dir);
        assert!(!meta.is_file);
    }
    #[test]
    fn test_file_metadata_readonly() {
        let meta = FileMetadata::regular_file("/etc/passwd", 2048).with_read_only();
        assert!(meta.read_only);
    }
    #[test]
    fn test_io_action_pipeline() {
        let mut pipeline = IoActionPipeline::new();
        pipeline.push(IoAction::println());
        pipeline.push(IoAction::read_line());
        assert_eq!(pipeline.len(), 2);
        assert!(!pipeline.has_exit());
    }
    #[test]
    fn test_io_action_pipeline_exit() {
        let mut pipeline = IoActionPipeline::new();
        pipeline.push(IoAction::exit(0));
        assert!(pipeline.has_exit());
    }
    #[test]
    fn test_io_action_pipeline_result_type() {
        let mut pipeline = IoActionPipeline::new();
        assert!(pipeline.result_type().is_none());
        pipeline.push(IoAction::println());
        assert!(pipeline.result_type().is_some());
    }
    #[test]
    fn test_build_io_throw() {
        let mut env = setup_env();
        build_io_env(&mut env).expect("build_io_env should succeed");
        assert!(build_io_throw(&mut env).is_ok());
        assert!(env.get(&Name::str("IO.throw")).is_some());
    }
    #[test]
    fn test_build_io_catch() {
        let mut env = setup_env();
        build_io_env(&mut env).expect("build_io_env should succeed");
        assert!(build_io_catch(&mut env).is_ok());
        assert!(env.get(&Name::str("IO.catch")).is_some());
    }
    #[test]
    fn test_build_io_extended() {
        let mut env = setup_env();
        assert!(build_io_extended(&mut env).is_ok());
        assert!(env.get(&Name::str("IO.throw")).is_some());
        assert!(env.get(&Name::str("IO.catch")).is_some());
        assert!(env.get(&Name::str("IO.getEnv")).is_some());
    }
    #[test]
    fn test_read_exact_eof() {
        let mut reader = BufferedReader::from_str("ab");
        assert!(reader.read_exact(5).is_err());
    }
}
#[cfg(test)]
mod extra_io_tests {
    use super::*;
    #[test]
    fn test_mock_fs_write_read() {
        let mut fs = MockFs::new();
        fs.write_str("hello.lean", "theorem foo : True := trivial");
        let content = fs.read_str("hello.lean").expect("read_str should succeed");
        assert!(content.contains("theorem"));
    }
    #[test]
    fn test_mock_fs_not_found() {
        let fs = MockFs::new();
        assert!(fs.read("nonexistent.lean").is_err());
    }
    #[test]
    fn test_mock_fs_exists() {
        let mut fs = MockFs::new();
        assert!(!fs.exists("x.lean"));
        fs.write_str("x.lean", "-- empty");
        assert!(fs.exists("x.lean"));
    }
    #[test]
    fn test_mock_fs_remove() {
        let mut fs = MockFs::new();
        fs.write_str("x.lean", "");
        assert!(fs.remove("x.lean"));
        assert!(!fs.remove("x.lean"));
    }
    #[test]
    fn test_mock_fs_list_paths() {
        let mut fs = MockFs::new();
        fs.write_str("a.lean", "");
        fs.write_str("b.lean", "");
        let paths = fs.list_paths();
        assert_eq!(paths.len(), 2);
    }
    #[test]
    fn test_mock_fs_file_size() {
        let mut fs = MockFs::new();
        fs.write_str("test.lean", "hello");
        assert_eq!(
            fs.file_size("test.lean").expect("file_size should succeed"),
            5
        );
    }
    #[test]
    fn test_record_writer() {
        let mut rw = RecordWriter::new(1024);
        rw.write_record("record1");
        rw.write_record("record2");
        assert_eq!(rw.record_count(), 2);
        assert_eq!(rw.total_written(), "record1\n".len() + "record2\n".len());
    }
    #[test]
    fn test_delimiter_splitter_feed_and_drain() {
        let mut splitter = DelimiterSplitter::new(b'\n');
        splitter.feed(b"hello\nworld\n");
        let records = splitter.drain();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0], b"hello");
        assert_eq!(records[1], b"world");
    }
    #[test]
    fn test_delimiter_splitter_partial() {
        let mut splitter = DelimiterSplitter::new(b'\n');
        splitter.feed(b"hello");
        let records = splitter.drain();
        assert!(records.is_empty());
        assert_eq!(splitter.buffered_len(), 5);
    }
    #[test]
    fn test_delimiter_splitter_multiple_feeds() {
        let mut splitter = DelimiterSplitter::new(b'\n');
        splitter.feed(b"hel");
        splitter.feed(b"lo\n");
        let records = splitter.drain();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0], b"hello");
    }
    #[test]
    fn test_env_registry_set_get() {
        let mut reg = EnvRegistry::new();
        reg.set("HOME", "/home/user");
        assert_eq!(reg.get("HOME"), Some("/home/user"));
        assert_eq!(reg.get("MISSING"), None);
    }
    #[test]
    fn test_env_registry_remove() {
        let mut reg = EnvRegistry::new();
        reg.set("X", "1");
        reg.remove("X");
        assert_eq!(reg.get("X"), None);
    }
    #[test]
    fn test_env_registry_keys() {
        let mut reg = EnvRegistry::new();
        reg.set("A", "1");
        reg.set("B", "2");
        let keys = reg.keys();
        assert_eq!(keys.len(), 2);
    }
    #[test]
    fn test_mock_fs_overwrite() {
        let mut fs = MockFs::new();
        fs.write_str("x.lean", "v1");
        fs.write_str("x.lean", "v2");
        assert_eq!(
            fs.read_str("x.lean").expect("read_str should succeed"),
            "v2"
        );
    }
    #[test]
    fn test_record_writer_flush() {
        let mut rw = RecordWriter::new(1024);
        rw.write_record("x");
        rw.flush();
        assert_eq!(rw.record_count(), 1);
    }
}
pub fn io_ext_type1() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn io_ext_prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn io_of(alpha: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("IO"), vec![])),
        Box::new(alpha),
    )
}
pub fn io_ext_unit() -> Expr {
    Expr::Const(Name::str("Unit"), vec![])
}
pub fn io_ext_nat() -> Expr {
    Expr::Const(Name::str("Nat"), vec![])
}
pub fn io_ext_bool() -> Expr {
    Expr::Const(Name::str("Bool"), vec![])
}
pub fn io_ext_string() -> Expr {
    Expr::Const(Name::str("String"), vec![])
}
pub fn io_ext_list(alpha: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("List"), vec![])),
        Box::new(alpha),
    )
}
pub fn io_ext_option(alpha: Expr) -> Expr {
    Expr::App(
        Box::new(Expr::Const(Name::str("Option"), vec![])),
        Box::new(alpha),
    )
}
pub fn io_ext_alpha_implicit(inner: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(io_ext_type1()),
        Box::new(inner),
    )
}
pub fn io_ext_ab_implicit(inner: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(io_ext_type1()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(io_ext_type1()),
            Box::new(inner),
        )),
    )
}
pub fn io_ext_abc_implicit(inner: Expr) -> Expr {
    Expr::Pi(
        BinderInfo::Implicit,
        Name::str("α"),
        Box::new(io_ext_type1()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("β"),
            Box::new(io_ext_type1()),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("γ"),
                Box::new(io_ext_type1()),
                Box::new(inner),
            )),
        )),
    )
}
pub fn io_axiom(env: &mut Environment, name: &str, ty: Expr) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str(name),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// `IO.denote : {α : Type} → IO α → (World → α × World)`
///
/// Denotational semantics: IO programs as state transformers over the world.
pub fn axiom_io_denote_ty() -> Expr {
    let world_fn = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(io_ext_nat()),
        Box::new(Expr::App(
            Box::new(Expr::App(
                Box::new(Expr::Const(Name::str("Prod"), vec![])),
                Box::new(Expr::BVar(1)),
            )),
            Box::new(io_ext_nat()),
        )),
    );
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(io_of(Expr::BVar(0))),
        Box::new(world_fn),
    ))
}
/// `IO.pure_denote : {α : Type} → ∀ x : α, ∀ w : World, denote (pure x) w = (x, w)`
///
/// Denotational correctness of pure: does not change world state.
pub fn axiom_io_pure_denote_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(Expr::BVar(0)),
        Box::new(io_ext_prop()),
    ))
}
/// `IO.bind_denote : {α β : Type} → ∀ m f w, denote (bind m f) w = let (a,w') := denote m w in denote (f a) w'`
///
/// Denotational correctness of bind: threads world state sequentially.
pub fn axiom_io_bind_denote_ty() -> Expr {
    io_ext_ab_implicit(io_ext_prop())
}
/// `IO.free_monad_inl : {α : Type} → α → IO α`
///
/// Free monad injection: lift a pure value into the free monad over IO operations.
pub fn axiom_io_free_monad_inl_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("x"),
        Box::new(Expr::BVar(0)),
        Box::new(io_of(Expr::BVar(1))),
    ))
}
/// `IO.free_monad_inr : {α : Type} → (Nat → IO α) → IO α`
///
/// Free monad injection: lift an operation into the free monad.
pub fn axiom_io_free_monad_inr_ty() -> Expr {
    let cont = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(io_ext_nat()),
        Box::new(io_of(Expr::BVar(1))),
    );
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("op"),
        Box::new(cont),
        Box::new(io_of(Expr::BVar(1))),
    ))
}
/// `IO.hoare_pre : {α : Type} → (World → Prop) → IO α → (α → World → Prop) → Prop`
///
/// Hoare triple for IO: `{P} m {Q}` means if P holds before, Q holds after.
pub fn axiom_io_hoare_pre_ty() -> Expr {
    let pre = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(io_ext_nat()),
        Box::new(io_ext_prop()),
    );
    let post = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(2)),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(io_ext_nat()),
            Box::new(io_ext_prop()),
        )),
    );
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("P"),
        Box::new(pre),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("m"),
            Box::new(io_of(Expr::BVar(1))),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("Q"),
                Box::new(post),
                Box::new(io_ext_prop()),
            )),
        )),
    ))
}
/// `IO.hoare_pure : {α : Type} → ∀ x P, {P x} pure x {P}`
///
/// Hoare rule for pure: trivially satisfies any postcondition.
pub fn axiom_io_hoare_pure_ty() -> Expr {
    io_ext_alpha_implicit(io_ext_prop())
}
/// `IO.hoare_bind : {α β : Type} → ∀ m f P Q R, {P} m {Q} → (∀ x, {Q x} f x {R}) → {P} bind m f {R}`
///
/// Hoare sequential composition rule.
pub fn axiom_io_hoare_bind_ty() -> Expr {
    io_ext_ab_implicit(io_ext_prop())
}
/// `IO.sep_star : {α : Type} → (Heap → Prop) → (Heap → Prop) → Heap → Prop`
///
/// Separation logic star (∗): heap splits into two disjoint parts.
pub fn axiom_io_sep_star_ty() -> Expr {
    let pred = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(io_ext_nat()),
        Box::new(io_ext_prop()),
    );
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("P"),
        Box::new(pred.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("Q"),
            Box::new(pred),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("h"),
                Box::new(io_ext_nat()),
                Box::new(io_ext_prop()),
            )),
        )),
    ))
}
/// `IO.sep_frame : {α : Type} → ∀ m P Q R, {P} m {Q} → {P ∗ R} m {Q ∗ R}`
///
/// Separation logic frame rule for IO programs.
pub fn axiom_io_sep_frame_ty() -> Expr {
    io_ext_alpha_implicit(io_ext_prop())
}
/// `IO.conc_par : {α β : Type} → IO α → IO β → IO (α × β)`
///
/// Concurrent IO: run two IO actions in parallel.
pub fn axiom_io_conc_par_ty() -> Expr {
    let prod_ty = Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Prod"), vec![])),
            Box::new(Expr::BVar(1)),
        )),
        Box::new(Expr::BVar(0)),
    );
    io_ext_ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("m1"),
        Box::new(io_of(Expr::BVar(1))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("m2"),
            Box::new(io_of(Expr::BVar(1))),
            Box::new(io_of(prod_ty)),
        )),
    ))
}
/// `IO.conc_race : {α : Type} → IO α → IO α → IO α`
///
/// Concurrent IO race: return the first result to complete.
pub fn axiom_io_conc_race_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("m1"),
        Box::new(io_of(Expr::BVar(0))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("m2"),
            Box::new(io_of(Expr::BVar(1))),
            Box::new(io_of(Expr::BVar(2))),
        )),
    ))
}
/// `IO.stm_atomically : {α : Type} → IO α → IO α`
///
/// STM (Software Transactional Memory): execute an IO action atomically.
pub fn axiom_io_stm_atomically_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("tx"),
        Box::new(io_of(Expr::BVar(0))),
        Box::new(io_of(Expr::BVar(1))),
    ))
}
/// `IO.stm_retry : {α : Type} → IO α`
///
/// STM retry: abort the current transaction and try again.
pub fn axiom_io_stm_retry_ty() -> Expr {
    io_ext_alpha_implicit(io_of(Expr::BVar(0)))
}
/// `IO.stm_orElse : {α : Type} → IO α → IO α → IO α`
///
/// STM alternative: try the first transaction, fall back to the second on retry.
pub fn axiom_io_stm_or_else_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("tx1"),
        Box::new(io_of(Expr::BVar(0))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("tx2"),
            Box::new(io_of(Expr::BVar(1))),
            Box::new(io_of(Expr::BVar(2))),
        )),
    ))
}
/// `IO.ioRef_new : {α : Type} → α → IO (IORef α)`
///
/// Allocate a new mutable reference cell.
pub fn axiom_io_ioref_new_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("init"),
        Box::new(Expr::BVar(0)),
        Box::new(io_of(Expr::App(
            Box::new(Expr::Const(Name::str("IORef"), vec![])),
            Box::new(Expr::BVar(1)),
        ))),
    ))
}
/// `IO.ioRef_read : {α : Type} → IORef α → IO α`
///
/// Read the current value of an IORef.
pub fn axiom_io_ioref_read_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("r"),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("IORef"), vec![])),
            Box::new(Expr::BVar(0)),
        )),
        Box::new(io_of(Expr::BVar(1))),
    ))
}
/// `IO.ioRef_write : {α : Type} → IORef α → α → IO Unit`
///
/// Write a new value to an IORef.
pub fn axiom_io_ioref_write_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("r"),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("IORef"), vec![])),
            Box::new(Expr::BVar(0)),
        )),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("v"),
            Box::new(Expr::BVar(1)),
            Box::new(io_of(io_ext_unit())),
        )),
    ))
}
/// `IO.ioRef_modify : {α : Type} → IORef α → (α → α) → IO Unit`
///
/// Atomically modify the value of an IORef.
pub fn axiom_io_ioref_modify_ty() -> Expr {
    let fn_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(0)),
        Box::new(Expr::BVar(1)),
    );
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("r"),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("IORef"), vec![])),
            Box::new(Expr::BVar(0)),
        )),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("f"),
            Box::new(fn_ty),
            Box::new(io_of(io_ext_unit())),
        )),
    ))
}
/// `IO.mvar_new : {α : Type} → IO (MVar α)`
///
/// Allocate a new empty MVar.
pub fn axiom_io_mvar_new_ty() -> Expr {
    io_ext_alpha_implicit(io_of(Expr::App(
        Box::new(Expr::Const(Name::str("MVar"), vec![])),
        Box::new(Expr::BVar(0)),
    )))
}
/// `IO.mvar_take : {α : Type} → MVar α → IO α`
///
/// Take the value from an MVar, blocking if empty.
pub fn axiom_io_mvar_take_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("mv"),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("MVar"), vec![])),
            Box::new(Expr::BVar(0)),
        )),
        Box::new(io_of(Expr::BVar(1))),
    ))
}
/// `IO.mvar_put : {α : Type} → MVar α → α → IO Unit`
///
/// Put a value into an MVar, blocking if full.
pub fn axiom_io_mvar_put_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("mv"),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("MVar"), vec![])),
            Box::new(Expr::BVar(0)),
        )),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("v"),
            Box::new(Expr::BVar(1)),
            Box::new(io_of(io_ext_unit())),
        )),
    ))
}
/// `IO.fd_open : String → IO Nat`
///
/// Open a file descriptor by path, returning a handle (modelled as Nat).
pub fn axiom_io_fd_open_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("path"),
        Box::new(io_ext_string()),
        Box::new(io_of(io_ext_nat())),
    )
}
/// `IO.fd_close : Nat → IO Unit`
///
/// Close a file descriptor.
pub fn axiom_io_fd_close_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("fd"),
        Box::new(io_ext_nat()),
        Box::new(io_of(io_ext_unit())),
    )
}
/// `IO.fd_read : Nat → Nat → IO (List Nat)`
///
/// Read up to `n` bytes from file descriptor `fd`.
pub fn axiom_io_fd_read_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("fd"),
        Box::new(io_ext_nat()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("n"),
            Box::new(io_ext_nat()),
            Box::new(io_of(io_ext_list(io_ext_nat()))),
        )),
    )
}
/// `IO.fd_write : Nat → List Nat → IO Nat`
///
/// Write bytes to file descriptor `fd`, returning bytes written.
pub fn axiom_io_fd_write_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("fd"),
        Box::new(io_ext_nat()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("data"),
            Box::new(io_ext_list(io_ext_nat())),
            Box::new(io_of(io_ext_nat())),
        )),
    )
}
/// `IO.async_spawn : {α : Type} → IO α → IO (Promise α)`
///
/// Asynchronous IO: spawn a concurrent task, returning a promise.
pub fn axiom_io_async_spawn_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(io_of(Expr::BVar(0))),
        Box::new(io_of(Expr::App(
            Box::new(Expr::Const(Name::str("Promise"), vec![])),
            Box::new(Expr::BVar(1)),
        ))),
    ))
}
/// `IO.async_await : {α : Type} → Promise α → IO α`
///
/// Asynchronous IO: await the result of a promise.
pub fn axiom_io_async_await_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("p"),
        Box::new(Expr::App(
            Box::new(Expr::Const(Name::str("Promise"), vec![])),
            Box::new(Expr::BVar(0)),
        )),
        Box::new(io_of(Expr::BVar(1))),
    ))
}
/// `IO.effect_send : {α : Type} → Nat → α → IO Unit`
///
/// Algebraic effects: send an effect labelled by `Nat` with payload `α`.
pub fn axiom_io_effect_send_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("label"),
        Box::new(io_ext_nat()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("payload"),
            Box::new(Expr::BVar(1)),
            Box::new(io_of(io_ext_unit())),
        )),
    ))
}
/// `IO.effect_handle : {α β : Type} → IO α → (Nat → β → IO α) → IO α`
///
/// Algebraic effect handler: intercept effects and provide a handler.
pub fn axiom_io_effect_handle_ty() -> Expr {
    let handler_ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("lbl"),
        Box::new(io_ext_nat()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("payload"),
            Box::new(Expr::BVar(2)),
            Box::new(io_of(Expr::BVar(3))),
        )),
    );
    io_ext_ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(io_of(Expr::BVar(1))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("h"),
            Box::new(handler_ty),
            Box::new(io_of(Expr::BVar(3))),
        )),
    ))
}
/// `IO.capability_restrict : {α : Type} → Nat → IO α → IO α`
///
/// Capability-based IO security: restrict an IO action to a given capability level.
pub fn axiom_io_capability_restrict_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("cap"),
        Box::new(io_ext_nat()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("m"),
            Box::new(io_of(Expr::BVar(1))),
            Box::new(io_of(Expr::BVar(2))),
        )),
    ))
}
/// `IO.linear_use : {α : Type} → IO α → IO α`
///
/// Linear types for IO: ensure a resource is used exactly once.
pub fn axiom_io_linear_use_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(io_of(Expr::BVar(0))),
        Box::new(io_of(Expr::BVar(1))),
    ))
}
/// `IO.session_send : {α β : Type} → α → IO β → IO β`
///
/// Session types for IO channels: send a value along a session channel.
pub fn axiom_io_session_send_ty() -> Expr {
    io_ext_ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("v"),
        Box::new(Expr::BVar(1)),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("cont"),
            Box::new(io_of(Expr::BVar(1))),
            Box::new(io_of(Expr::BVar(2))),
        )),
    ))
}
/// `IO.session_recv : {α β : Type} → (α → IO β) → IO β`
///
/// Session types for IO channels: receive a value along a session channel.
pub fn axiom_io_session_recv_ty() -> Expr {
    let cont = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(1)),
        Box::new(io_of(Expr::BVar(1))),
    );
    io_ext_ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("k"),
        Box::new(cont),
        Box::new(io_of(Expr::BVar(1))),
    ))
}
/// `IO.monad_left_id : {α β : Type} → ∀ x f, bind (pure x) f = f x`
///
/// IO monad left identity law.
pub fn axiom_io_monad_left_id_ty() -> Expr {
    io_ext_ab_implicit(io_ext_prop())
}
/// `IO.monad_right_id : {α : Type} → ∀ m, bind m pure = m`
///
/// IO monad right identity law.
pub fn axiom_io_monad_right_id_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(io_of(Expr::BVar(0))),
        Box::new(io_ext_prop()),
    ))
}
/// `IO.monad_assoc : {α β γ : Type} → ∀ m f g, bind (bind m f) g = bind m (fun x => bind (f x) g)`
///
/// IO monad associativity law.
pub fn axiom_io_monad_assoc_ty() -> Expr {
    io_ext_abc_implicit(io_ext_prop())
}
/// `IO.map_pure : {α β : Type} → ∀ f x, map f (pure x) = pure (f x)`
///
/// Functor homomorphism for IO.
pub fn axiom_io_map_pure_ty() -> Expr {
    io_ext_ab_implicit(io_ext_prop())
}
/// `IO.map_id : {α : Type} → ∀ m, map id m = m`
///
/// Functor identity for IO.
pub fn axiom_io_map_id_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(io_of(Expr::BVar(0))),
        Box::new(io_ext_prop()),
    ))
}
/// `IO.map_comp : {α β γ : Type} → ∀ f g m, map (f ∘ g) m = map f (map g m)`
///
/// Functor composition for IO.
pub fn axiom_io_map_comp_ty() -> Expr {
    io_ext_abc_implicit(io_ext_prop())
}
/// `IO.ap_pure : {α β : Type} → ∀ f x, ap (pure f) (pure x) = pure (f x)`
///
/// Applicative homomorphism for IO.
pub fn axiom_io_ap_pure_ty() -> Expr {
    io_ext_ab_implicit(io_ext_prop())
}
/// `IO.getLine : IO String`
///
/// Read a line of text from standard input.
pub fn axiom_io_getline_ty() -> Expr {
    io_of(io_ext_string())
}
/// `IO.putStr : String → IO Unit`
///
/// Write a string to standard output without newline.
pub fn axiom_io_putstr_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(io_ext_string()),
        Box::new(io_of(io_ext_unit())),
    )
}
/// `IO.sleep_ms : Nat → IO Unit`
///
/// Sleep for the given number of milliseconds.
pub fn axiom_io_sleep_ms_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("ms"),
        Box::new(io_ext_nat()),
        Box::new(io_of(io_ext_unit())),
    )
}
/// `IO.exit_code : Nat → IO Unit`
///
/// Terminate the program with the given exit code.
pub fn axiom_io_exit_code_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("code"),
        Box::new(io_ext_nat()),
        Box::new(io_of(io_ext_unit())),
    )
}
/// `IO.stderr_write : String → IO Unit`
///
/// Write a string to standard error.
pub fn axiom_io_stderr_write_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("s"),
        Box::new(io_ext_string()),
        Box::new(io_of(io_ext_unit())),
    )
}
/// `IO.fork : {α : Type} → IO α → IO Nat`
///
/// Fork an IO action in a new thread, returning a thread identifier.
pub fn axiom_io_fork_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(io_of(Expr::BVar(0))),
        Box::new(io_of(io_ext_nat())),
    ))
}
/// `IO.join_thread : Nat → IO Unit`
///
/// Wait for a thread identified by its handle to complete.
pub fn axiom_io_join_thread_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("tid"),
        Box::new(io_ext_nat()),
        Box::new(io_of(io_ext_unit())),
    )
}
/// `IO.timeout : {α : Type} → Nat → IO α → IO (Option α)`
///
/// Execute an IO action with a timeout; returns None if time exceeded.
pub fn axiom_io_timeout_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("ms"),
        Box::new(io_ext_nat()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("m"),
            Box::new(io_of(Expr::BVar(1))),
            Box::new(io_of(io_ext_option(Expr::BVar(2)))),
        )),
    ))
}
/// `IO.retry : {α : Type} → Nat → IO α → IO (Option α)`
///
/// Retry an IO action up to `n` times.
pub fn axiom_io_retry_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("n"),
        Box::new(io_ext_nat()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("m"),
            Box::new(io_of(Expr::BVar(1))),
            Box::new(io_of(io_ext_option(Expr::BVar(2)))),
        )),
    ))
}
/// `IO.bracket : {α β : Type} → IO α → (α → IO Unit) → (α → IO β) → IO β`
///
/// Resource bracket: acquire, use, and release a resource safely.
pub fn axiom_io_bracket_ty() -> Expr {
    let release = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(1)),
        Box::new(io_of(io_ext_unit())),
    );
    let use_fn = Expr::Pi(
        BinderInfo::Default,
        Name::str("_"),
        Box::new(Expr::BVar(1)),
        Box::new(io_of(Expr::BVar(1))),
    );
    io_ext_ab_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("acquire"),
        Box::new(io_of(Expr::BVar(1))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("release"),
            Box::new(release),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("use_fn"),
                Box::new(use_fn),
                Box::new(io_of(Expr::BVar(3))),
            )),
        )),
    ))
}
/// `IO.finally : {α : Type} → IO α → IO Unit → IO α`
///
/// Execute a cleanup action regardless of whether the main action succeeds.
pub fn axiom_io_finally_ty() -> Expr {
    io_ext_alpha_implicit(Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(io_of(Expr::BVar(0))),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("cleanup"),
            Box::new(io_of(io_ext_unit())),
            Box::new(io_of(Expr::BVar(2))),
        )),
    ))
}
/// `IO.read_file : String → IO String`
///
/// Read the entire contents of a file as a string.
pub fn axiom_io_read_file_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("path"),
        Box::new(io_ext_string()),
        Box::new(io_of(io_ext_string())),
    )
}
/// `IO.write_file : String → String → IO Unit`
///
/// Write a string to a file, overwriting if it exists.
pub fn axiom_io_write_file_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("path"),
        Box::new(io_ext_string()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("content"),
            Box::new(io_ext_string()),
            Box::new(io_of(io_ext_unit())),
        )),
    )
}
/// `IO.append_file : String → String → IO Unit`
///
/// Append a string to a file.
pub fn axiom_io_append_file_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("path"),
        Box::new(io_ext_string()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("content"),
            Box::new(io_ext_string()),
            Box::new(io_of(io_ext_unit())),
        )),
    )
}
/// `IO.file_exists : String → IO Bool`
///
/// Check whether a file exists.
pub fn axiom_io_file_exists_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("path"),
        Box::new(io_ext_string()),
        Box::new(io_of(io_ext_bool())),
    )
}
/// `IO.list_dir : String → IO (List String)`
///
/// List the entries of a directory.
pub fn axiom_io_list_dir_ty() -> Expr {
    Expr::Pi(
        BinderInfo::Default,
        Name::str("path"),
        Box::new(io_ext_string()),
        Box::new(io_of(io_ext_list(io_ext_string()))),
    )
}
/// Register all extended IO axioms into the given environment.
pub fn register_io_extended(env: &mut Environment) -> Result<(), String> {
    let entries: &[(&str, fn() -> Expr)] = &[
        ("IO.denote", axiom_io_denote_ty),
        ("IO.pure_denote", axiom_io_pure_denote_ty),
        ("IO.bind_denote", axiom_io_bind_denote_ty),
        ("IO.free_monad_inl", axiom_io_free_monad_inl_ty),
        ("IO.free_monad_inr", axiom_io_free_monad_inr_ty),
        ("IO.hoare_pre", axiom_io_hoare_pre_ty),
        ("IO.hoare_pure", axiom_io_hoare_pure_ty),
        ("IO.hoare_bind", axiom_io_hoare_bind_ty),
        ("IO.sep_star", axiom_io_sep_star_ty),
        ("IO.sep_frame", axiom_io_sep_frame_ty),
        ("IO.conc_par", axiom_io_conc_par_ty),
        ("IO.conc_race", axiom_io_conc_race_ty),
        ("IO.stm_atomically", axiom_io_stm_atomically_ty),
        ("IO.stm_retry", axiom_io_stm_retry_ty),
        ("IO.stm_orElse", axiom_io_stm_or_else_ty),
        ("IO.ioRef_new", axiom_io_ioref_new_ty),
        ("IO.ioRef_read", axiom_io_ioref_read_ty),
        ("IO.ioRef_write", axiom_io_ioref_write_ty),
        ("IO.ioRef_modify", axiom_io_ioref_modify_ty),
        ("IO.mvar_new", axiom_io_mvar_new_ty),
        ("IO.mvar_take", axiom_io_mvar_take_ty),
        ("IO.mvar_put", axiom_io_mvar_put_ty),
        ("IO.fd_open", axiom_io_fd_open_ty),
        ("IO.fd_close", axiom_io_fd_close_ty),
        ("IO.fd_read", axiom_io_fd_read_ty),
        ("IO.fd_write", axiom_io_fd_write_ty),
        ("IO.async_spawn", axiom_io_async_spawn_ty),
        ("IO.async_await", axiom_io_async_await_ty),
        ("IO.effect_send", axiom_io_effect_send_ty),
        ("IO.effect_handle", axiom_io_effect_handle_ty),
        ("IO.capability_restrict", axiom_io_capability_restrict_ty),
        ("IO.linear_use", axiom_io_linear_use_ty),
        ("IO.session_send", axiom_io_session_send_ty),
        ("IO.session_recv", axiom_io_session_recv_ty),
        ("IO.monad_left_id", axiom_io_monad_left_id_ty),
        ("IO.monad_right_id", axiom_io_monad_right_id_ty),
        ("IO.monad_assoc", axiom_io_monad_assoc_ty),
        ("IO.map_pure", axiom_io_map_pure_ty),
        ("IO.map_id", axiom_io_map_id_ty),
        ("IO.map_comp", axiom_io_map_comp_ty),
        ("IO.ap_pure", axiom_io_ap_pure_ty),
        ("IO.getLine", axiom_io_getline_ty),
        ("IO.putStr", axiom_io_putstr_ty),
        ("IO.sleep_ms", axiom_io_sleep_ms_ty),
        ("IO.exit_code", axiom_io_exit_code_ty),
        ("IO.stderr_write", axiom_io_stderr_write_ty),
        ("IO.fork", axiom_io_fork_ty),
        ("IO.join_thread", axiom_io_join_thread_ty),
        ("IO.timeout", axiom_io_timeout_ty),
        ("IO.retry", axiom_io_retry_ty),
        ("IO.bracket", axiom_io_bracket_ty),
        ("IO.finally", axiom_io_finally_ty),
        ("IO.read_file", axiom_io_read_file_ty),
        ("IO.write_file", axiom_io_write_file_ty),
        ("IO.append_file", axiom_io_append_file_ty),
        ("IO.file_exists", axiom_io_file_exists_ty),
        ("IO.list_dir", axiom_io_list_dir_ty),
    ];
    for (name, ty_fn) in entries {
        io_axiom(env, name, ty_fn())?;
    }
    Ok(())
}
#[cfg(test)]
mod io_extended_tests {
    use super::*;
    fn io_base_env() -> Environment {
        let mut env = Environment::new();
        let t1 = Expr::Sort(Level::succ(Level::zero()));
        for name in [
            "String", "Unit", "Bool", "Nat", "List", "Option", "Prod", "IORef", "MVar", "Promise",
        ] {
            env.add(Declaration::Axiom {
                name: Name::str(name),
                univ_params: vec![],
                ty: t1.clone(),
            })
            .unwrap_or(());
        }
        build_io_env(&mut env).expect("build_io_env should succeed");
        env
    }
    #[test]
    fn test_register_io_extended_all_registered() {
        let mut env = io_base_env();
        let result = register_io_extended(&mut env);
        assert!(result.is_ok(), "register_io_extended failed: {:?}", result);
        assert!(env.get(&Name::str("IO.denote")).is_some());
        assert!(env.get(&Name::str("IO.hoare_pre")).is_some());
        assert!(env.get(&Name::str("IO.stm_atomically")).is_some());
        assert!(env.get(&Name::str("IO.ioRef_new")).is_some());
        assert!(env.get(&Name::str("IO.async_spawn")).is_some());
        assert!(env.get(&Name::str("IO.effect_handle")).is_some());
        assert!(env.get(&Name::str("IO.session_send")).is_some());
        assert!(env.get(&Name::str("IO.bracket")).is_some());
        assert!(env.get(&Name::str("IO.list_dir")).is_some());
    }
    #[test]
    fn test_hoare_verifier_add_triple() {
        let mut hv = HoareVerifier::new();
        hv.add_triple("IO.println", "true", "true");
        assert_eq!(hv.triple_count(), 1);
        assert!(hv.has_triple("IO.println"));
        assert_eq!(hv.postcondition_of("IO.println"), Some("true"));
    }
    #[test]
    fn test_hoare_verifier_operations() {
        let mut hv = HoareVerifier::new();
        hv.add_triple("op1", "P", "Q");
        hv.add_triple("op2", "R", "S");
        let ops = hv.operations();
        assert_eq!(ops.len(), 2);
    }
    #[test]
    fn test_stm_log_record() {
        let mut log = StmLog::new();
        log.record_read(0, 42);
        log.record_write(1, 99);
        assert_eq!(log.read_count(), 1);
        assert_eq!(log.write_count(), 1);
        assert!(!log.is_aborted());
    }
    #[test]
    fn test_stm_log_abort() {
        let mut log = StmLog::new();
        log.abort();
        assert!(log.is_aborted());
    }
    #[test]
    fn test_stm_log_conflict() {
        let mut log1 = StmLog::new();
        let mut log2 = StmLog::new();
        log1.record_write(5, 1);
        log2.record_write(5, 2);
        assert!(log1.conflicts_with(&log2));
    }
    #[test]
    fn test_stm_log_no_conflict() {
        let mut log1 = StmLog::new();
        let mut log2 = StmLog::new();
        log1.record_write(3, 1);
        log2.record_write(7, 2);
        assert!(!log1.conflicts_with(&log2));
    }
    #[test]
    fn test_async_task_queue_enqueue_complete() {
        let mut q = AsyncTaskQueue::new();
        let id = q.enqueue("fetch file");
        assert_eq!(q.pending_count(), 1);
        q.complete_next("file contents");
        assert_eq!(q.completed_count(), 1);
        assert!(q.is_complete(id));
        assert_eq!(q.result_of(id), Some("file contents"));
    }
    #[test]
    fn test_async_task_queue_empty_complete() {
        let mut q = AsyncTaskQueue::new();
        let result = q.complete_next("nothing");
        assert!(result.is_none());
    }
    #[test]
    fn test_session_channel_send_recv() {
        let mut ch = SessionChannel::new(vec![SessionAction::Send, SessionAction::Recv]);
        assert!(ch.send("hello").is_ok());
        let msg = ch.recv().expect("recv should succeed");
        assert_eq!(msg, Some("hello".to_string()));
        assert!(ch.is_complete());
    }
    #[test]
    fn test_session_channel_violation() {
        let mut ch = SessionChannel::new(vec![SessionAction::Recv]);
        assert!(ch.send("oops").is_err());
    }
    #[test]
    fn test_session_channel_remaining_steps() {
        let ch = SessionChannel::new(vec![
            SessionAction::Send,
            SessionAction::Send,
            SessionAction::Recv,
        ]);
        assert_eq!(ch.remaining_steps(), 3);
    }
    #[test]
    fn test_capability_levels() {
        let c = Capability::read_write();
        assert!(c.is_sufficient_for(1));
        assert!(c.is_sufficient_for(2));
        assert!(!c.is_sufficient_for(3));
    }
    #[test]
    fn test_capability_attenuate() {
        let c = Capability::full();
        let reduced = c.attenuate(5);
        assert_eq!(reduced.level, 5);
    }
    #[test]
    fn test_capability_null() {
        let c = Capability::null();
        assert!(!c.is_sufficient_for(1));
    }
    #[test]
    fn test_axiom_io_denote_ty_is_pi() {
        let ty = axiom_io_denote_ty();
        assert!(matches!(ty, Expr::Pi(..)));
    }
    #[test]
    fn test_axiom_io_bracket_ty_is_pi() {
        let ty = axiom_io_bracket_ty();
        assert!(matches!(ty, Expr::Pi(..)));
    }
    #[test]
    fn test_axiom_io_monad_assoc_ty_is_pi() {
        let ty = axiom_io_monad_assoc_ty();
        assert!(matches!(ty, Expr::Pi(..)));
    }
}
