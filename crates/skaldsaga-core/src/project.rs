//! Where Skaldsaga's files live inside a writer's project folder.

use std::path::{Path, PathBuf};

/// The hidden folder that marks a folder as a Skaldsaga project.
pub const HARNESS_DIR: &str = ".harness";

/// Paths inside one project folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    root: PathBuf,
}

impl Project {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Project { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn harness_dir(&self) -> PathBuf {
        self.root.join(HARNESS_DIR)
    }

    /// True once first-run setup has created `.harness/`. Everything that
    /// runs automatically (status line, hooks) checks this first, so
    /// installing the plugin never affects folders that aren't Skaldsaga
    /// projects.
    pub fn is_initialized(&self) -> bool {
        self.harness_dir().is_dir()
    }

    /// The story spec, the source of truth.
    pub fn spec_dir(&self) -> PathBuf {
        self.root.join("spec")
    }

    /// Pending proposals, one JSON file each.
    pub fn inbox_dir(&self) -> PathBuf {
        self.harness_dir().join("inbox")
    }

    pub fn settings_path(&self) -> PathBuf {
        self.harness_dir().join("settings.json")
    }

    pub fn meter_path(&self) -> PathBuf {
        self.harness_dir().join("meter.json")
    }

    /// Claude Code's shared project settings, where the status line is wired.
    pub fn claude_settings_path(&self) -> PathBuf {
        self.root.join(".claude").join("settings.json")
    }

    /// Claude Code's personal project settings, which override the shared file.
    pub fn claude_local_settings_path(&self) -> PathBuf {
        self.root.join(".claude").join("settings.local.json")
    }
}
