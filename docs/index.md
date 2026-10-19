# Generated index

`skaldsaga index` rebuilds `spec/_index/`: five JSON views of the spec, so an agent loads only the view it needs instead of reading whole nodes. The single committer will run it after every commit; until then it can be run by hand.

Nothing in `_index/` is ever hand-edited. Deleting the folder and rebuilding always restores it, and the folder carries its own `.gitignore`, so it never enters the writer's git history. Output is deterministic (no timestamps, sorted keys): the same spec always gives the same files.

## Command

| Command | Output | Notes |
|---|---|---|
| `index [--project DIR]` | JSON summary | Refuses (exit 1) when the folder has no `.harness/`, and writes nothing. |

```json
{
  "outcome": "rebuilt",
  "files": ["_index/backlinks.json", "_index/attributes.json", "_index/grid.json", "_index/scenes.json", "_index/status.json"],
  "nodes": 27,
  "problems": 1,
  "broken_references": 2
}
```

A file that can't be loaded doesn't stop the rebuild: the index is built from everything that loaded, and the problem is listed in `status.json`.

## The files

### `backlinks.json`

Keyed by whatever is referenced (a node, a fact, or a scene). Each entry says who points at it and through which field. A fact naming a scene (reader hints and reveals, who learns it where, `true_from` or `ends` in a scene) is listed under the scene; the fact already lists its own scenes.

```json
{
  "s-harbffff": [
    {"from": "a-mainharb", "via": "landmark:low_point"},
    {"from": "p-jnahharb", "via": "fact:f-jnspyhrb.reader.revealed"},
    {"from": "s-harbgggg", "via": "when"},
    {"from": "sequence.md", "via": "sequence"}
  ]
}
```

`via` values: node fields by name (`pov`, `location`, `participants`, `coverage`, `reads`, `when`, `evidence`, `pivot`, `arc`, `characters`, `decisions`, `threads`, `viewpoints`), `landmark:<key>` and `beat:<key>` for arc placements, `fact:<fact ID>.<field>` for references inside a fact (a relationship value uses its attribute name, for example `fact:f-….sibling_of`), and `sequence` for the reading order. A node pointing at itself is left out. References to IDs that don't exist go to `status.json` instead.

### `attributes.json`

- `registered`: every key, the starter vocabulary first, then the story's own keys from `spec/attributes.md`, each with `relation` (`property`, `symmetric`, `inverse`, `one_way`, `related`), `inverse` for pairs, `cardinality` (`single`, `many`, `per_target`, or a map by node type), and `source` (`starter` or `story`).
- `in_use`: per node type, every key facts use, with `registered`, `source`, `relation`, `pair_of` (for the inverse side of a pair, such as `child_of`), the `cardinality` that applies to that node type (`null` when there is none), and the fact IDs.

### `grid.json`

One entry per arc, each thread with its four cells (`setup`, `complications`, `crisis`, `resolution`). A cell is `empty` (no summary, no scene), `planned` (a summary but no scene) or `covered` (scenes whose `coverage` points at it, in reading order, alternatives last). Shelved subplots have `counted: false` and stay out of `totals`. Threads whose arc doesn't exist are listed under `unattached`.

### `scenes.json`

- `order`: scene IDs in reading order; `outside_sequence`: alternative scenes.
- `scenes`: every scene, in reading order then the alternatives, with its own fields plus what the facts say happens in it: `writes` (facts that become true here), `ends`, `hints`, `reveals`, `learns` (fact, who, how), and its `landmarks`, `beats` and `movements`.
- `movements`: per arc (the main story and active subplots), its scenes split into the four movements. The end of Setup is the scene where the MC commits, the end of Complications the midpoint, the end of Crisis the low point. Until all three are placed in order (`method: landmarks`), each scene falls back on its `coverage` stage (`method: coverage`), and scenes with no stage for that arc are `unplaced`. A `note` explains when placed landmarks couldn't be used. The main story spans the whole reading order; a subplot spans from its first scene to its last.

Landmark keys, in story order: `normal_world`, `inciting_incident`, `mc_commits`, `rising_stakes`, `midpoint`, `things_fall_apart`, `low_point`, `climax`, `new_normal`.

### `status.json`

`format` (bumped when these shapes change), `built_by`, `counts` per node folder plus facts and scenes in and outside the reading order, `labels` across all facts and shape decisions, `decisions` (`answered`, `kept_simple`, `unanswered`), `grid` totals, `nodes` (per node: type, name, summary, and a derived plain-language status such as "2 provisional, 1 unreviewed"), `problems` (files that couldn't be loaded) and `broken_references`.

## The story's own attribute keys: `spec/attributes.md`

Keys beyond the starter vocabulary live in canon, next to the facts that use them. Only proposals change the file (once the committer exists). A missing file means no custom keys.

```markdown
---
attributes:
- key: mentor_of
  relation: inverse
  inverse: mentee_of
  cardinality: many
  summary: Teaches or guides another person
---

Notes for the writer.
```

Keys are lowercase letters, digits and `_`, starting with a letter. A key that clashes with the starter list or another key in the file, or an inverse pair without its other direction, is reported as a problem and skipped; the rest still load.
