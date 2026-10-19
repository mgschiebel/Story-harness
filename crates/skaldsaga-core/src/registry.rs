//! The attribute registry: which property and relation keys a story uses,
//! how each relation works, and how many live values each may have.
//!
//! The registry has two parts (HANDOFF, Node format, Relationship vocabulary):
//!
//! - the **starter vocabulary**, built into Skaldsaga, and
//! - **the story's own keys**, kept in `spec/attributes.md` (decided in the
//!   build step 6 session). They are story vocabulary, so they live in canon next to
//!   the facts that use them, and go back in history with them. The file is
//!   changed only through proposals, like any other spec file.
//!
//! `spec/attributes.md` is Markdown with YAML front matter:
//!
//! ```text
//! ---
//! attributes:
//! - key: mentor_of
//!   relation: inverse
//!   inverse: mentee_of
//!   cardinality: many
//!   summary: Teaches or guides another person
//! ---
//!
//! Notes for the writer.
//! ```
//!
//! The generated `_index/attributes.json` combines both parts with the keys
//! actually in use.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::model::NodeType;
use crate::node_file;

/// File name of the story's own keys, inside `spec/`.
pub const ATTRIBUTES_FILE: &str = "attributes.md";

/// How a key relates its node to the fact's value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Relation {
    /// A property such as `eye_color`; the value is plain text.
    Property,
    /// Holds both ways (`sibling_of`); stored once.
    Symmetric,
    /// One of a pair (`parent_of` / `child_of`); only one direction is stored.
    Inverse,
    /// Each direction is its own fact (`bond`).
    OneWay,
    /// The `related` fallback; the fact's statement describes the connection.
    Related,
}

/// How many live values an entity may have for a key at any in-world moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cardinality {
    Single,
    Many,
    PerTarget,
}

/// One cardinality for every node type, or a different one per node type
/// (`located_in` is single for a person but many for a place). Node types not
/// listed in the per-type form have no uniqueness rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CardinalitySpec {
    All(Cardinality),
    ByType(IndexMap<NodeType, Cardinality>),
}

impl CardinalitySpec {
    /// The cardinality for one node type, if there is a rule for it.
    pub fn for_type(&self, node_type: NodeType) -> Option<Cardinality> {
        match self {
            CardinalitySpec::All(c) => Some(*c),
            CardinalitySpec::ByType(map) => map.get(&node_type).copied(),
        }
    }
}

/// One registered key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttributeDef {
    pub key: String,
    pub relation: Relation,
    /// Inverse pairs only: the key for the other direction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inverse: Option<String>,
    pub cardinality: CardinalitySpec,
    /// A plain-language line saying what the key means.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// Where a registered key comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Starter,
    Story,
}

fn def(
    key: &str,
    relation: Relation,
    inverse: Option<&str>,
    cardinality: CardinalitySpec,
    summary: &str,
) -> AttributeDef {
    AttributeDef {
        key: key.to_string(),
        relation,
        inverse: inverse.map(str::to_string),
        cardinality,
        summary: Some(summary.to_string()),
    }
}

/// The starter vocabulary (HANDOFF, Relationship vocabulary).
pub fn starter() -> Vec<AttributeDef> {
    use Cardinality::{Many, PerTarget, Single};
    use CardinalitySpec::{All, ByType};
    use Relation::*;
    let located = ByType(IndexMap::from([
        (NodeType::Person, Single),
        (NodeType::Item, Single),
        (NodeType::Group, Many),
        (NodeType::Place, Many),
    ]));
    vec![
        def(
            "parent_of",
            Inverse,
            Some("child_of"),
            All(Many),
            "Is a parent of",
        ),
        def(
            "sibling_of",
            Symmetric,
            None,
            All(Many),
            "Is a brother or sister of",
        ),
        def("spouse_of", Symmetric, None, All(Single), "Is married to"),
        def(
            "partner_of",
            Symmetric,
            None,
            All(Many),
            "Is a romantic partner of (unmarried)",
        ),
        def("friend_of", Symmetric, None, All(Many), "Is a friend of"),
        def("rival_of", Symmetric, None, All(Many), "Is a rival of"),
        def(
            "works_for",
            Inverse,
            Some("employs"),
            All(Many),
            "Works for",
        ),
        def(
            "member_of",
            Inverse,
            Some("has_member"),
            All(Many),
            "Belongs to a group",
        ),
        def(
            "bond",
            OneWay,
            None,
            All(PerTarget),
            "How one feels about another: trusts, fears, loves, resents",
        ),
        def(
            "located_in",
            Inverse,
            Some("contains"),
            located,
            "Is in a place",
        ),
        def("owns", Inverse, Some("owned_by"), All(Many), "Owns"),
        def(
            "related",
            Related,
            None,
            All(Many),
            "Some other connection, described in the fact",
        ),
        def(
            "strength",
            Property,
            None,
            All(Single),
            "What this character does well",
        ),
        def(
            "flaw",
            Property,
            None,
            All(Single),
            "What holds this character back",
        ),
    ]
}

/// A registered key as found by [`Registry::lookup`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Found<'a> {
    pub def: &'a AttributeDef,
    pub source: Source,
    /// True when the key is the inverse side of a pair (`child_of`).
    pub is_inverse_side: bool,
}

/// The starter vocabulary plus the story's own keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Registry {
    defs: Vec<(AttributeDef, Source)>,
}

impl Registry {
    /// Builds the registry from the story's own keys, which have already
    /// been checked by [`parse_attributes_file`].
    pub fn new(story: &[AttributeDef]) -> Self {
        let defs = starter()
            .into_iter()
            .map(|d| (d, Source::Starter))
            .chain(story.iter().cloned().map(|d| (d, Source::Story)))
            .collect();
        Registry { defs }
    }

    /// Every registered key, starter vocabulary first.
    pub fn defs(&self) -> impl Iterator<Item = (&AttributeDef, Source)> {
        self.defs.iter().map(|(d, s)| (d, *s))
    }

    /// Finds a key, including the inverse side of a pair.
    pub fn lookup(&self, key: &str) -> Option<Found<'_>> {
        self.defs.iter().find_map(|(def, source)| {
            let is_inverse_side = def.inverse.as_deref() == Some(key);
            (def.key == key || is_inverse_side).then_some(Found {
                def,
                source: *source,
                is_inverse_side,
            })
        })
    }
}

/// The front matter of `spec/attributes.md`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AttributesFront {
    #[serde(default)]
    attributes: Vec<AttributeDef>,
}

fn valid_key(key: &str) -> bool {
    let mut chars = key.chars();
    chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// Parses `spec/attributes.md`. Returns the keys that are fine and a
/// plain-language problem for each one that isn't (a bad key never stops
/// the good ones from loading).
pub fn parse_attributes_file(text: &str) -> (Vec<AttributeDef>, Vec<String>) {
    let front = match node_file::split_front_matter(text) {
        Ok((front, _notes)) => front,
        Err(e) => return (Vec::new(), vec![e.0]),
    };
    let parsed = match serde_norway::from_str::<Option<AttributesFront>>(front) {
        Ok(Some(p)) => p,
        Ok(None) => return (Vec::new(), Vec::new()),
        Err(e) => return (Vec::new(), vec![format!("front matter: {e}")]),
    };

    let mut good: Vec<AttributeDef> = Vec::new();
    let mut problems = Vec::new();
    for d in parsed.attributes {
        match validate_new(&good, &d) {
            Ok(()) => good.push(d),
            Err(p) => problems.push(p),
        }
    }
    (good, problems)
}

/// The explanation written below the keys when Skaldsaga creates the file.
pub const ATTRIBUTES_NOTES: &str = "\
Your story's own kinds of facts and relationships, beyond the ones Skaldsaga
knows already. Skaldsaga keeps this list up to date as you answer questions.
";

/// Renders `spec/attributes.md` for the story's keys, keeping the notes
/// below the front matter.
pub fn render_attributes_file(defs: &[AttributeDef], notes: &str) -> Result<String, String> {
    #[derive(Serialize)]
    struct Front<'a> {
        attributes: &'a [AttributeDef],
    }
    let yaml = serde_norway::to_string(&Front { attributes: defs }).map_err(|e| e.to_string())?;
    let mut out = format!("---\n{yaml}---\n");
    let notes = notes.trim();
    if !notes.is_empty() {
        out.push('\n');
        out.push_str(notes);
        out.push('\n');
    }
    Ok(out)
}

/// Checks a new story key against the starter list and the story's existing
/// keys: a usable name, an inverse exactly when it's a pair, and no clash.
pub fn validate_new(existing: &[AttributeDef], d: &AttributeDef) -> Result<(), String> {
    let names: Vec<&String> = std::iter::once(&d.key).chain(d.inverse.as_ref()).collect();
    if let Some(bad) = names.iter().find(|k| !valid_key(k)) {
        return Err(format!(
            "'{bad}' isn't a usable key (lowercase letters, digits and '_', starting with a letter)"
        ));
    }
    if d.relation == Relation::Inverse && d.inverse.is_none() {
        return Err(format!(
            "'{}' is an inverse pair but doesn't name the other direction ('inverse:')",
            d.key
        ));
    }
    if d.relation != Relation::Inverse && d.inverse.is_some() {
        return Err(format!(
            "'{}' names an inverse, but only inverse pairs have one",
            d.key
        ));
    }
    if d.inverse.as_ref() == Some(&d.key) {
        return Err(format!("'{}' can't be its own inverse", d.key));
    }
    let starter = starter();
    for (list, by) in [
        (starter.as_slice(), "Skaldsaga's starter list"),
        (existing, "this story's keys"),
    ] {
        for other in list {
            for taken in std::iter::once(&other.key).chain(other.inverse.as_ref()) {
                if names.contains(&taken) {
                    return Err(format!("'{taken}' is already defined in {by}"));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starter_keys_are_unique_and_well_formed() {
        let mut seen = std::collections::HashSet::new();
        for d in starter() {
            for k in std::iter::once(&d.key).chain(d.inverse.as_ref()) {
                assert!(valid_key(k), "{k}");
                assert!(seen.insert(k.clone()), "{k} twice");
            }
            assert_eq!(d.relation == Relation::Inverse, d.inverse.is_some());
        }
    }

    #[test]
    fn lookup_finds_both_sides_of_a_pair() {
        let reg = Registry::new(&[]);
        let parent = reg.lookup("parent_of").unwrap();
        assert!(!parent.is_inverse_side);
        let child = reg.lookup("child_of").unwrap();
        assert!(child.is_inverse_side);
        assert_eq!(child.def.key, "parent_of");
        assert_eq!(child.source, Source::Starter);
        assert!(reg.lookup("eye_color").is_none());
        let located = reg.lookup("located_in").unwrap().def;
        assert_eq!(
            located.cardinality.for_type(NodeType::Person),
            Some(Cardinality::Single)
        );
        assert_eq!(
            located.cardinality.for_type(NodeType::Place),
            Some(Cardinality::Many)
        );
        assert_eq!(located.cardinality.for_type(NodeType::Scene), None);
    }

    #[test]
    fn parses_story_keys_and_reports_bad_ones() {
        let text = "\
---
attributes:
- key: mentor_of
  relation: inverse
  inverse: mentee_of
  cardinality: many
  summary: Teaches or guides another person
- key: eye_color
  relation: property
  cardinality: {person: single}
- key: Debt
  relation: one_way
  cardinality: per_target
- key: sibling_of
  relation: symmetric
  cardinality: many
- key: owes
  relation: inverse
  cardinality: many
- key: mentee_of
  relation: property
  cardinality: single
- key: rank
  relation: property
  inverse: rank_of
  cardinality: single
---

Notes.
";
        let (good, problems) = parse_attributes_file(text);
        let keys: Vec<_> = good.iter().map(|d| d.key.as_str()).collect();
        assert_eq!(keys, ["mentor_of", "eye_color"]);
        assert_eq!(problems.len(), 5, "{problems:#?}");
        for needle in [
            "'Debt' isn't a usable key",
            "'sibling_of' is already defined in Skaldsaga's starter list",
            "'owes' is an inverse pair but doesn't name",
            "'mentee_of' is already defined in this story's keys",
            "'rank' names an inverse",
        ] {
            assert!(problems.iter().any(|p| p.contains(needle)), "{needle}");
        }

        let reg = Registry::new(&good);
        let found = reg.lookup("mentee_of").unwrap();
        assert_eq!(found.source, Source::Story);
        assert!(found.is_inverse_side);
    }

    #[test]
    fn empty_and_broken_files() {
        assert_eq!(parse_attributes_file("---\n---\n"), (vec![], vec![]));
        let (good, problems) = parse_attributes_file("no front matter");
        assert!(good.is_empty());
        assert_eq!(problems.len(), 1);
        let (_, problems) = parse_attributes_file("---\nattributes: [{key: x}]\n---\n");
        assert!(problems[0].contains("front matter"), "{problems:?}");
    }
}
