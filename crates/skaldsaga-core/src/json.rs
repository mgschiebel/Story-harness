//! Minimal JSON value, parser and pretty printer.
//!
//! This is a deliberate stand-in for `serde_json` while the pilot is built
//! without dependencies. It keeps object keys in their original order so that
//! files Skaldsaga edits on the writer's behalf (such as `.claude/settings.json`)
//! keep their layout apart from the keys we change.

use std::fmt::Write as _;

/// A JSON value. Objects keep insertion order.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}

/// A parse failure with the byte offset where it happened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub offset: usize,
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid JSON at byte {}: {}", self.offset, self.message)
    }
}

impl std::error::Error for ParseError {}

impl Value {
    /// An empty object.
    pub fn object() -> Value {
        Value::Object(Vec::new())
    }

    /// Looks up a key on an object. Returns `None` for other kinds.
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    /// Follows a path of object keys, for example `["context_window", "used_percentage"]`.
    pub fn get_path(&self, path: &[&str]) -> Option<&Value> {
        path.iter().try_fold(self, |v, key| v.get(key))
    }

    /// Sets a key on an object, replacing an existing value in place or
    /// appending a new entry. Turns a non-object into an object first.
    pub fn set(&mut self, key: &str, value: Value) {
        if !matches!(self, Value::Object(_)) {
            *self = Value::object();
        }
        if let Value::Object(entries) = self {
            match entries.iter_mut().find(|(k, _)| k == key) {
                Some(entry) => entry.1 = value,
                None => entries.push((key.to_string(), value)),
            }
        }
    }

    /// Removes a key from an object, returning its value.
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        match self {
            Value::Object(entries) => {
                let pos = entries.iter().position(|(k, _)| k == key)?;
                Some(entries.remove(pos).1)
            }
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    /// Pretty-prints with two-space indentation and a trailing newline.
    pub fn to_pretty_string(&self) -> String {
        let mut out = String::new();
        write_value(&mut out, self, 0);
        out.push('\n');
        out
    }

    /// Prints on one line with no extra whitespace.
    pub fn to_compact_string(&self) -> String {
        let mut out = String::new();
        write_compact(&mut out, self);
        out
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(v: Option<T>) -> Self {
        v.map_or(Value::Null, Into::into)
    }
}

/// Parses a complete JSON document. Trailing non-whitespace is an error.
pub fn parse(input: &str) -> Result<Value, ParseError> {
    let mut p = Parser {
        s: input.as_bytes(),
        pos: 0,
        depth: 0,
    };
    p.skip_ws();
    let v = p.value()?;
    p.skip_ws();
    if p.pos != p.s.len() {
        return Err(p.err("unexpected trailing characters"));
    }
    Ok(v)
}

const MAX_DEPTH: usize = 256;

struct Parser<'a> {
    s: &'a [u8],
    pos: usize,
    depth: usize,
}

impl Parser<'_> {
    fn err(&self, message: &str) -> ParseError {
        ParseError {
            offset: self.pos,
            message: message.to_string(),
        }
    }

    fn peek(&self) -> Option<u8> {
        self.s.get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.pos += 1;
        }
    }

    fn expect_literal(&mut self, lit: &str, v: Value) -> Result<Value, ParseError> {
        if self.s[self.pos..].starts_with(lit.as_bytes()) {
            self.pos += lit.len();
            Ok(v)
        } else {
            Err(self.err("unexpected token"))
        }
    }

    fn value(&mut self) -> Result<Value, ParseError> {
        match self.peek() {
            None => Err(self.err("unexpected end of input")),
            Some(b'n') => self.expect_literal("null", Value::Null),
            Some(b't') => self.expect_literal("true", Value::Bool(true)),
            Some(b'f') => self.expect_literal("false", Value::Bool(false)),
            Some(b'"') => Ok(Value::String(self.string()?)),
            Some(b'[') => self.array(),
            Some(b'{') => self.object(),
            Some(b'-' | b'0'..=b'9') => self.number(),
            Some(_) => Err(self.err("unexpected character")),
        }
    }

    fn enter(&mut self) -> Result<(), ParseError> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            return Err(self.err("nesting too deep"));
        }
        Ok(())
    }

    fn array(&mut self) -> Result<Value, ParseError> {
        self.enter()?;
        self.pos += 1;
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b']') {
            self.pos += 1;
            self.depth -= 1;
            return Ok(Value::Array(items));
        }
        loop {
            self.skip_ws();
            items.push(self.value()?);
            self.skip_ws();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    self.depth -= 1;
                    return Ok(Value::Array(items));
                }
                _ => return Err(self.err("expected ',' or ']'")),
            }
        }
    }

    fn object(&mut self) -> Result<Value, ParseError> {
        self.enter()?;
        self.pos += 1;
        let mut entries: Vec<(String, Value)> = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            self.depth -= 1;
            return Ok(Value::Object(entries));
        }
        loop {
            self.skip_ws();
            if self.peek() != Some(b'"') {
                return Err(self.err("expected a string key"));
            }
            let key = self.string()?;
            self.skip_ws();
            if self.peek() != Some(b':') {
                return Err(self.err("expected ':'"));
            }
            self.pos += 1;
            self.skip_ws();
            let v = self.value()?;
            // Last duplicate wins, as in most JSON readers.
            match entries.iter_mut().find(|(k, _)| *k == key) {
                Some(entry) => entry.1 = v,
                None => entries.push((key, v)),
            }
            self.skip_ws();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    self.depth -= 1;
                    return Ok(Value::Object(entries));
                }
                _ => return Err(self.err("expected ',' or '}'")),
            }
        }
    }

    fn number(&mut self) -> Result<Value, ParseError> {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        match self.peek() {
            Some(b'0') => self.pos += 1,
            Some(b'1'..=b'9') => self.digits(),
            _ => return Err(self.err("invalid number")),
        }
        if self.peek() == Some(b'.') {
            self.pos += 1;
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(self.err("invalid number"));
            }
            self.digits();
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(self.err("invalid number"));
            }
            self.digits();
        }
        let text = std::str::from_utf8(&self.s[start..self.pos])
            .map_err(|_| self.err("invalid number"))?;
        text.parse::<f64>()
            .map(Value::Number)
            .map_err(|_| self.err("invalid number"))
    }

    fn digits(&mut self) {
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.pos += 1;
        }
    }

    fn hex4(&mut self) -> Result<u32, ParseError> {
        let end = self.pos + 4;
        if end > self.s.len() {
            return Err(self.err("short unicode escape"));
        }
        let text = std::str::from_utf8(&self.s[self.pos..end])
            .map_err(|_| self.err("bad unicode escape"))?;
        let n = u32::from_str_radix(text, 16).map_err(|_| self.err("bad unicode escape"))?;
        self.pos = end;
        Ok(n)
    }

    fn string(&mut self) -> Result<String, ParseError> {
        self.pos += 1; // opening quote
        let mut out = String::new();
        loop {
            let start = self.pos;
            while let Some(c) = self.peek() {
                if c == b'"' || c == b'\\' || c < 0x20 {
                    break;
                }
                self.pos += 1;
            }
            // Input came from a &str, and we only stop on ASCII bytes, so this slice is valid UTF-8.
            out.push_str(
                std::str::from_utf8(&self.s[start..self.pos])
                    .map_err(|_| self.err("invalid UTF-8"))?,
            );
            match self.peek() {
                None => return Err(self.err("unterminated string")),
                Some(b'"') => {
                    self.pos += 1;
                    return Ok(out);
                }
                Some(b'\\') => {
                    self.pos += 1;
                    let esc = self.peek().ok_or_else(|| self.err("unterminated escape"))?;
                    self.pos += 1;
                    match esc {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{8}'),
                        b'f' => out.push('\u{c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let hi = self.hex4()?;
                            let code = if (0xD800..0xDC00).contains(&hi) {
                                if !self.s[self.pos..].starts_with(b"\\u") {
                                    return Err(self.err("lone high surrogate"));
                                }
                                self.pos += 2;
                                let lo = self.hex4()?;
                                if !(0xDC00..0xE000).contains(&lo) {
                                    return Err(self.err("invalid low surrogate"));
                                }
                                0x10000 + ((hi - 0xD800) << 10) + (lo - 0xDC00)
                            } else if (0xDC00..0xE000).contains(&hi) {
                                return Err(self.err("lone low surrogate"));
                            } else {
                                hi
                            };
                            out.push(
                                char::from_u32(code)
                                    .ok_or_else(|| self.err("invalid code point"))?,
                            );
                        }
                        _ => return Err(self.err("invalid escape")),
                    }
                }
                Some(_) => return Err(self.err("control character in string")),
            }
        }
    }
}

fn write_string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

fn write_number(out: &mut String, n: f64) {
    if !n.is_finite() {
        out.push_str("null");
    } else if n.fract() == 0.0 && n.abs() < 1e15 {
        let _ = write!(out, "{}", n as i64);
    } else {
        let _ = write!(out, "{n}");
    }
}

fn indent(out: &mut String, level: usize) {
    for _ in 0..level {
        out.push_str("  ");
    }
}

fn write_value(out: &mut String, v: &Value, level: usize) {
    match v {
        Value::Array(items) if !items.is_empty() => {
            out.push_str("[\n");
            for (i, item) in items.iter().enumerate() {
                indent(out, level + 1);
                write_value(out, item, level + 1);
                if i + 1 < items.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            indent(out, level);
            out.push(']');
        }
        Value::Object(entries) if !entries.is_empty() => {
            out.push_str("{\n");
            for (i, (k, item)) in entries.iter().enumerate() {
                indent(out, level + 1);
                write_string(out, k);
                out.push_str(": ");
                write_value(out, item, level + 1);
                if i + 1 < entries.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            indent(out, level);
            out.push('}');
        }
        other => write_compact(out, other),
    }
}

fn write_compact(out: &mut String, v: &Value) {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => write_number(out, *n),
        Value::String(s) => write_string(out, s),
        Value::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_compact(out, item);
            }
            out.push(']');
        }
        Value::Object(entries) => {
            out.push('{');
            for (i, (k, item)) in entries.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_string(out, k);
                out.push(':');
                write_compact(out, item);
            }
            out.push('}');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_status_line_sample() {
        let v = parse(
            r#"{"session_id":"abc","workspace":{"project_dir":"/p"},"context_window":{"used_percentage":42.5,"context_window_size":200000}}"#,
        )
        .unwrap();
        assert_eq!(
            v.get_path(&["workspace", "project_dir"])
                .and_then(Value::as_str),
            Some("/p")
        );
        assert_eq!(
            v.get_path(&["context_window", "used_percentage"])
                .and_then(Value::as_f64),
            Some(42.5)
        );
    }

    #[test]
    fn parses_scalars_and_nesting() {
        assert_eq!(parse("null").unwrap(), Value::Null);
        assert_eq!(parse(" true ").unwrap(), Value::Bool(true));
        assert_eq!(parse("-1.5e2").unwrap(), Value::Number(-150.0));
        assert_eq!(
            parse("[1,[2,[]],{}]").unwrap().to_compact_string(),
            "[1,[2,[]],{}]"
        );
    }

    #[test]
    fn handles_escapes_and_unicode() {
        let v = parse(r#""a\"b\\c\n\u00e9\ud83d\ude00/""#).unwrap();
        assert_eq!(v.as_str(), Some("a\"b\\c\né😀/"));
        let round = parse(&v.to_compact_string()).unwrap();
        assert_eq!(round, v);
        assert_eq!(parse("\"Skáld ✓\"").unwrap().as_str(), Some("Skáld ✓"));
    }

    #[test]
    fn rejects_bad_input() {
        for bad in [
            "",
            "{",
            "[1,]",
            "{\"a\" 1}",
            "01",
            "1.",
            "\"\\x\"",
            "\"\\ud800\"",
            "nul",
            "{} x",
            "\"a\nb\"",
        ] {
            assert!(parse(bad).is_err(), "should reject {bad:?}");
        }
        let deep = "[".repeat(MAX_DEPTH + 1) + &"]".repeat(MAX_DEPTH + 1);
        assert!(parse(&deep).is_err());
    }

    #[test]
    fn keeps_key_order_and_edits_in_place() {
        let mut v = parse(r#"{"z":1,"a":{"b":2},"m":3}"#).unwrap();
        v.set("a", Value::Bool(false));
        v.set("new", "x".into());
        assert_eq!(v.remove("m"), Some(Value::Number(3.0)));
        assert_eq!(v.to_compact_string(), r#"{"z":1,"a":false,"new":"x"}"#);
    }

    #[test]
    fn pretty_prints() {
        let v = parse(r#"{"a":[1,2],"b":{},"c":"d"}"#).unwrap();
        assert_eq!(
            v.to_pretty_string(),
            "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {},\n  \"c\": \"d\"\n}\n"
        );
    }

    #[test]
    fn numbers_print_cleanly() {
        assert_eq!(Value::Number(50.0).to_compact_string(), "50");
        assert_eq!(Value::Number(42.5).to_compact_string(), "42.5");
        assert_eq!(Value::Number(f64::NAN).to_compact_string(), "null");
    }
}
