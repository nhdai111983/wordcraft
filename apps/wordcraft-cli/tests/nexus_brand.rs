//! NexusOffices: the CLI is installed as `nexus-docs-cli` (the Cargo binary is still
//! `wordcraft-cli`; packaging renames it), so its usage, version and error text use that name.
// Integration tests: unwrapping and panicking on failure is fine here, unlike in shipped code (AGENTS.md).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::process::{Command, Output};

fn cli(args: &[&str]) -> Output {
    // `output()` gives the child no stdin, so nothing waits for input.
    Command::new(env!("CARGO_BIN_EXE_wordcraft-cli")).args(args).output().expect("run the CLI")
}

#[test]
fn version_usage_and_errors_name_nexus_docs_cli() {
    let version = cli(&["--version"]);
    assert!(version.status.success());
    let version = String::from_utf8_lossy(&version.stdout).trim().to_string();
    assert_eq!(version, format!("nexus-docs-cli {}", env!("CARGO_PKG_VERSION")));

    let help = String::from_utf8_lossy(&cli(&["--help"]).stdout).to_string();
    assert!(help.starts_with("nexus-docs-cli — Nexus Docs from the command line\n"), "{help}");
    assert!(help.contains("\n  nexus-docs-cli mcp [--connect HOST:PORT"), "{help}");
    // `WORDCRAFT_REGION` is an environment variable and keeps upstream's spelling.
    assert!(!help.contains("wordcraft") && !help.contains("WordCraft"), "{help}");

    // Errors are prefixed with the installed name too.
    let out = cli(&["frobnicate"]);
    assert!(!out.status.success());
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.starts_with("nexus-docs-cli: unknown command `frobnicate`"), "{err}");
    assert!(!err.contains("wordcraft") && !err.contains("WordCraft"), "{err}");
}

/// `mcp --connect` to a port nothing listens on names the app as installed.
#[test]
fn unreachable_app_is_named_nexus_docs() {
    // A port that was free a moment ago: bind, read the port, close.
    let port = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
    let out = cli(&["mcp", "--connect", &format!("127.0.0.1:{port}")]);
    assert!(!out.status.success());
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.starts_with("nexus-docs-cli: "), "{err}");
    assert!(err.contains(&format!("can't reach the Nexus Docs app at 127.0.0.1:{port}")), "{err}");
    assert!(!err.contains("WordCraft") && !err.contains("wordcraft-cli"), "{err}");
}
