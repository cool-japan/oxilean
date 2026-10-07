//! The command line: an input file, a subcommand, or neither (refused by
//! clap, and reported by `run_cli` for a `Cli` built without an input).

use super::*;
use clap::CommandFactory;

#[test]
fn the_command_definition_passes_clap_s_assertions() {
    Cli::command().debug_assert();
}

#[test]
fn an_empty_command_line_is_refused_with_the_missing_input() {
    match Cli::try_parse_from(["oxilean-doc"]) {
        Ok(_) => panic!("an empty command line must be refused"),
        Err(e) => {
            assert_eq!(e.kind(), clap::error::ErrorKind::MissingRequiredArgument);
            assert!(e.to_string().contains("<INPUT>"), "{e}");
        }
    }
}

#[test]
fn clap_accepts_an_input_alone_and_a_subcommand_alone() {
    match Cli::try_parse_from(["oxilean-doc", "a.lean"]) {
        Ok(cli) => {
            assert!(cli.command.is_none());
            assert_eq!(cli.input, Some(PathBuf::from("a.lean")));
        }
        Err(e) => panic!("an input alone must parse: {e}"),
    }
    match Cli::try_parse_from(["oxilean-doc", "multi", "a.lean"]) {
        Ok(cli) => {
            assert!(cli.command.is_some());
            assert!(cli.input.is_none());
        }
        Err(e) => panic!("a subcommand alone must parse: {e}"),
    }
}

#[test]
fn run_cli_reports_a_missing_input_as_an_error() {
    let cli = Cli {
        command: None,
        input: None,
        output: None,
        title: None,
    };
    match run_cli(cli) {
        Err(e) => assert_eq!(e.to_string(), "input required in single-file mode"),
        Ok(()) => panic!("a command line without input must not succeed"),
    }
}

#[test]
fn run_cli_documents_a_single_file_end_to_end() {
    let dir = std::env::temp_dir().join(format!("oxilean_doc_cli_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp directory must be creatable");
    let input = dir.join("Foo.lean");
    let output = dir.join("Foo.html");
    std::fs::write(&input, "/-- A constant. -/\ndef answer : Nat := 42\n")
        .expect("temp input must be writable");
    let cli = Cli {
        command: None,
        input: Some(input),
        output: Some(output.clone()),
        title: Some("Foo".to_string()),
    };
    run_cli(cli).expect("a readable input must be documented");
    let html = std::fs::read_to_string(&output).expect("the HTML output must exist");
    assert!(html.contains("Foo"));
    let _ = std::fs::remove_dir_all(&dir);
}
