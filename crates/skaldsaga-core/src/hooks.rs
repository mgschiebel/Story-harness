//! Work done by the plugin's Claude Code hooks.
//!
//! Hooks run in every folder where the plugin is enabled, so each one first
//! checks for `.harness/` and does nothing outside a Skaldsaga project.

use crate::meter;
use crate::project::Project;
use crate::statusline;

/// The status line command for a plugin installed at `plugin_root`.
/// Quoted so a path with spaces stays one word in the shell.
pub fn meter_command(plugin_root: &str) -> String {
    format!("sh \"{plugin_root}/scripts/skaldsaga\" meter statusline")
}

/// Runs the session-start work and returns the context to add for Claude,
/// or `None` outside a Skaldsaga project.
///
/// * `source` is Claude Code's reason for the hook: `startup`, `resume`,
///   `clear` and so on. A fresh session (`startup`, `clear`) clears the last
///   meter reading so a full meter from the previous session can't end the
///   new one early; a resumed session keeps it.
/// * `plugin_root`, when known, keeps the status line pointing at the
///   current plugin folder after an update.
pub fn session_start(project: &Project, source: &str, plugin_root: Option<&str>) -> Option<String> {
    if !project.is_initialized() {
        return None;
    }
    // Failures here must not block the session; the meter degrades to "unknown".
    if matches!(source, "startup" | "clear") {
        let _ = meter::reset(project);
    }
    if let Some(root) = plugin_root.filter(|r| !r.is_empty()) {
        let _ = statusline::refresh(project, &meter_command(root));
    }
    Some(format!(
        "This folder is a Skaldsaga project. Skaldsaga test build {} is installed, and its session-start hook ran (reason: {source}).",
        crate::VERSION
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meter::Reading;
    use crate::testutil::TempDir;

    fn project_with_reading() -> (TempDir, Project) {
        let dir = TempDir::new();
        let project = Project::new(dir.path());
        std::fs::create_dir_all(project.harness_dir()).unwrap();
        let reading = Reading {
            session_id: Some("old".into()),
            used_percentage: Some(90.0),
            context_window_size: None,
            updated_at: 1,
        };
        meter::write_reading(&project, &reading).unwrap();
        (dir, project)
    }

    #[test]
    fn does_nothing_outside_a_project() {
        let dir = TempDir::new();
        let project = Project::new(dir.path());
        assert_eq!(session_start(&project, "startup", Some("/p")), None);
        assert!(!project.harness_dir().exists());
        assert!(!project.claude_settings_path().exists());
    }

    #[test]
    fn fresh_sessions_reset_the_meter() {
        for source in ["startup", "clear"] {
            let (_d, project) = project_with_reading();
            let context = session_start(&project, source, None).unwrap();
            assert!(context.contains(source));
            assert!(meter::read_reading(&project).is_none(), "{source}");
        }
    }

    #[test]
    fn resumed_sessions_keep_the_meter() {
        let (_d, project) = project_with_reading();
        assert!(session_start(&project, "resume", None).is_some());
        assert!(meter::read_reading(&project).is_some());
    }

    #[test]
    fn refreshes_a_moved_plugin_path() {
        let (_d, project) = project_with_reading();
        statusline::wire(&project, &meter_command("/old"), None).unwrap();
        session_start(&project, "startup", Some("/new place")).unwrap();
        let text = std::fs::read_to_string(project.claude_settings_path()).unwrap();
        assert!(
            text.contains(r#"sh \"/new place/scripts/skaldsaga\" meter statusline"#),
            "{text}"
        );
    }
}
