//! Work done by the plugin's Claude Code hooks.
//!
//! Hooks run in every folder where the plugin is enabled, so each one first
//! looks for `.harness/` (in the session folder, or one level down) and does
//! nothing outside a Skaldsaga project.

use std::path::Path;

use crate::meter;
use crate::project::{self, Location, Project};
use crate::statusline;

/// The status line command for a plugin installed at `plugin_root`.
/// Quoted so a path with spaces stays one word in the shell.
pub fn meter_command(plugin_root: &str) -> String {
    format!("sh \"{plugin_root}/scripts/skaldsaga\" meter statusline")
}

/// What the session-start hook decided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionStart {
    /// The context to add for Claude, or `None` to stay silent.
    pub context: Option<String>,
    /// One plain line saying what the hook found or why it stayed silent,
    /// printed only when the debug switch is on (`SKALDSAGA_HOOK_DEBUG=1`).
    pub reason: String,
}

/// Runs the session-start work for a session that starts in `start`.
///
/// * `source` is Claude Code's reason for the hook: `startup`, `resume`,
///   `clear` and so on. A fresh session (`startup`, `clear`) clears the last
///   meter reading so a full meter from the previous session can't end the
///   new one early; a resumed session keeps it.
/// * `plugin_root`, when known, keeps the status line pointing at the
///   current plugin folder after an update.
///
/// A story one level below `start` (where cloud sessions clone a repository)
/// gets the context line only: the session doesn't load that folder's
/// settings, so touching the meter there would change files for nothing.
pub fn session_start(start: &Path, source: &str, plugin_root: Option<&str>) -> SessionStart {
    let shown = start.display();
    match project::locate(start) {
        Location::Here(root) => {
            let project = Project::new(root);
            // Failures here must not block the session; the meter degrades to "unknown".
            if matches!(source, "startup" | "clear") {
                let _ = meter::reset(&project);
            }
            if let Some(root) = plugin_root.filter(|r| !r.is_empty()) {
                let _ = statusline::refresh(&project, &meter_command(root));
            }
            SessionStart {
                context: Some(format!(
                    "This folder is a Skaldsaga project. Skaldsaga test build {} is installed, and its session-start hook ran (reason: {source}).",
                    crate::VERSION
                )),
                reason: format!("found a Skaldsaga project in {shown}"),
            }
        }
        Location::Below { name, .. } => SessionStart {
            context: Some(format!(
                "Skaldsaga project found in the `{name}` folder, one level below this session's folder. Skaldsaga test build {} is installed, and its session-start hook ran (reason: {source}); the session meter can't run from here.",
                crate::VERSION
            )),
            reason: format!(
                "found a Skaldsaga project one level down in {name}; added the context line only"
            ),
        },
        Location::Several(names) => SessionStart {
            context: None,
            reason: format!(
                "several Skaldsaga projects one level down ({}); not guessing",
                names.join(", ")
            ),
        },
        Location::Candidate { .. } | Location::Nothing => SessionStart {
            context: None,
            reason: format!("no .harness folder in {shown} or one level down"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meter::Reading;
    use crate::testutil::TempDir;

    fn project_with_reading(root: &Path) -> Project {
        let project = Project::new(root);
        std::fs::create_dir_all(project.harness_dir()).unwrap();
        let reading = Reading {
            session_id: Some("old".into()),
            used_percentage: Some(90.0),
            context_window_size: None,
            updated_at: 1,
        };
        meter::write_reading(&project, &reading).unwrap();
        project
    }

    #[test]
    fn does_nothing_outside_a_project() {
        let dir = TempDir::new();
        let project = Project::new(dir.path());
        let out = session_start(dir.path(), "startup", Some("/p"));
        assert_eq!(out.context, None);
        assert!(out.reason.contains("no .harness"), "{}", out.reason);
        assert!(!project.harness_dir().exists());
        assert!(!project.claude_local_settings_path().exists());
    }

    #[test]
    fn fresh_sessions_reset_the_meter() {
        for source in ["startup", "clear"] {
            let dir = TempDir::new();
            let project = project_with_reading(dir.path());
            let context = session_start(dir.path(), source, None).context.unwrap();
            assert!(context.starts_with("This folder is a Skaldsaga project"));
            assert!(context.contains(source));
            assert!(meter::read_reading(&project).is_none(), "{source}");
        }
    }

    #[test]
    fn resumed_sessions_keep_the_meter() {
        let dir = TempDir::new();
        let project = project_with_reading(dir.path());
        assert!(session_start(dir.path(), "resume", None).context.is_some());
        assert!(meter::read_reading(&project).is_some());
    }

    #[test]
    fn refreshes_a_moved_plugin_path() {
        let dir = TempDir::new();
        let project = project_with_reading(dir.path());
        statusline::wire(&project, &meter_command("/old"), None).unwrap();
        session_start(dir.path(), "startup", Some("/new place"));
        let text = std::fs::read_to_string(project.claude_local_settings_path()).unwrap();
        assert!(
            text.contains(r#"sh \"/new place/scripts/skaldsaga\" meter statusline"#),
            "{text}"
        );
    }

    #[test]
    fn a_story_one_level_down_gets_context_only() {
        let dir = TempDir::new();
        let project = project_with_reading(&dir.path().join("my-story"));
        statusline::wire(&project, &meter_command("/old"), None).unwrap();
        let before = std::fs::read_to_string(project.claude_local_settings_path()).unwrap();

        let out = session_start(dir.path(), "startup", Some("/new"));
        let context = out.context.unwrap();
        assert!(context.contains("Skaldsaga project found in the `my-story` folder"));
        assert!(out.reason.contains("one level down in my-story"));
        // Nothing written: the reading and the status line are untouched.
        assert!(meter::read_reading(&project).is_some());
        assert_eq!(
            std::fs::read_to_string(project.claude_local_settings_path()).unwrap(),
            before
        );
    }

    #[test]
    fn several_stories_below_stay_silent() {
        let dir = TempDir::new();
        project_with_reading(&dir.path().join("a"));
        project_with_reading(&dir.path().join("b"));
        let out = session_start(dir.path(), "startup", None);
        assert_eq!(out.context, None);
        assert!(
            out.reason.contains("(a, b); not guessing"),
            "{}",
            out.reason
        );
    }

    #[test]
    fn a_subfolder_of_a_story_stays_silent() {
        let dir = TempDir::new();
        project_with_reading(dir.path());
        std::fs::create_dir_all(dir.path().join("spec")).unwrap();
        assert_eq!(
            session_start(&dir.path().join("spec"), "startup", None).context,
            None
        );
    }
}
