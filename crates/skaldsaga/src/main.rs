//! `skaldsaga`: the command-line front end for Skaldsaga's deterministic scripts.
//!
//! One binary with subcommands. First-run setup, the session meter, the
//! session-start hook and the index rebuild are built so far; the other
//! subcommands are reserved and say so.

mod wrapped;

use std::io::{self, Read as _, Write as _};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use skaldsaga_core::hooks;
use skaldsaga_core::index;
use skaldsaga_core::init::{self, InitOutcome, InitReport};
use skaldsaga_core::json::{self, Value, ValueExt as _};
use skaldsaga_core::meter::{self, StatusInput};
use skaldsaga_core::project::Project;
use skaldsaga_core::settings::MeterSettings;
use skaldsaga_core::statusline::{self, WireError, WireOutcome};

const USAGE: &str = "\
skaldsaga: deterministic scripts for the Skaldsaga story mentor

Usage:
  skaldsaga init [--project DIR] [--meter-command CMD] [--user-settings FILE]
                                     First-run setup for a fresh project: story folders,
                                     .harness and default settings. Safe to run again.
                                     With --meter-command, also wires the session meter.
                                     Prints what it did as JSON
  skaldsaga meter statusline         Status line command: reads Claude Code's JSON on
                                     stdin, records context usage, prints a status line
  skaldsaga meter check [--session ID] [--project DIR]
                                     Prints the meter state as JSON
  skaldsaga meter reset [--project DIR]
                                     Clears the last reading (for a new session)
  skaldsaga meter wire --command CMD [--project DIR] [--user-settings FILE]
                                     Wires the meter into the project's status line
  skaldsaga hook session-start       SessionStart hook: reads Claude Code's JSON on stdin;
                                     in a Skaldsaga project, prepares the session and adds
                                     context for Claude. Elsewhere it prints nothing
  skaldsaga index [--project DIR]    Rebuilds the generated index in spec/_index (backlinks,
                                     attributes, grid, scenes, status). Prints a summary as JSON
  skaldsaga --version                Prints the version

Not built yet: check, commit, complete

DIR defaults to $CLAUDE_PROJECT_DIR, then the current folder.
";

/// Exit code for usage mistakes.
const USAGE_ERROR: u8 = 64;
/// Exit code for a reserved subcommand that isn't built yet.
const NOT_BUILT: u8 = 69;
/// Exit code for a failure while doing the work.
const FAILURE: u8 = 1;

/// Upper bound on status line input; the real payload is a few kilobytes.
const MAX_STDIN: u64 = 4 * 1024 * 1024;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        [] | ["help" | "--help" | "-h"] => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
        ["--version" | "-V" | "version"] => {
            println!("skaldsaga {}", skaldsaga_core::VERSION);
            ExitCode::SUCCESS
        }
        ["meter", "statusline", ..] => meter_statusline(),
        ["meter", "check", rest @ ..] => run(meter_check(rest)),
        ["meter", "reset", rest @ ..] => run(meter_reset(rest)),
        ["meter", "wire", rest @ ..] => run(meter_wire(rest)),
        ["hook", "session-start", ..] => hook_session_start(),
        ["init", rest @ ..] => run(init_cmd(rest)),
        ["index", rest @ ..] => run(index_cmd(rest)),
        [cmd @ ("check" | "commit" | "complete"), ..] => {
            eprintln!("skaldsaga {cmd}: not built yet");
            ExitCode::from(NOT_BUILT)
        }
        _ => {
            eprint!("skaldsaga: unknown command\n\n{USAGE}");
            ExitCode::from(USAGE_ERROR)
        }
    }
}

enum CliError {
    Usage(String),
    Failed(String),
}

fn run(result: Result<(), CliError>) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(CliError::Usage(msg)) => {
            eprintln!("skaldsaga: {msg}\n\n{USAGE}");
            ExitCode::from(USAGE_ERROR)
        }
        Err(CliError::Failed(msg)) => {
            eprintln!("skaldsaga: {msg}");
            ExitCode::from(FAILURE)
        }
    }
}

/// Parses `--flag value` pairs, allowing only the given flags.
fn flags<'a>(rest: &[&'a str], allowed: &[&str]) -> Result<Vec<(&'a str, &'a str)>, CliError> {
    let mut out = Vec::new();
    let mut it = rest.iter();
    while let Some(&flag) = it.next() {
        if !allowed.contains(&flag) {
            return Err(CliError::Usage(format!("unexpected argument '{flag}'")));
        }
        let Some(&value) = it.next() else {
            return Err(CliError::Usage(format!("{flag} needs a value")));
        };
        out.push((flag, value));
    }
    Ok(out)
}

fn flag<'a>(parsed: &[(&str, &'a str)], name: &str) -> Option<&'a str> {
    parsed
        .iter()
        .rev()
        .find(|(f, _)| *f == name)
        .map(|(_, v)| *v)
}

fn default_project_dir() -> PathBuf {
    std::env::var_os("CLAUDE_PROJECT_DIR")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}

fn project_from(parsed: &[(&str, &str)]) -> Project {
    Project::new(flag(parsed, "--project").map_or_else(default_project_dir, PathBuf::from))
}

/// The writer's personal Claude Code settings file.
fn default_user_settings() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("CLAUDE_CONFIG_DIR").filter(|v| !v.is_empty()) {
        return Some(PathBuf::from(dir).join("settings.json"));
    }
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .filter(|v| !v.is_empty())?;
    Some(PathBuf::from(home).join(".claude").join("settings.json"))
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

fn print_json(v: &Value) -> Result<(), CliError> {
    let mut out = io::stdout().lock();
    out.write_all(v.to_pretty_string().as_bytes())
        .map_err(|e| CliError::Failed(e.to_string()))
}

/// The status line command. It must never fail visibly: whatever happens, it
/// prints a line (possibly empty) and exits successfully.
fn meter_statusline() -> ExitCode {
    let mut raw = String::new();
    let _ = io::stdin().lock().take(MAX_STDIN).read_to_string(&mut raw);
    let input = StatusInput::parse(&raw);

    let project = input
        .as_ref()
        .and_then(|i| i.project_dir.as_deref())
        .filter(|d| !d.is_empty())
        .map_or_else(|| Project::new(default_project_dir()), Project::new);

    if !project.is_initialized() {
        // Not a Skaldsaga project: stay out of the way.
        println!();
        return ExitCode::SUCCESS;
    }

    let reading = input.as_ref().map(|i| i.to_reading(now()));
    if let Some(reading) = &reading {
        // A failed write only means a missing reading; the manual session boundary still works.
        let _ = meter::write_reading(&project, reading);
    }

    let settings = MeterSettings::load(&project).unwrap_or_default();
    let line = settings
        .wrapped_statusline
        .as_ref()
        .and_then(|w| w.get("command").and_then(Value::as_str))
        .and_then(|cmd| wrapped::run(cmd, &raw, project.root()))
        .unwrap_or_else(|| {
            let current = input.as_ref().and_then(|i| i.session_id.as_deref());
            format!(
                "{}\n",
                meter::evaluate(reading.as_ref(), &settings, current).status_text()
            )
        });
    let mut out = io::stdout().lock();
    let _ = out.write_all(line.as_bytes());
    let _ = out.flush();
    ExitCode::SUCCESS
}

fn meter_check(rest: &[&str]) -> Result<(), CliError> {
    let parsed = flags(rest, &["--session", "--project"])?;
    let project = project_from(&parsed);
    let settings = MeterSettings::load(&project).unwrap_or_default();
    let reading = meter::read_reading(&project);
    let eval = meter::evaluate(reading.as_ref(), &settings, flag(&parsed, "--session"));
    print_json(&eval.to_json())
}

fn meter_reset(rest: &[&str]) -> Result<(), CliError> {
    let parsed = flags(rest, &["--project"])?;
    let project = project_from(&parsed);
    if !project.is_initialized() {
        // The SessionStart hook calls this in every folder; outside a project it does nothing.
        return Ok(());
    }
    meter::reset(&project)
        .map(|_| ())
        .map_err(|e| CliError::Failed(e.to_string()))
}

fn meter_wire(rest: &[&str]) -> Result<(), CliError> {
    let parsed = flags(rest, &["--command", "--project", "--user-settings"])?;
    let command = flag(&parsed, "--command")
        .ok_or_else(|| CliError::Usage("--command is required".into()))?;
    if !command.contains(statusline::METER_SUBCOMMAND) {
        return Err(CliError::Usage(format!(
            "--command must run '{}'",
            statusline::METER_SUBCOMMAND
        )));
    }
    let project = project_from(&parsed);
    let user = flag(&parsed, "--user-settings")
        .map(PathBuf::from)
        .or_else(default_user_settings);

    print_json(&wire_meter(&project, command, user.as_deref())?)
}

/// Wires the meter and describes the outcome as JSON.
fn wire_meter(
    project: &Project,
    command: &str,
    user: Option<&std::path::Path>,
) -> Result<Value, CliError> {
    let outcome = statusline::wire(project, command, user).map_err(|e| match e {
        WireError::NotInitialized => CliError::Failed(e.to_string()),
        WireError::Fs(e) => CliError::Failed(e.to_string()),
    })?;

    let mut v = json::object();
    match outcome {
        WireOutcome::AlreadyWired => v.set("outcome", "already_wired".into()),
        WireOutcome::Updated => v.set("outcome", "updated".into()),
        WireOutcome::Wired { wrapped } => {
            v.set("outcome", "wired".into());
            v.set("wrapped", wrapped.unwrap_or(Value::Null));
        }
        WireOutcome::OverriddenByLocal { existing } => {
            v.set("outcome", "overridden_by_local".into());
            v.set("existing", existing);
        }
    }
    Ok(v)
}

fn strings(items: &[String]) -> Value {
    Value::Array(items.iter().map(|s| Value::from(s.as_str())).collect())
}

fn init_cmd(rest: &[&str]) -> Result<(), CliError> {
    let parsed = flags(rest, &["--project", "--meter-command", "--user-settings"])?;
    let meter_command = flag(&parsed, "--meter-command");
    if let Some(cmd) = meter_command
        && !cmd.contains(statusline::METER_SUBCOMMAND)
    {
        return Err(CliError::Usage(format!(
            "--meter-command must run '{}'",
            statusline::METER_SUBCOMMAND
        )));
    }
    let project = project_from(&parsed);
    let report = init::init(&project).map_err(|e| CliError::Failed(e.to_string()))?;

    let mut v = json::object();
    v.set("outcome", report.outcome.as_str().into());
    if report.outcome == InitOutcome::ExistingFiles {
        v.set("existing", strings(&report.existing));
        v.set("existing_count", Value::from(report.existing_count));
    } else {
        v.set("created", strings(&report.created));
        v.set("commit_paths", strings(&InitReport::commit_paths()));
        let meter = match meter_command {
            Some(cmd) => {
                let user = flag(&parsed, "--user-settings")
                    .map(PathBuf::from)
                    .or_else(default_user_settings);
                wire_meter(&project, cmd, user.as_deref())?
            }
            None => Value::Null,
        };
        v.set("meter", meter);
    }
    print_json(&v)
}

/// Rebuilds `spec/_index/`. Refuses outside a Skaldsaga project, so it never
/// writes into a folder the writer hasn't set up.
fn index_cmd(rest: &[&str]) -> Result<(), CliError> {
    let parsed = flags(rest, &["--project"])?;
    let project = project_from(&parsed);
    if !project.is_initialized() {
        return Err(CliError::Failed(
            "this folder isn't set up for Skaldsaga yet (no .harness folder)".into(),
        ));
    }
    let rebuilt =
        index::rebuild(&project.spec_dir()).map_err(|e| CliError::Failed(e.to_string()))?;
    let mut v = json::object();
    v.set("outcome", "rebuilt".into());
    v.set("files", strings(&rebuilt.files));
    v.set("nodes", Value::from(rebuilt.nodes));
    v.set("problems", Value::from(rebuilt.problems));
    v.set("broken_references", Value::from(rebuilt.broken_references));
    print_json(&v)
}

/// The SessionStart hook. Like the status line, it must never get in the
/// writer's way: it always exits successfully, and prints nothing outside a
/// Skaldsaga project.
fn hook_session_start() -> ExitCode {
    let mut raw = String::new();
    let _ = io::stdin().lock().take(MAX_STDIN).read_to_string(&mut raw);
    let input = json::parse(&raw).ok();
    let field = |key: &str| {
        input
            .as_ref()
            .and_then(|v| v.get(key))
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
    };

    // Claude Code exports CLAUDE_PROJECT_DIR to hooks; fall back to the cwd it reports.
    let project_dir = std::env::var_os("CLAUDE_PROJECT_DIR")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| field("cwd").map(PathBuf::from))
        .unwrap_or_else(default_project_dir);
    let source = field("source").unwrap_or_else(|| "startup".to_string());
    let plugin_root = std::env::var("CLAUDE_PLUGIN_ROOT").ok();

    let Some(context) =
        hooks::session_start(&Project::new(project_dir), &source, plugin_root.as_deref())
    else {
        return ExitCode::SUCCESS;
    };
    let mut specific = json::object();
    specific.set("hookEventName", "SessionStart".into());
    specific.set("additionalContext", context.into());
    let mut out = json::object();
    out.set("hookSpecificOutput", specific);
    println!("{}", out.to_compact_string());
    ExitCode::SUCCESS
}
