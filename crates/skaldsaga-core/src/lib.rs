//! Core library for Skaldsaga.
//!
//! The deterministic parts of Skaldsaga (spec checks, the single committer,
//! the index rebuild, the session meter) live here so that the command-line
//! front end and a later standalone app share one implementation. The core
//! stays portable: no desktop-only dependencies, and file access goes through
//! [`fsio`].
//!
//! Built so far: the session meter (the pilot), status line wiring, and the
//! session-start hook.

pub mod fsio;
pub mod hooks;
pub mod json;
pub mod meter;
pub mod project;
pub mod settings;
pub mod statusline;

#[cfg(test)]
pub(crate) mod testutil;

/// Version shared by the plugin and the binary.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
