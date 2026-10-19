//! Proposals: one JSON file per confirmed answer, waiting in
//! `.harness/inbox/` until it is checked and committed.
//!
//! The format follows HANDOFF, Node format, Proposal. A proposal is a list of
//! field-level operations plus the bookkeeping the checker and the committer
//! need: which agent made it (its lane), the writer's own words, and the
//! revisions it was written and checked against. `check` writes its verdict
//! back into the same file, under `check`.
//!
//! ```json
//! {
//!   "id": "pr-7kq2xmhd",
//!   "agent": "interviewer",
//!   "created": "2026-10-18T12:58",
//!   "answer": "Mara learns the truth from Jonah's sister",
//!   "ops": [
//!     {"op": "set", "target": "p-mara2345#f-trst2345.ends", "value": {"in": "s-scene4aa"}},
//!     {"op": "add_item", "target": "s-scene4aa.participants", "value": "p-jnah2345"}
//!   ],
//!   "revisions": {"writes": {"f-trst2345": 2}, "read": {}, "checked": {}}
//! }
//! ```
//!
//! Facts are addressed by ID (`node#fact.field`), never by list position, so
//! parallel edits can't shift each other.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::json::Value;
use crate::model::FactKind;
use crate::registry::AttributeDef;

/// One field-level change. The list follows HANDOFF (Node format, Op list),
/// plus `add_attribute`, which records a new story key in
/// `spec/attributes.md` (added in build step 7, since only proposals change
/// that file).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Op {
    /// A new node, in the same shape as a node file's front matter
    /// (`id`, `type`, `name`, … and optional `facts`). `revision` may be left
    /// out; new nodes and facts start at 1.
    Create {
        node: Value,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        notes: Option<String>,
    },
    /// Sets one field: `node.field` or `node#fact.field`. `null` clears an
    /// optional field. `id`, `type`, `revision` and `facts` can't be set.
    Set { target: String, value: Value },
    /// Adds a fact to a node. The fact's ID must be new.
    AddFact { node: String, fact: Value },
    /// Adds an item to a list field (`s-….participants`, `p-…#f-….links`), or
    /// a scene to the reading order (`sequence`, optionally `after` or
    /// `before` another scene; otherwise at the end).
    AddItem {
        target: String,
        value: Value,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        after: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        before: Option<String>,
    },
    /// Removes an item from a list field, or a scene from the reading order.
    RemoveItem { target: String, value: Value },
    /// The writer changed their mind: the fact is replaced by a new one (new
    /// ID) on the same node, and references to the old fact move to the new
    /// one. Scenes that read it are flagged.
    Supersede { target: String, replacement: Value },
    /// Removes a node or a fact. Blocked while anything still references it.
    Delete { target: String },
    /// Two entities become one: the merged node's facts move to the survivor,
    /// every reference is redirected, and the merged node is removed.
    Merge { survivor: String, merged: String },
    /// Moves a fact to another node, keeping its ID and revision.
    MoveFact { fact: String, to: String },
    /// A truth becomes a belief, or the reverse. Becoming a belief needs a
    /// `holder` (a person ID or `reader`).
    Reclassify {
        target: String,
        kind: FactKind,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        holder: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        contradicts: Option<String>,
    },
    /// Records a new story key in `spec/attributes.md`.
    AddAttribute { attribute: AttributeDef },
}

impl Op {
    pub fn name(&self) -> &'static str {
        match self {
            Op::Create { .. } => "create",
            Op::Set { .. } => "set",
            Op::AddFact { .. } => "add_fact",
            Op::AddItem { .. } => "add_item",
            Op::RemoveItem { .. } => "remove_item",
            Op::Supersede { .. } => "supersede",
            Op::Delete { .. } => "delete",
            Op::Merge { .. } => "merge",
            Op::MoveFact { .. } => "move_fact",
            Op::Reclassify { .. } => "reclassify",
            Op::AddAttribute { .. } => "add_attribute",
        }
    }
}

/// The lane a proposal travels in (HANDOFF, Governance, Per-agent lanes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Lane {
    /// Commits right away as unreviewed; a later housekeeper pass clears it.
    Optimistic,
    /// Held until a housekeeper scoped pass clears it.
    Strict,
}

/// Revisions a proposal was written and checked against, keyed by node or
/// fact ID. `sequence.md` and `attributes.md` have no revision; they are
/// recorded by a fingerprint of their content instead.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Revisions {
    /// What the proposal writes, as its author saw it.
    #[serde(default)]
    pub writes: BTreeMap<String, Value>,
    /// What its author read to form the answer.
    #[serde(default)]
    pub read: BTreeMap<String, Value>,
    /// What the checker examined when clearing it (written by `check`).
    #[serde(default)]
    pub checked: BTreeMap<String, Value>,
}

/// One proposal file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Proposal {
    pub id: String,
    /// `interviewer`, `harvester`, `researcher`, `weaver` or `outliner`.
    pub agent: String,
    /// The lane the author asked for. An agent can ask for the strict lane
    /// but never loosen its own: the checker's lane is never lower.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lane: Option<Lane>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The writer's answer in their own words, for recaps.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    pub ops: Vec<Op>,
    #[serde(default)]
    pub revisions: Revisions,
    /// The checker's verdict, written by `check`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check: Option<Value>,
}

/// Parses a proposal file.
pub fn parse(text: &str) -> Result<Proposal, String> {
    serde_json::from_str(text).map_err(|e| e.to_string())
}

/// Renders a proposal file.
pub fn render(proposal: &Proposal) -> String {
    // Plain data with string keys; serializing cannot fail.
    let mut out = serde_json::to_string_pretty(proposal).unwrap_or_default();
    out.push('\n');
    out
}

/// Where a `set`, `add_item` or `remove_item` points.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    /// `sequence`: the reading order.
    Sequence,
    /// `node.field`
    NodeField { node: String, field: String },
    /// `node#fact.field`
    FactField {
        node: String,
        fact: String,
        field: String,
    },
}

/// Where a `delete`, `supersede` or `reclassify` points.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemRef {
    Node(String),
    /// `node#fact`, or a bare fact ID.
    Fact {
        node: Option<String>,
        fact: String,
    },
}

/// Parses `sequence`, `node.field` or `node#fact.field`.
pub fn parse_target(s: &str) -> Result<Target, String> {
    if s == "sequence" {
        return Ok(Target::Sequence);
    }
    let Some((head, field)) = s.split_once('.') else {
        return Err(format!(
            "'{s}' should look like 'node.field' or 'node#fact.field'"
        ));
    };
    if field.is_empty() || field.contains('.') || head.is_empty() {
        return Err(format!(
            "'{s}' should name one field: 'node.field' or 'node#fact.field'"
        ));
    }
    Ok(match head.split_once('#') {
        Some((node, fact)) => Target::FactField {
            node: node.to_string(),
            fact: fact.to_string(),
            field: field.to_string(),
        },
        None => Target::NodeField {
            node: head.to_string(),
            field: field.to_string(),
        },
    })
}

/// Parses `node`, `node#fact` or a bare fact ID.
pub fn parse_item_ref(s: &str) -> ItemRef {
    match s.split_once('#') {
        Some((node, fact)) => ItemRef::Fact {
            node: Some(node.to_string()),
            fact: fact.to_string(),
        },
        None if crate::ids::is_kind(s, crate::ids::IdKind::Fact) => ItemRef::Fact {
            node: None,
            fact: s.to_string(),
        },
        None => ItemRef::Node(s.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_handoff_example() {
        let text = r#"{
          "id": "pr-7kq2xmhd",
          "agent": "interviewer",
          "lane": "optimistic",
          "created": "2026-10-18T12:58",
          "answer": "Mara learns the truth from Jonah's sister",
          "ops": [
            {"op": "add_fact", "node": "p-7kq2xmhd", "fact": {"id": "f-3wq8zkpa"}},
            {"op": "set", "target": "p-7kq2xmhd#f-3wq8zkpa.ends", "value": {"in": "s-4hq8nzwt"}},
            {"op": "add_item", "target": "s-4hq8nzwt.participants", "value": "p-p3rwe9ka"},
            {"op": "add_item", "target": "sequence", "value": "s-4hq8nzwt", "after": "s-7kq2xmhd"},
            {"op": "move_fact", "fact": "f-3wq8zkpa", "to": "p-p3rwe9ka"},
            {"op": "reclassify", "target": "p-7kq2xmhd#f-3wq8zkpa", "kind": "belief", "holder": "reader"},
            {"op": "add_attribute", "attribute": {"key": "owes", "relation": "one_way", "cardinality": "per_target"}}
          ],
          "revisions": {"writes": {}, "read": {}, "checked": {}}
        }"#;
        let p = parse(text).unwrap();
        assert_eq!(p.ops.len(), 7);
        assert_eq!(p.lane, Some(Lane::Optimistic));
        let names: Vec<_> = p.ops.iter().map(Op::name).collect();
        assert_eq!(names[4], "move_fact");
        // Round trip keeps everything.
        assert_eq!(parse(&render(&p)).unwrap(), p);
    }

    #[test]
    fn rejects_unknown_ops_and_fields() {
        let bad_op =
            r#"{"id":"pr-7kq2xmhd","agent":"interviewer","ops":[{"op":"rename","target":"x"}]}"#;
        assert!(parse(bad_op).is_err());
        let bad_field = r#"{"id":"pr-7kq2xmhd","agent":"interviewer","ops":[{"op":"delete","target":"p-7kq2xmhd","why":"x"}]}"#;
        assert!(parse(bad_field).unwrap_err().contains("why"));
        let bad_top = r#"{"id":"pr-7kq2xmhd","agent":"interviewer","ops":[],"extra":1}"#;
        assert!(parse(bad_top).is_err());
    }

    #[test]
    fn targets() {
        assert_eq!(parse_target("sequence"), Ok(Target::Sequence));
        assert_eq!(
            parse_target("s-4hq8nzwt.participants"),
            Ok(Target::NodeField {
                node: "s-4hq8nzwt".into(),
                field: "participants".into()
            })
        );
        assert_eq!(
            parse_target("p-7kq2xmhd#f-3wq8zkpa.ends"),
            Ok(Target::FactField {
                node: "p-7kq2xmhd".into(),
                fact: "f-3wq8zkpa".into(),
                field: "ends".into()
            })
        );
        assert!(parse_target("p-7kq2xmhd").is_err());
        assert!(parse_target("s-4hq8nzwt.turn.wants").is_err());
        assert_eq!(
            parse_item_ref("f-3wq8zkpa"),
            ItemRef::Fact {
                node: None,
                fact: "f-3wq8zkpa".into()
            }
        );
        assert_eq!(
            parse_item_ref("p-7kq2xmhd"),
            ItemRef::Node("p-7kq2xmhd".into())
        );
    }
}
