//! End-to-end tests for `skaldsaga hook session-start`, run the way Claude
//! Code runs a plugin hook: JSON on stdin, plugin variables in the environment.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

struct TempDir(PathBuf);

impl TempDir {
    fn new() -> Self {
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("skaldsaga-hook-test-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        TempDir(path)
    }
    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn hook(project: &Path, stdin: &str, plugin_root: Option<&str>) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_skaldsaga"));
    cmd.args(["hook", "session-start"])
        .env("CLAUDE_PROJECT_DIR", project)
        .env_remove("CLAUDE_PLUGIN_ROOT")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(root) = plugin_root {
        cmd.env("CLAUDE_PLUGIN_ROOT", root);
    }
    let mut child = cmd.spawn().unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

#[test]
fn silent_outside_a_project() {
    let dir = TempDir::new();
    let o = hook(dir.path(), r#"{"source":"startup"}"#, Some("/plugin"));
    assert!(o.status.success());
    assert_eq!(stdout(&o), "");
    assert!(!dir.path().join(".harness").exists());
}

#[test]
fn adds_context_and_resets_the_meter_in_a_project() {
    let dir = TempDir::new();
    let harness = dir.path().join(".harness");
    std::fs::create_dir_all(&harness).unwrap();
    std::fs::write(
        harness.join("meter.json"),
        r#"{"session_id":"old","used_percentage":90,"updated_at":1}"#,
    )
    .unwrap();

    let o = hook(
        dir.path(),
        r#"{"session_id":"new","source":"clear","cwd":"/elsewhere"}"#,
        None,
    );
    assert!(o.status.success());
    let out: serde_json::Value =
        serde_json::from_str(stdout(&o).trim()).expect("hook prints one JSON object");
    assert_eq!(out["hookSpecificOutput"]["hookEventName"], "SessionStart");
    let context = out["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .unwrap();
    assert!(context.contains("Skaldsaga project"), "{context}");
    assert!(context.contains("clear"), "{context}");
    assert!(!harness.join("meter.json").exists());
}

#[test]
fn refreshes_the_status_line_after_a_plugin_update() {
    let dir = TempDir::new();
    std::fs::create_dir_all(dir.path().join(".harness")).unwrap();
    std::fs::create_dir_all(dir.path().join(".claude")).unwrap();
    std::fs::write(
        dir.path().join(".claude/settings.json"),
        r#"{"statusLine":{"type":"command","command":"sh \"/old/scripts/skaldsaga\" meter statusline"}}"#,
    )
    .unwrap();
    let o = hook(dir.path(), r#"{"source":"startup"}"#, Some("/new"));
    assert!(o.status.success());
    let settings = std::fs::read_to_string(dir.path().join(".claude/settings.json")).unwrap();
    assert!(settings.contains(r#"/new/scripts/skaldsaga"#), "{settings}");
}

#[test]
fn garbage_input_still_succeeds() {
    let dir = TempDir::new();
    std::fs::create_dir_all(dir.path().join(".harness")).unwrap();
    for input in ["", "not json", "[]"] {
        let o = hook(dir.path(), input, None);
        assert!(o.status.success(), "{input:?}");
        assert!(stdout(&o).contains("additionalContext"), "{input:?}");
    }
}
