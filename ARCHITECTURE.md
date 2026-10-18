# Skaldsaga: Architecture Sketch (draft)

*October 18, 2026. Companion to `HANDOFF.md` (v5), which holds the reasoning and full schemas. Everything here reflects Decided items unless marked* **Proposed***. The project name Skaldsaga and the entry-point skill name `skaldsaga` are Decided; other names of skills, scripts and files are working names. Internal role names (orchestrator, interviewer, housekeeper and so on) are never shown to the writer.*

## Components

### Main thread (talks to the writer)

| Component | Kind | Job |
|---|---|---|
| **orchestrator** (skill name: `skaldsaga`) | skill (entry point; the writer types `/skaldsaga`) | First-run setup; session open (recap from the last session file, start a full housekeeper pass, suggest one next step plus "something else"); chooses the topic; dispatches sub-agents; session wrap-up. Reads only `spec/_index/` and `.harness/`, never whole nodes. |
| **interviewer** | skill | The live question loop for one topic. Socratic, infer-and-confirm, plain language. Reads spec nodes plus the pending inbox. Writes one proposal per confirmed answer. Raises flags at natural pauses. Checks the meter after each saved answer. |

### Sub-agents (no writer contact; short summary back)

| Sub-agent | Job |
|---|---|
| **housekeeper** | Full pass (session start, gate before weave and outline) or scoped pass (touched nodes and facts plus their readers; rolling every N answers; immediately after a confirmed-decision change). Reads cleared state only. Never edits; returns flags. |
| **harvester** | Reads drafts in `manuscript/`, proposes spec updates for new facts, flags drift against declared scene reads. |
| **weaver** | Chooses a framework with the writer (see open item 2), interleaves the storyforms' threads across the book, may surface a new fog storyform. |
| **outliner** | Turns the woven plan into chapters (scene ID groupings), scenes, and beats. |
| **completeness-rubric** | The deep, model-judged half of the completeness check. |

### Scripts (deterministic; ship inside skill folders)

| Script | Job |
|---|---|
| `init` | First-run scaffold: four visible folders with one-line READMEs, `.harness/`, default settings, status line wiring. |
| `check` | Dependency checker: validity, routing (fan-out, confirmed-decision touch, lane), hard ordering and uniqueness checks (cardinality-aware). |
| `commit` | The single committer: applies one proposal, version check, bumps node and fact revisions, then calls `index`. |
| `index` | Rebuilds everything in `spec/_index/`. |
| `meter` | Status line script writes context usage to `.harness/meter.json`; a helper reads it against the thresholds. |
| `complete` | Script half of the completeness check: shape decisions answered, grid cells, setup-and-payoff ledger, timeline and knowledge ordering. |

### Shared primitives (reference files and helpers loaded by skills and sub-agents)

Plain-language glossary (our names for every internal concept), consistency and impact lookup (via the backlinks index), spec read with pending-inbox overlay, proposal writer, length guide (data file plus rules), harvest rules.

## Folder layout

```
my-novel/
  spec/
    people/       p-0001.md ...
    places/       l-0001.md ...
    scenes/       s-0001.md ...
    decisions/    d-0001.md ...
    threads/      t-0001.md ...
    storyforms/   sf-0001.md ... (main, active, and fog subplots)
    chapters.md   ordered scene ID groupings          (Proposed: file name)
    style.md      viewpoint and style node
    _index/       backlinks, attributes, grid, scenes, status (generated)
  manuscript/     prose drafts and what-if variants
  reviews/        later phases
  style-guide/    later phases
  .harness/
    inbox/        pending proposals, one JSON each
    settings.json thresholds, rolling-pass interval, reader cap
    meter.json    written by the status line
    sessions/     one recap per session
```

Distributed as a Claude Code plugin named `skaldsaga`, bundling the skills, agent definitions and scripts. **Proposed**, and it depends on verifying that a plugin can register the status line (open item 10).

## Call flows

### Session

1. Writer starts Claude Code in the project and the orchestrator loads. On first run, `init` runs, with a plain explanation before the status line permission prompt.
2. Orchestrator reads the latest `.harness/sessions/` recap and `_index/status.json`, starts a full housekeeper pass in the background, and gives the recap plus one suggested topic.
3. Writer picks a topic. The interviewer runs the loop (next flow).
4. Wrap-up when the topic ends or the meter says so: finish the question, save, run the end-of-topic scoped pass over the last few answers, write the session recap, tell the writer to clear the chat.

### One answer

```
writer answers -> interviewer confirms back
  -> proposal JSON into .harness/inbox/
  -> check (instant)
       hard failure      -> held; explain; revise only
       optimistic lane   -> commit as unreviewed -> index
       strict lane       -> held until a housekeeper scoped pass clears it -> commit -> index
  -> every N answers: background scoped housekeeper pass over unreviewed facts
       clean   -> facts marked cleared
       flagged -> interviewer raises at next pause: revise / keep anyway / not sure yet
  -> meter check: first threshold suggests wrapping up, second wraps up
```

A change to a confirmed decision is caught by the interviewer's consistency check, confirmed live, escalated to the strict lane, and starts a scoped pass immediately. The interviewer steers to unrelated topics until it returns.

### Drafts

Writer drops prose in `manuscript/` -> orchestrator dispatches the harvester -> proposals go into the inbox like any answer -> the interviewer walks the writer through approving them.

### Toward the outline

1. Orchestrator runs `complete` and, for depth, the completeness rubric. Gaps become frontier questions for the interviewer.
2. Full housekeeper pass as a gate.
3. Weaver (with the writer's framework choice) produces the woven plan; any new storyform goes in as fog.
4. Full housekeeper pass as a gate, then the outliner writes chapters, scenes and beats as proposals.
5. A rough outline with gaps marked is allowed at any point; "complete" requires every shape decision answered at least provisionally, no empty grid cells, and clean plot-hole checks.

## Open in this sketch

- How the weaver and outliner get answers from the writer (HANDOFF open item 2).
- The chapters file name and format.
- Plugin packaging and status line registration (open item 10).
