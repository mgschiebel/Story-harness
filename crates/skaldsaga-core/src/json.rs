//! JSON helpers on top of `serde_json`.
//!
//! `serde_json` is built with `preserve_order`, so object keys keep their
//! original order. That matters for files Skaldsaga edits on the writer's
//! behalf (such as `.claude/settings.json`): only the keys we change move.

pub use serde_json::{Error as ParseError, Map, Value};

/// Parses a complete JSON document.
pub fn parse(input: &str) -> Result<Value, ParseError> {
    serde_json::from_str(input)
}

/// An empty JSON object.
pub fn object() -> Value {
    Value::Object(Map::new())
}

/// Small editing and printing helpers for [`Value`].
pub trait ValueExt {
    /// Follows a path of object keys, for example `["context_window", "used_percentage"]`.
    fn get_path(&self, path: &[&str]) -> Option<&Value>;
    /// Sets a key on an object, replacing an existing value in place or
    /// appending a new entry. Turns a non-object into an object first.
    fn set(&mut self, key: &str, value: Value);
    /// Removes a key from an object, keeping the order of the other keys.
    fn remove_key(&mut self, key: &str) -> Option<Value>;
    /// Pretty-prints with two-space indentation and a trailing newline.
    fn to_pretty_string(&self) -> String;
    /// Prints on one line with no extra whitespace.
    fn to_compact_string(&self) -> String;
}

impl ValueExt for Value {
    fn get_path(&self, path: &[&str]) -> Option<&Value> {
        path.iter().try_fold(self, |v, key| v.get(key))
    }

    fn set(&mut self, key: &str, value: Value) {
        if !self.is_object() {
            *self = object();
        }
        if let Value::Object(map) = self {
            map.insert(key.to_string(), value);
        }
    }

    fn remove_key(&mut self, key: &str) -> Option<Value> {
        self.as_object_mut()?.shift_remove(key)
    }

    fn to_pretty_string(&self) -> String {
        // Serializing a Value cannot fail.
        let mut out = serde_json::to_string_pretty(self).unwrap_or_default();
        out.push('\n');
        out
    }

    fn to_compact_string(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_key_order_and_edits_in_place() {
        let mut v = parse(r#"{"z":1,"a":{"b":2},"m":3,"q":4}"#).unwrap();
        v.set("a", Value::Bool(false));
        v.set("new", "x".into());
        assert_eq!(v.remove_key("m"), Some(Value::from(3)));
        assert_eq!(
            v.to_compact_string(),
            r#"{"z":1,"a":false,"q":4,"new":"x"}"#
        );
    }

    #[test]
    fn get_path_and_set_on_non_object() {
        let v = parse(r#"{"a":{"b":{"c":"d"}}}"#).unwrap();
        assert_eq!(
            v.get_path(&["a", "b", "c"]).and_then(Value::as_str),
            Some("d")
        );
        assert!(v.get_path(&["a", "x"]).is_none());
        let mut n = Value::Null;
        n.set("k", true.into());
        assert_eq!(n.to_compact_string(), r#"{"k":true}"#);
        assert_eq!(n.remove_key("missing"), None);
    }

    #[test]
    fn pretty_prints_with_trailing_newline() {
        let v = parse(r#"{"a":[1,2],"b":{},"c":"d"}"#).unwrap();
        assert_eq!(
            v.to_pretty_string(),
            "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {},\n  \"c\": \"d\"\n}\n"
        );
    }

    #[test]
    fn rejects_bad_input() {
        for bad in ["", "{", "[1,]", "01", "{} x"] {
            assert!(parse(bad).is_err(), "should reject {bad:?}");
        }
    }
}
