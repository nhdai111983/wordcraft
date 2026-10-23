//! NexusOffice: what the MCP server tells an agent names the product Nexus Docs and the installed
//! binaries `nexus-docs` / `nexus-docs-cli` (the Cargo binaries are still `wordcraft` /
//! `wordcraft-cli`; packaging renames them). Machine identifiers (`serverInfo.name`,
//! `wordcraft://` resource URIs) keep upstream's spelling.
// Integration tests: unwrapping and panicking on failure is fine here, unlike in shipped code (AGENTS.md).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use serde_json::{Value, json};
use wordcraft_mcp::tools::tool_definitions;
use wordcraft_mcp::{Backend, Headless, Remote, Server};

fn call(s: &mut Server, id: u64, method: &str, params: Value) -> Value {
    let line = json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}).to_string();
    serde_json::from_str(&s.handle_line(&line).unwrap()).unwrap()
}

/// Tool descriptions name the product and no upstream command.
#[test]
fn tool_definitions_name_nexus_docs() {
    let defs = serde_json::to_string(&tool_definitions()).unwrap();
    assert!(!defs.contains("wordcraft-cli") && !defs.contains("WordCraft"), "{defs}");
    assert!(defs.contains("Every Nexus Docs command") && defs.contains("whole Nexus Docs window"), "{defs}");
}

/// A UI-only tool in headless mode explains how to start the app, with the installed names.
#[test]
fn headless_no_ui_reply_names_the_nexus_binaries() {
    let mut s = Server::new(Box::new(Headless::default()));
    let init = call(&mut s, 1, "initialize", json!({"protocolVersion": "2025-06-18"}));
    assert_eq!(init["result"]["serverInfo"]["name"], "wordcraft", "a machine identifier, kept");
    assert_eq!(init["result"]["serverInfo"]["title"], "Nexus Docs");

    let reply = call(&mut s, 2, "tools/call", json!({"name": "screenshot", "arguments": {}}));
    assert_eq!(reply["result"]["isError"], true, "{reply}");
    let text = reply["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("`nexus-docs --control 7981`") && text.contains("`nexus-docs-cli mcp --connect 127.0.0.1:7981`"), "{text}");
    assert!(!text.contains("wordcraft") && !text.contains("WordCraft"), "{text}");
}

/// The bridged backend describes the app as Nexus Docs (the backend line of the MCP instructions).
#[test]
fn remote_backend_names_nexus_docs() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let remote = Remote::connect(&addr).unwrap();
    assert_eq!(remote.describe(), format!("connected to the Nexus Docs app at {addr}"));
}
