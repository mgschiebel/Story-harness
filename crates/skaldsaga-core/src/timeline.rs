//! In-world time, as far as the spec pins it down.
//!
//! In-world time is relative by default (HANDOFF, Node format): a scene
//! happens `after` a fact or another scene, a fact becomes true `in` a scene
//! or `after` something, and ends the same way. This module turns those
//! anchors into a graph of "before" and "same moment" links between points
//! (each scene, each fact's start, each fact's end), and answers one kind of
//! question: is A *provably* before B?
//!
//! The hard checks only fail on relations that are actually pinned down;
//! anything the graph can't decide is left to the housekeeper (HANDOFF,
//! Governance, Dependency checker). Reading order never implies in-world
//! order, because of flashbacks.

use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use crate::model::{NodeBody, TimeAnchor};
use crate::spec::Spec;

/// A moment in in-world time.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Point {
    /// When a scene happens.
    Scene(String),
    /// When a fact becomes true.
    Start(String),
    /// When a fact stops being true (only facts with `ends`).
    End(String),
}

impl Point {
    pub fn id(&self) -> &str {
        match self {
            Point::Scene(s) | Point::Start(s) | Point::End(s) => s,
        }
    }
}

/// "Before" and "same moment" links between points.
#[derive(Debug, Default)]
pub struct TimeGraph {
    /// Outgoing links; `true` means strictly before, `false` at or before.
    edges: HashMap<Point, Vec<(Point, bool)>>,
}

impl TimeGraph {
    pub fn build(spec: &Spec) -> Self {
        let mut g = TimeGraph::default();
        let scenes: HashSet<&str> = spec
            .nodes
            .iter()
            .filter(|n| matches!(n.body, NodeBody::Scene(_)))
            .map(|n| n.id.as_str())
            .collect();
        let facts: HashSet<&str> = spec
            .nodes
            .iter()
            .flat_map(|n| n.facts.iter().map(|f| f.id.as_str()))
            .collect();
        // The point an `after` names: a scene, or a fact's start.
        let point_of = |id: &str| {
            if scenes.contains(id) {
                Some(Point::Scene(id.to_string()))
            } else if facts.contains(id) {
                Some(Point::Start(id.to_string()))
            } else {
                None
            }
        };

        for node in &spec.nodes {
            if let NodeBody::Scene(s) = &node.body
                && let Some(anchor) = &s.when
            {
                g.anchor(Point::Scene(node.id.clone()), anchor, &point_of);
            }
            for fact in &node.facts {
                let start = Point::Start(fact.id.clone());
                if let Some(anchor) = &fact.true_from {
                    g.anchor(start.clone(), anchor, &point_of);
                }
                if let Some(anchor) = &fact.ends {
                    let end = Point::End(fact.id.clone());
                    // A fact can't end before it starts.
                    g.link(start, end.clone(), false);
                    g.anchor(end, anchor, &point_of);
                }
            }
        }
        g
    }

    fn link(&mut self, a: Point, b: Point, strict: bool) {
        self.edges.entry(a).or_default().push((b, strict));
    }

    fn same(&mut self, a: Point, b: Point) {
        self.link(a.clone(), b.clone(), false);
        self.link(b, a, false);
    }

    fn anchor(&mut self, p: Point, anchor: &TimeAnchor, point_of: &impl Fn(&str) -> Option<Point>) {
        match anchor {
            TimeAnchor::After(a) => {
                if let Some(q) = point_of(&a.after) {
                    self.link(q, p, true);
                }
            }
            TimeAnchor::In(i) => self.same(p, Point::Scene(i.scene.clone())),
            TimeAnchor::At(a) => self.same(p, Point::Start(a.at.clone())),
            TimeAnchor::Unknown(_) | TimeAnchor::Date(_) => {}
        }
    }

    /// Searches from `a`; `need_strict` asks for a path with at least one
    /// strict link.
    fn reaches(&self, a: &Point, b: &Point, need_strict: bool) -> bool {
        let mut seen: HashSet<(&Point, bool)> = HashSet::new();
        let mut queue: VecDeque<(&Point, bool)> = VecDeque::from([(a, false)]);
        while let Some((p, strict)) = queue.pop_front() {
            if !seen.insert((p, strict)) {
                continue;
            }
            for (q, s) in self.edges.get(p).into_iter().flatten() {
                let now = strict || *s;
                if q == b && (now || !need_strict) {
                    return true;
                }
                queue.push_back((q, now));
            }
        }
        false
    }

    /// True when the spec pins `a` strictly before `b`.
    pub fn before(&self, a: &Point, b: &Point) -> bool {
        self.reaches(a, b, true)
    }

    /// True when the spec pins `a` at the same moment as `b` or before it.
    pub fn at_or_before(&self, a: &Point, b: &Point) -> bool {
        a == b || self.reaches(a, b, false)
    }

    /// Groups of points that come strictly before themselves: a loop in
    /// the timeline, something happening before its own cause.
    pub fn loops(&self) -> Vec<BTreeSet<String>> {
        let mut out: Vec<BTreeSet<String>> = Vec::new();
        let mut points: Vec<&Point> = self.edges.keys().collect();
        points.sort();
        for p in points {
            if out.iter().any(|g| g.contains(p.id())) || !self.before(p, p) {
                continue;
            }
            // Everything on a loop through p: reachable from p and reaching p.
            let group = self
                .edges
                .keys()
                .filter(|q| self.at_or_before(p, q) && self.at_or_before(q, p))
                .map(|q| q.id().to_string())
                .collect();
            out.push(group);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec;
    use std::path::PathBuf;

    fn harbor() -> Spec {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/specs/harbor");
        spec::load(&dir).unwrap().spec
    }

    #[test]
    fn follows_scene_and_fact_anchors() {
        let g = TimeGraph::build(&harbor());
        let s = |id: &str| Point::Scene(id.into());
        // s-harbgggg happens after s-harbffff.
        assert!(g.before(&s("s-harbffff"), &s("s-harbgggg")));
        assert!(!g.before(&s("s-harbgggg"), &s("s-harbffff")));
        // Mara's belief ends in s-harbffff, so the end is at that scene.
        let end = Point::End("f-trstharb".into());
        assert!(g.at_or_before(&end, &s("s-harbffff")));
        assert!(g.before(&end, &s("s-harbgggg")));
        // Nothing pins the other scenes: reading order isn't in-world time.
        assert!(!g.before(&s("s-harbaaaa"), &s("s-harbhhhh")));
        assert!(g.loops().is_empty());
    }

    #[test]
    fn finds_loops() {
        let mut spec = harbor();
        let node = spec
            .nodes
            .iter_mut()
            .find(|n| n.id == "s-harbffff")
            .unwrap();
        let NodeBody::Scene(s) = &mut node.body else {
            panic!()
        };
        s.when = Some(TimeAnchor::After(crate::model::AfterAnchor {
            after: "s-harbgggg".into(),
            offset: None,
            precision: None,
        }));
        let loops = TimeGraph::build(&spec).loops();
        assert_eq!(loops.len(), 1);
        assert!(loops[0].contains("s-harbffff") && loops[0].contains("s-harbgggg"));
    }
}
