//! `.harness/settings.json`: the writer's tunable settings.
//!
//! Only the parts the pilot needs are modelled here (the session meter). The
//! file may hold other keys; they are preserved untouched when we write.

use crate::fsio::{self, FsError};
use crate::json::Value;
use crate::project::Project;

/// Default: suggest wrapping up at 50% of the context window (HANDOFF, Default settings).
pub const DEFAULT_SUGGEST_WRAP_UP_PERCENT: f64 = 50.0;
/// Default: wrap up regardless at 70%.
pub const DEFAULT_WRAP_UP_PERCENT: f64 = 70.0;

/// The session meter's settings.
#[derive(Debug, Clone, PartialEq)]
pub struct MeterSettings {
    /// At this share of the context window, recommend wrapping up at the next natural pause.
    pub suggest_wrap_up_percent: f64,
    /// At this share, wrap up regardless.
    pub wrap_up_percent: f64,
    /// The writer's own status line, which the meter runs and shows in place of its own line.
    /// Stored as the original `statusLine` object (`type`, `command`, `padding`, ...).
    pub wrapped_statusline: Option<Value>,
}

impl Default for MeterSettings {
    fn default() -> Self {
        MeterSettings {
            suggest_wrap_up_percent: DEFAULT_SUGGEST_WRAP_UP_PERCENT,
            wrap_up_percent: DEFAULT_WRAP_UP_PERCENT,
            wrapped_statusline: None,
        }
    }
}

impl MeterSettings {
    /// Reads the meter section from a whole settings document, falling back to
    /// defaults for anything missing or out of range, so a hand-edited typo
    /// never stops the meter.
    pub fn from_settings(doc: Option<&Value>) -> Self {
        let mut s = MeterSettings::default();
        let Some(meter) = doc.and_then(|d| d.get("meter")) else {
            return s;
        };
        let pct = |key: &str| {
            meter
                .get(key)
                .and_then(Value::as_f64)
                .filter(|n| (1.0..=100.0).contains(n))
        };
        if let Some(n) = pct("suggest_wrap_up_percent") {
            s.suggest_wrap_up_percent = n;
        }
        if let Some(n) = pct("wrap_up_percent") {
            s.wrap_up_percent = n;
        }
        // A suggestion that comes after the hard stop would never fire.
        if s.suggest_wrap_up_percent > s.wrap_up_percent {
            s.suggest_wrap_up_percent = s.wrap_up_percent;
        }
        s.wrapped_statusline = meter
            .get("wrapped_statusline")
            .filter(|v| {
                v.get("command")
                    .and_then(Value::as_str)
                    .is_some_and(|c| !c.trim().is_empty())
            })
            .cloned();
        s
    }

    /// Loads the meter settings for a project.
    pub fn load(project: &Project) -> Result<Self, FsError> {
        let doc = fsio::read_json_object(&project.settings_path())?;
        Ok(Self::from_settings(doc.as_ref()))
    }
}

/// The meter section written by first-run setup, with the decided defaults.
pub fn default_meter_section() -> Value {
    let mut meter = Value::object();
    meter.set(
        "suggest_wrap_up_percent",
        DEFAULT_SUGGEST_WRAP_UP_PERCENT.into(),
    );
    meter.set("wrap_up_percent", DEFAULT_WRAP_UP_PERCENT.into());
    meter
}

/// Records (or clears) the writer's wrapped status line, keeping every other
/// key in the settings file as it was.
pub fn store_wrapped_statusline(project: &Project, wrapped: Option<Value>) -> Result<(), FsError> {
    let path = project.settings_path();
    let mut doc = fsio::read_json_object(&path)?.unwrap_or_else(Value::object);
    let mut meter = doc
        .get("meter")
        .filter(|m| m.is_object())
        .cloned()
        .unwrap_or_else(default_meter_section);
    match wrapped {
        Some(v) => meter.set("wrapped_statusline", v),
        None => {
            meter.remove("wrapped_statusline");
        }
    }
    doc.set("meter", meter);
    fsio::write_json(&path, &doc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::parse;
    use crate::testutil::TempDir;

    #[test]
    fn defaults_when_missing() {
        assert_eq!(MeterSettings::from_settings(None), MeterSettings::default());
        let doc = parse(r#"{"other":true}"#).unwrap();
        assert_eq!(
            MeterSettings::from_settings(Some(&doc)),
            MeterSettings::default()
        );
    }

    #[test]
    fn reads_values_and_ignores_bad_ones() {
        let doc =
            parse(r#"{"meter":{"suggest_wrap_up_percent":40,"wrap_up_percent":"lots"}}"#).unwrap();
        let s = MeterSettings::from_settings(Some(&doc));
        assert_eq!(s.suggest_wrap_up_percent, 40.0);
        assert_eq!(s.wrap_up_percent, DEFAULT_WRAP_UP_PERCENT);

        let doc =
            parse(r#"{"meter":{"suggest_wrap_up_percent":0,"wrap_up_percent":150}}"#).unwrap();
        assert_eq!(
            MeterSettings::from_settings(Some(&doc)),
            MeterSettings::default()
        );
    }

    #[test]
    fn suggestion_never_comes_after_hard_stop() {
        let doc =
            parse(r#"{"meter":{"suggest_wrap_up_percent":80,"wrap_up_percent":60}}"#).unwrap();
        let s = MeterSettings::from_settings(Some(&doc));
        assert_eq!((s.suggest_wrap_up_percent, s.wrap_up_percent), (60.0, 60.0));
    }

    #[test]
    fn wrapped_statusline_needs_a_command() {
        let doc =
            parse(r#"{"meter":{"wrapped_statusline":{"type":"command","command":"  "}}}"#).unwrap();
        assert!(
            MeterSettings::from_settings(Some(&doc))
                .wrapped_statusline
                .is_none()
        );
        let doc =
            parse(r#"{"meter":{"wrapped_statusline":{"type":"command","command":"my-line"}}}"#)
                .unwrap();
        assert!(
            MeterSettings::from_settings(Some(&doc))
                .wrapped_statusline
                .is_some()
        );
    }

    #[test]
    fn storing_wrapped_line_preserves_other_keys() {
        let dir = TempDir::new();
        let project = Project::new(dir.path());
        std::fs::create_dir_all(project.harness_dir()).unwrap();
        std::fs::write(
            project.settings_path(),
            r#"{"models":{"housekeeper":"strong"},"meter":{"wrap_up_percent":65}}"#,
        )
        .unwrap();
        let line = parse(r#"{"type":"command","command":"mine"}"#).unwrap();
        store_wrapped_statusline(&project, Some(line)).unwrap();
        let s = MeterSettings::load(&project).unwrap();
        assert_eq!(s.wrap_up_percent, 65.0);
        assert!(s.wrapped_statusline.is_some());
        let text = std::fs::read_to_string(project.settings_path()).unwrap();
        assert!(text.contains("housekeeper"));

        store_wrapped_statusline(&project, None).unwrap();
        assert!(
            MeterSettings::load(&project)
                .unwrap()
                .wrapped_statusline
                .is_none()
        );
    }
}
