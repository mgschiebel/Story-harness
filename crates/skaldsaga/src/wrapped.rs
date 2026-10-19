//! Running the writer's own status line command.
//!
//! This is the only process launching in the front end. It stays out of the
//! core so the core keeps no process dependencies.

use std::io::{Read as _, Write as _};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// How long the writer's command may take. Claude Code cancels a slow status
/// line anyway; this just keeps us from waiting forever.
const TIMEOUT: Duration = Duration::from_secs(5);
/// Most output we pass through.
const MAX_OUTPUT: u64 = 64 * 1024;

#[cfg(windows)]
fn shell(cmd: &str) -> Command {
    let mut c = Command::new("cmd");
    c.args(["/C", cmd]);
    c
}

#[cfg(not(windows))]
fn shell(cmd: &str) -> Command {
    let mut c = Command::new("sh");
    c.args(["-c", cmd]);
    c
}

/// Runs `cmd` with the status line JSON on stdin and returns what it printed.
/// Returns `None` if it fails, times out or prints nothing, so the caller can
/// fall back to the meter's own line.
pub fn run(cmd: &str, stdin_json: &str, cwd: &Path) -> Option<String> {
    let mut child = shell(cmd)
        .current_dir(cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    // Feed stdin from a thread so a command that ignores its input can't block us.
    let mut stdin = child.stdin.take()?;
    let payload = stdin_json.to_owned();
    let writer = std::thread::spawn(move || {
        let _ = stdin.write_all(payload.as_bytes());
    });

    let mut stdout = child.stdout.take()?;
    let reader = std::thread::spawn(move || {
        let mut out = Vec::new();
        let _ = (&mut stdout).take(MAX_OUTPUT).read_to_end(&mut out);
        out
    });

    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if start.elapsed() < TIMEOUT => std::thread::sleep(Duration::from_millis(10)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
        }
    };
    // On timeout, don't wait for the pipes: a grandchild may still hold them
    // open. The threads are dropped when this short-lived process exits.
    let status = status?;
    let _ = writer.join();
    let out = reader.join().ok()?;
    if !status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out).into_owned();
    if text.trim().is_empty() {
        None
    } else {
        Some(text)
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn passes_output_through_and_feeds_stdin() {
        let dir = std::env::temp_dir();
        assert_eq!(run("echo hello", "{}", &dir).as_deref(), Some("hello\n"));
        assert_eq!(run("cat", "{\"a\":1}", &dir).as_deref(), Some("{\"a\":1}"));
    }

    #[test]
    fn failures_fall_back() {
        let dir = std::env::temp_dir();
        assert_eq!(run("exit 3", "{}", &dir), None);
        assert_eq!(run("true", "{}", &dir), None);
        assert_eq!(run("definitely-not-a-command-xyz", "{}", &dir), None);
    }

    #[test]
    fn slow_commands_time_out() {
        let dir = std::env::temp_dir();
        let start = Instant::now();
        // exec so the kill reaches the sleeping process rather than a parent shell.
        assert_eq!(run("exec sleep 30", "{}", &dir), None);
        assert!(start.elapsed() < Duration::from_secs(10));
    }
}
