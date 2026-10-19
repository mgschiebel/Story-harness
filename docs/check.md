# Dependency checker

`skaldsaga check` checks pending proposals before anything reaches the spec. It applies each proposal to an in-memory copy of the spec and checks the result; the spec itself never changes. The committer (build step 8) will reuse the same operation engine to write the result, so check and commit can't disagree about what a proposal does.

## Command

| Command | Output | Notes |
|---|---|---|
| `check [--project DIR] [--proposal ID]...` | JSON summary | Checks every `.json` file in `.harness/inbox/`, or only the IDs named. Refuses (exit 1) without `.harness/`. |

Each verdict is written back into its proposal file, under `check`, and the revisions it examined go into `revisions.checked`. A file that isn't a readable proposal, or whose name doesn't match its ID, is reported as `unreadable` and left untouched.

```json
{
  "outcome": "checked",
  "spec_problems": 0,
  "proposals": [
    {"id": "pr-…", "status": "passed", "lane": "optimistic", "fan_out": 2, "touches_confirmed": false, "findings": []},
    {"id": "pr-…", "status": "failed", "lane": "optimistic", "fan_out": 0, "touches_confirmed": false,
     "findings": [{"severity": "hard", "code": "delete_blocked", "message": "p-… is still referenced by …", "ids": ["…"]}]},
    {"file": "pr-….json", "status": "unreadable", "message": "…"}
  ],
  "not_found": ["pr-…"]
}
```

## Verdicts

- **passed:** no findings. Optimistic lane: ready to commit as unreviewed. Strict lane: waits for a housekeeper scoped pass.
- **flagged:** soft findings only. The writer decides: revise, keep anyway, or not sure yet.
- **failed:** at least one hard finding. It can't be true, so the answer must be revised.

Only problems a proposal **introduces** count. Findings already present in the spec before it are left out, so an old, unrelated problem never blocks a new answer.

## Lanes

Each agent starts in a lane: the interviewer optimistic; the harvester, researcher, weaver and outliner strict; an unknown agent strict (with a soft finding). The housekeeper never proposes changes (hard finding). A proposal escalates to strict when `routing.strict_fan_out` (default 10) or more other nodes read what it changes, or when it touches a confirmed shape decision or that decision's pivot belief. A proposal may ask for the strict lane (`"lane": "strict"`) but never loosen its own.

## Findings

| Code | Severity | Meaning |
|---|---|---|
| `op_failed` | hard | An operation couldn't be applied (names the step). |
| `stale` | hard | Something in `revisions.writes` or `revisions.read` changed or is gone. |
| `broken_reference` | hard | A reference to an ID that doesn't exist. |
| `delete_blocked` | hard | Something still references what the proposal deletes. |
| `wrong_kind` | hard | A reference points at the wrong kind of thing (a place as a POV, a person as a time anchor). |
| `fact_shape` | hard | A belief without a holder, a truth with one, an open question on a settled fact, `ended_by` without `ends`. |
| `contradicts_belief` | hard | A belief contradicts another belief instead of a truth. |
| `timeline_loop` | hard | In-world times loop: something would happen before its own cause. |
| `learns_before_true` | hard | A character learns a fact in a scene pinned before it becomes true. |
| `reads_after_end` | hard | A scene relies on a fact that's pinned as ended before it. |
| `two_reveals` | hard | A fact is revealed more than once in the reading order (alternative scenes don't count). |
| `two_live_values` | hard | An entity would have two live values for a single-valued key (or two for the same target, for a per-target key), and neither ends. |
| `maybe_two_live_values` | soft | As above, but one ends and the timeline doesn't say whether it ends first. |
| `hint_after_reveal` | soft | A hint comes after the reveal in reading order. |
| `reads_superseded` | soft | A scene reads a fact this answer replaces. |
| `reads_reclassified` | soft | A scene reads a fact this answer turns from truth to belief, or back. |
| `housekeeper_edit` | hard | The housekeeper tried to change the spec. |
| `unknown_agent` | soft | Not a known agent; checked in the strict lane. |
| `bad_id` | hard | The proposal ID isn't valid. |

In-world order comes only from anchors (`when`, `true_from`, `ends`): `after` means strictly later, `in` and `at` mean the same moment. Reading order never implies in-world order, because of flashbacks. What the anchors don't pin down is never a hard failure.

## Proposal file

```json
{
  "id": "pr-7kq2xmhd",
  "agent": "interviewer",
  "lane": "optimistic",
  "created": "2026-10-18T12:58",
  "answer": "Mara learns the truth from Jonah's sister",
  "ops": [ … ],
  "revisions": {"writes": {"p-…": 3}, "read": {"sequence.md": "…"}, "checked": {}}
}
```

Unknown keys are rejected. `revisions` maps node and fact IDs to revisions; `sequence.md` and `attributes.md` have no revision and are recorded by a fingerprint of their content.

## Operations

| Op | Shape | Notes |
|---|---|---|
| `create` | `{"node": {front matter}, "notes": "…"}` | Same fields as a node file; `revision` may be left out. IDs must be new. |
| `set` | `{"target": "node.field" or "node#fact.field", "value": …}` | `null` clears an optional field. `id`, `type`, `revision`, `facts` can't be set. `notes` sets the prose notes. |
| `add_fact` | `{"node": "…", "fact": {…}}` | The fact's ID must be new. |
| `add_item` | `{"target": "node.field", "value": …}` or `{"target": "sequence", "value": "s-…", "after"/"before": "s-…"}` | Adds to a list field, or places a scene in the reading order (at the end by default). |
| `remove_item` | `{"target": …, "value": …}` | Removes from a list field or the reading order. |
| `supersede` | `{"target": "node#fact", "replacement": {fact}}` | Replaces the fact (new ID) and moves references to it; scenes that read it are flagged. |
| `delete` | `{"target": "node" or "node#fact"}` | Blocked while anything references it. |
| `merge` | `{"survivor": "…", "merged": "…"}` | Same type only. Facts move, references are redirected, the merged name becomes an alias. |
| `move_fact` | `{"fact": "f-…", "to": "…"}` | Keeps the fact's ID and revision; both nodes' revisions go up. |
| `reclassify` | `{"target": "node#fact", "kind": "belief", "holder": "…", "contradicts": "…"}` | Becoming a belief needs a holder; becoming a truth clears holder and contradicts. |
| `add_attribute` | `{"attribute": {key, relation, inverse, cardinality, summary}}` | Records a new story key in `spec/attributes.md`. |

Revisions are bumped once per proposal by comparing the result with the spec before it: a node's revision when a node-level field changes or a fact moves in or out; a fact's revision when the fact changes. New nodes and facts start at 1.
