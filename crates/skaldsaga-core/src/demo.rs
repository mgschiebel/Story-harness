//! `skaldsaga demo`: two sample answers for trying `check` and `commit`
//! before the interviewer exists (decided in build step 8).
//!
//! It only runs in a set-up project whose spec is still empty, so it never
//! mixes sample content into a real story. The first answer creates a tiny
//! story (Mara, Jonah, the docks, two scenes) and passes; the second has
//! Mara learn about Jonah's arrest in a scene that happens before the arrest,
//! which the checker refuses.

use crate::fsio::{self, FsError};
use crate::project::Project;
use crate::spec;

/// The sample answers, in the order to try them.
pub const PROPOSALS: [&str; 2] = ["pr-demstart", "pr-demmixup"];

const START: &str = r#"{
  "id": "pr-demstart",
  "agent": "interviewer",
  "created": "2026-10-18T12:00",
  "answer": "Mara is a courier at the east docks. Jonah is arrested after she crosses the checkpoint, and she sees it happen.",
  "ops": [
    {"op": "create", "node": {"id": "p-demmara2", "type": "person", "name": "Mara Vance",
      "aliases": ["Mara"], "summary": "Courier who smuggles letters past the blockade"}},
    {"op": "create", "node": {"id": "p-demjnah2", "type": "person", "name": "Jonah",
      "summary": "Mara's partner on the letter runs",
      "facts": [{"id": "f-demarstd", "kind": "truth", "statement": "Jonah is arrested",
        "true_from": {"in": "s-demarrst"},
        "characters": [{"who": "p-demmara2", "learns_in": "s-demarrst", "how": "saw"}],
        "confidence": "provisional", "review": "unreviewed", "source": "interview"}]}},
    {"op": "create", "node": {"id": "l-demdcks2", "type": "place", "name": "The east docks"}},
    {"op": "create", "node": {"id": "s-demchckp", "type": "scene", "name": "The checkpoint",
      "pov": "p-demmara2", "location": "l-demdcks2", "purpose": "Mara gets the letters past the guards",
      "status": "slot"}},
    {"op": "create", "node": {"id": "s-demarrst", "type": "scene", "name": "The arrest",
      "pov": "p-demmara2", "location": "l-demdcks2", "participants": ["p-demmara2", "p-demjnah2"],
      "when": {"after": "s-demchckp", "offset": "that evening"},
      "purpose": "Mara watches Jonah taken away", "status": "slot"}},
    {"op": "add_item", "target": "sequence", "value": "s-demchckp"},
    {"op": "add_item", "target": "sequence", "value": "s-demarrst"}
  ]
}
"#;

const MIXUP: &str = r#"{
  "id": "pr-demmixup",
  "agent": "interviewer",
  "created": "2026-10-18T12:05",
  "answer": "At the checkpoint, Mara already knows Jonah has been arrested.",
  "ops": [
    {"op": "add_item", "target": "p-demjnah2#f-demarstd.characters",
     "value": {"who": "p-demmara2", "learns_in": "s-demchckp", "how": "told"}}
  ]
}
"#;

/// What `write` did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemoOutcome {
    /// The sample answers are in the inbox.
    Written,
    /// The spec already holds nodes; the demo only runs in an empty story.
    SpecNotEmpty,
    /// A sample answer is already in the inbox or the archive.
    AlreadyWritten,
}

/// Writes the sample answers into the inbox.
pub fn write(project: &Project) -> Result<DemoOutcome, FsError> {
    let loaded = spec::load(&project.spec_dir())?;
    let archive = project.harness_dir().join(crate::commit::COMMITTED_DIR);
    for id in PROPOSALS {
        let name = format!("{id}.json");
        if project.inbox_dir().join(&name).exists() || archive.join(&name).exists() {
            return Ok(DemoOutcome::AlreadyWritten);
        }
    }
    if !loaded.spec.nodes.is_empty() || !loaded.spec.sequence.is_empty() {
        return Ok(DemoOutcome::SpecNotEmpty);
    }
    for (id, text) in PROPOSALS.iter().zip([START, MIXUP]) {
        fsio::write_atomic(&project.inbox_dir().join(format!("{id}.json")), text)?;
    }
    Ok(DemoOutcome::Written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proposal;

    #[test]
    fn sample_answers_parse() {
        for text in [START, MIXUP] {
            let p = proposal::parse(text).unwrap();
            assert!(
                crate::ids::is_kind(&p.id, crate::ids::IdKind::Proposal),
                "{}",
                p.id
            );
        }
    }
}
