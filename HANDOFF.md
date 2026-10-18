# Skaldsaga: Planning Handoff (v7)

*Planning session summary, October 18, 2026 (evening). Supersedes v6 (October 18, late afternoon). Status: design phase, nothing built yet. Companion file: `ARCHITECTURE.md` (one-page sketch, reviewed in the v6 session). The project was called Story Harness until the v6 session; the GitHub repo has been renamed to Skaldsaga.*

## How to use this document

Drop it in the project root (for example as `HANDOFF.md`) and reference it at the start of a Claude Code session, or paste or upload it into a chat session. Suggested opening prompt: "Read HANDOFF.md. Continue from the open items, and grill me one question at a time before we build anything."

Labels used below:

- **Decided**: confirmed by the author.
- **Proposed**: suggested in discussion, not yet explicitly confirmed.
- **Research**: from web searches or documentation; verify before relying on it. Items marked *unverified* have not been checked at all.

## What changed since v6

This session worked through editions (open item 14) from the writer's day-to-day point of view. All of the following are **Decided**; details are in "Editions" under Spec and storage.

- **Editions are separate files; `sequence.md` stays pure.** Chapter, act and update breaks never go into the sequence. Reason: the sequence is canon, so breaks inside it would stale reading-order checks whenever a break moved, and CD would need to write to a canon file.
- **One true canon.** Every edition uses the same facts. An edition may leave scenes out or use alternative scenes, but it can never change what is true.
- **Reader checks run per edition.** Facts, timing and character knowledge are checked once, in canon. What the reader sees (hints, reveals, setups and payoffs, cause and effect between visible scenes) is checked per edition, because each edition has its own scene list.
- **Alternative scenes are ordinary scene nodes** that are not in the main sequence. They don't have to cover the same ground as the scene they replace; the edition just has to pass every check.
- **A retelling that changes canon is a new project,** forked by copying the facts with a note of where they came from (project and version), never synced back.
- **Edition lifecycle:** not published → publishing in progress (serials and anything released in parts) → published. Published editions are frozen snapshots; canon may move on and republishing means a new edition. While in progress, a canon change that touches released groupings marks them "needs republishing" and Skaldsaga strongly recommends bringing them up to current canon.
- **Review snapshots** (beta readers, editors, publishers) belong to not-published editions. Each is pinned to a canon version so feedback can be shown against the exact text the reviewer read. They never trigger republishing.
- **Phase 1 ends with a suggested first edition** (closes the main question in open item 14): the outliner's last step proposes a not-published edition chosen from the writer's goal, clearly labeled as a suggestion. The edition file format is still open.
- **IDs are random instead of counting up** (see Spec and storage), for collision safety if facts later move between projects, and so shuffled scenes never look out of order.
- **Book series noted for a later phase** (see Future phases): books are story-level units, not editions; the series shape is undecided.

## What changed in v6

The v6 session reviewed `ARCHITECTURE.md` (open item 1, now closed) and settled open item 2. All of the following are **Decided** unless marked; details are in the sections named.

- **Name: Skaldsaga** (see "Name"). It is the project, plugin and command name (`/skaldsaga`). Internal role names such as "orchestrator" stay internal and are never shown to the writer.
- **Starting Skaldsaga** (see Architecture): the writer types `/skaldsaga` the first time, which runs setup. Afterwards a SessionStart hook shipped in the plugin opens each session with the recap (also after `/clear`). The hook is guarded: it does nothing unless `.harness/` exists, so it never starts questions in other folders or in an existing project before setup.
- **Existing projects** (see Architecture): setup looks before touching anything and asks the writer how to bring files in, recommending: move originals untouched into an archive folder, copy prose drafts into `manuscript/`. Non-prose notes stay in the archive and are harvested into the spec from there. Folders that clash with Skaldsaga's names go into the archive too. If the folder already uses git, setup asks whether to commit into the writer's repo (recommended, with a labeled "before Skaldsaga" commit) or keep a separate history.
- **Setup pattern:** where writers reasonably differ, setup asks in plain language, recommends a default, and stores the answer in `.harness/settings.json`.
- **Hand-back pattern** (closes open item 2): a sub-agent that needs the writer returns plain-language questions; the interviewer asks them and saves proposals; the orchestrator re-dispatches the sub-agent.
- **Incremental session-open pass:** the housekeeper pass at session start covers only changes since the last cleared pass. Full passes run before weave and outline and on request. *Proposed:* a full pass every N sessions.
- **Models per sub-agent are tunable settings.** The housekeeper (it does judgment work; the scripts are the deterministic part), the completeness rubric, weaver and outliner default to the strong model; the harvester to mid-tier. A cheaper housekeeper default must first pass the evals.
- **Scripts are written for Node**, for room to grow (for example a local web viewer for the manuscript). How they reach the writer (require Node, or ship single executables) is deferred to a future phase, alongside research into embedding the agent harness in a Node app with a web front end.
- **No chapters in the story model.** The story is one ordered sequence of scenes, stored in `spec/sequence.md`. Chapters, acts and serial updates belong to **editions**, groupings laid over the sequence, and a project can hold several. Flashbacks need nothing special (the two clocks already cover them).
- **CI/CD as the long-term shape** (see Future phases): CI makes the story and produces a pinned version; CD runs one pipeline per edition with format-specific checks, can block and send proposals back, and never edits canon.
- **Research recorded** (see Research notes): a fresh Claude Code install guarantees no Node or Python; plugins cannot register the main status line, so setup writes it into the project's `.claude/settings.json`; a skill's `model` field lasts one turn, so the main-thread skills run on the writer's model.

## The project

A set of Claude Code skills (a "harness") that helps a novice aspiring author turn an incomplete story idea into a well-crafted, detailed novel outline with no plot holes. The skills ask questions, build the spec in the background, and teach craft along the way. The goal is a good story, not an officially Dramatica-correct one. Dramatica is inspiration only. The harness will be distributed for use by others.

**Decided**

- Runs in Claude Code. Building begins in future sessions.
- Targets novel-length fiction in this phase, but the story model stays medium-agnostic (one scene sequence, with chapters, acts or serial updates as editions), so plays, serials and other forms can follow.
- This phase delivers only a complete, detailed outline (spec, plot structure, and the scene sequence with beats).
- Modeled on Matt Pocock's wayfinder skill concept: planning across many sessions where each decision reveals what to plan next.
- **Long-term goal (later phases, out of scope now):** a fully automated novel-writing factory, with either assisted writing or automated scene writing for human review, producing publishable scenes and documents. Later phases include writer and review agent pairs and editor agents (pacing, flow, grammar, voice, and so on). The spec layout must be future-proofed for this (see Spec and storage, and Node format).

## Name

**Decided**

- **The project is called Skaldsaga.** It is the open-source project name, the plugin name, and the command the writer types (`/skaldsaga`). The GitHub repo has been renamed from Story-harness to Skaldsaga.
- **Internal role names stay internal.** "Orchestrator," "interviewer," "housekeeper" and the rest remain the names used in these docs and in code, but the writer never sees them. The orchestrator is the `skaldsaga` skill.
- **Why this name:**
  - A *skald* is a Norse poet-storyteller, which gives us the mentor persona (an expert advisor who guides the writer). Like a muse, it also covers later phases (pair and automated writing) without a rename.
  - *Saga* is Old Norse for "story" or "what is told." It keeps the name about story rather than any medium (novels, serials, comics), and everyone can spell it.
  - In modern Icelandic, *skáldsaga* (poet + story) is the everyday word for "novel."
  - Short, one word, no hyphens or apostrophes, so the command matches the name. "Harness" was dropped because it is a technical word.
- **Names rejected and why:** any "muse" name (Subtxt, the Dramatica-based app, already has an AI coach called Muse in the same niche; Plotmuse is an existing company); bare "Skald" (existing AI writing apps Skald Writer and Skald Scribe); Gaelic pairings such as *scéal* (hard to spell, and a cross-language blend).

**Research (web search only, October 18, 2026; GitHub search was not available)**

- No product found using "Skaldsaga." The term appears only in scholarship, where "skaldsagas" are the medieval Icelandic sagas about poets. Recheck GitHub, package registries and trademarks before publishing.
- Known risk: "skald" sounds like "scald," so some people will misspell the command. Consider whether the plugin can also answer to a misspelling.

## Audience and interaction principles

**Decided**

- Users have never written a story and know neither Dramatica nor writing jargon. Every prompt explains in plain English what is being asked and why it matters to the story.
- Socratic style: open-ended questions that guide the writer and explain the impact on the story. Questions are teachable moments that lean into the why.
- Ambiguous answers: record the possibilities as unresolved and let later answers collapse them. Once a question becomes frontier (the next thing worth resolving), ask a plain-language disambiguating follow-up.
- Stuck users: if the question is not frontier, move on, track it, and re-ask when a later answer helps. If it is frontier, rephrase concretely first, then offer contrasting "what if" options.
- Provisional answers matter more than provenance. Placeholders supplied by the user are tracked as provisional too.
- Writers may draft scenes before structure exists. This phase harvests story facts from drafts into the spec. Writing feedback on drafts is parked for a later phase.
- Role: expert advisor that guides the writer and teaches craft.
- **Infer-and-confirm** is the default for the character and plot shape decisions: ask about concrete scenes and people, infer the decision, confirm it back ("it sounds like your hero ends up changing, is that right?"). Inferred answers stay provisional until confirmed. A direct plain-language question is the fallback when the inference is ambiguous and the decision is frontier.
- **No fixed question order.** The orchestrator and the writer's momentum drive the flow. When momentum slows, the orchestrator offers another area to look at.
- **Raise issues while the topic is live.** Flags and conflicts are raised as close as possible to the answer that caused them. Topical questions are easier for the writer to remember, and deferring them to wrap-up forces a context switch that can confuse. This principle drives the live handling of confirmed-decision changes and the rolling housekeeper passes (see Governance).

**Proposed**

- Ask about instances before principles for everything: scenes, people, places first, then infer structural choices and confirm them back.
- Build option sparks from the user's own earlier answers. Ask early about favorite stories ("taste anchors").
- At most one ripple question per answer. A no-deadlock rule: if still stuck after options, set a flagged provisional answer and keep going.
- Hide internal terms (map, tickets, fog). Users see "what we've decided" and "things we'll get to later."
- When momentum slows, suggest the area where confirming one provisional answer would unblock the most downstream work.

## Story-shape layer

The coverage grid checks that every thread has content at every stage. It does not check what the story is arguing. The story-shape layer is the set of decisions that define the story's argument and shape, matching the completeness of Dramatica's 12 essential questions.

**Decided**

- Every shape decision must be answered at least provisionally before the outline step ("complete" requires it).
- Translate the Dramatica jargon into our own plain-English names upfront, since the checker is our own.
- **Theme layer (the four thematic questions):** the writer answers in their own words, not from a fixed menu. Follow-up questions guide them toward non-conflicting answers.
- **Challenger and relationship threads:** add our own shape decisions, one or two per thread (for example: does the challenger change or hold firm; does the bond deepen, break, or stay unresolved), answered in the writer's own words with guiding follow-ups.
- **No hard combination rules.** The checker raises soft "possible issue" warnings when story content seems to clash with a decision (for example, "your hero is steadfast, but scene 12 shows them changing their core belief; which did you mean?").
- **Evidence and pivot (new):** each decision records the scenes and facts its inference came from, and change-type decisions can name a pivot belief (see Node format). Character change is tied to a character learning the truth.

**Proposed working descriptions (wording to be finalized)**

| Area | Working plain-English decision (source Dramatica question) |
|---|---|
| Hero | Does the hero change or hold firm (Main Character Resolve)? |
| Hero | Does the hero need to start something or stop something (Growth)? |
| Hero | Does the hero solve things by acting or by adjusting themselves (Approach)? |
| Hero | Does the hero work step by step or by gut and whole picture (Problem-Solving Style)? |
| Plot | Do events force decisions, or decisions force events (Story Driver)? |
| Plot | Does the story end by a deadline or by running out of options (Story Limit)? |
| Plot | Does the central problem get solved or not (Outcome)? |
| Plot | Is the hero better or worse off at the end (Judgment)? Outcome and Judgment together give the four endings. |
| Theme (own words) | Where the conflict lives; what everyone cares about; the central tension (point and counterpoint); the root problem. |
| Challenger | Our own one or two decisions (see above). |
| Relationship | Our own one or two decisions (see above). |

Note: the exact four theme questions were not confirmed from the official pages (see Research notes).

## Completeness bar

**Decided**

- A "complete" outline requires: all shape decisions answered at least provisionally, no empty cells in the coverage grid, and the five plot-hole checks run clean.
- A rough outline is allowed early, with gaps marked (so Dramatica concepts are never forced on the writer).
- A complete outline may contain provisional answers. Each provisional answer gates the tasks that depend on it. Anything unresolved blocks "complete."

**Proposed**

- A complete outline lists its provisional items as "assumptions to confirm," each with a note on what would settle it.

## Architecture

**Decided**

- Hub and workers, not a linear pipeline. An orchestrator reads story state, recaps the last session in plain language, and guides the human, including suggestions for where to pick up.
- A living story skeleton that changes and grows as questions are answered. A rough outline is allowed early, with gaps marked.
- A housekeeper task that rechecks old answers for conflicts, forgotten details, idea drift, and plot changes. Full passes run as a gate before the weave and outline steps, and whenever the writer asks. Scoped passes run in the background during a topic (see Governance).
- **Session-open pass is incremental (decided this session).** The pass at the start of each session checks only what changed since the last cleared pass (found through the review labels), not the whole spec. The housekeeper already runs as a sub-agent, so main-thread context was never at risk; the saving is token cost and time, which for a full pass grow with the size of the spec every session. *Proposed:* a full pass every N sessions (a setting) catches slow drift that incremental passes can miss, such as the premise wandering across many small answers.
- A primitive shared by interviewer agents for consistency, worked into the question flow ("that affects X, how do you see that changing?").
- A weaving step that combines storyforms into a plot structure. Weaving may reveal a relationship that needs its own storyform, creating a feedback loop back to the map.
- The gap assessor is **not** a separate worker. It is a shared **completeness-check primitive** called directly by the orchestrator, housekeeper, and weaver. The orchestrator turns its findings into frontier questions.
- The interviewer is a separate worker from the orchestrator. The orchestrator owns the session frame (recap, choosing the topic, wrap-up). The interviewer runs the question loop for a chosen topic.
- **Confirmed split:**
  - **Main thread:** the orchestrator (index-only reads, recap) and the interviewer skill (the live conversation).
  - **Sub-agents (non-interactive, short summaries back):** housekeeper, harvester for drafts, weaver, outliner.
  - **Scripts:** deterministic checks (dependency checker, links, status, timelines, setup-and-payoff ledger), the committer, and the index rebuild.
  - **Completeness check:** a primitive with a script half and a rubric half; the rubric half runs as a sub-agent for deep passes.
- **Session boundaries:** each topic ends at a deliberate boundary. The writer clears the chat and the next session opens with the orchestrator's recap. State is saved to disk after every confirmed answer, so nothing lives only in the chat.
- **Session meter:** a small script, not a sub-agent (a sub-agent cannot see the main context). The status line script writes context usage to a project file, and the interviewer checks it after each saved answer. At a first threshold the harness recommends wrapping up at the next natural pause (the writer may decline once). At a second, higher threshold it wraps up regardless. Thresholds are settings. Wrap-up means: finish the current question, save state, write the recap, tell the writer to clear.
- **Models per sub-agent are settings (decided this session).** Each sub-agent's model is a default in `.harness/settings.json` that the writer can tune, so a writer on a budget can turn things down and newer models can be adopted without a code change. Starting defaults:
  - **Scripts:** no model, no tokens.
  - **Housekeeper (all passes) and completeness rubric:** the strong model. The housekeeper is not the deterministic part (the scripts are); it does judgment work such as unearned twists, uncorrected beliefs and premise drift. A cheap pass that misses something still marks facts cleared, and strict-lane agents trust cleared state.
  - **Harvester:** mid-tier. It is mostly careful extraction, and adopting an existing project can mean reading a whole manuscript, so this is where tiering clearly saves.
  - **Weaver and outliner:** strong (creative structural work).
  - **Orchestrator and interviewer:** run in the main conversation on whatever model the writer has selected. A skill's `model` field only lasts one turn, so it cannot pin these (see Research notes).
  - **Lowering a default needs evidence.** The seeded-plot-hole evals (open item 7) test whether a cheaper model catches enough in scoped housekeeper passes; only then is that default lowered.
- **Hand-back pattern for sub-agents that need the writer.** Sub-agents cannot talk to the writer, but the weaver (framework choice) and the outliner may need answers. Such a sub-agent stops and returns a short list of plain-language questions. The interviewer asks them in the main conversation and saves the answers as normal proposals. The orchestrator then re-dispatches the sub-agent, which finds the answers in the spec. Rejected alternative: running the weaver as a main-thread skill, which would fill the chat's context with weaving work and shorten sessions.
- **Scripts are written for Node (decided this session).** Chosen over Python, Go and compiled TypeScript to future-proof later phases: Node can also run local tools such as an embedded web server that shows the manuscript as a web page. A fresh Claude Code install does not include Node (see Research notes), so how scripts reach the writer is open item 13.
- Scripts can ship inside skill folders (a skill is a folder with a markdown file plus scripts).
- **First-run setup** (all four top-level folders, session meter wiring, default thresholds) is part of the orchestrator's first-run path, not a separate skill. The orchestrator handles it in the background so a non-programmer never has to invoke a setup skill.
- **Starting Skaldsaga:** the writer types `/skaldsaga` the first time in a folder; that runs first-run setup and creates `.harness/`. Two first-run paths:
  - **Fresh project** (empty folder): create the folders and start the conversation.
  - **Existing project** (folder already holds the writer's files): look before touching anything, summarize what is there in plain language, then ask the writer how to bring the files in, with a recommended default. Writers will have their own preferences, so this is always asked, never assumed.
- **Recommended default for an existing project:** move the writer's existing files untouched into an archive folder, copy the drafts into `manuscript/`, then set up as normal (the harvester reads the copies). Other options to offer: leave files where they are and record where the drafts live, or move them straight into `manuscript/`.
- **Git in an existing project is a setup-time choice.** If the folder already has git history, setup asks whether Skaldsaga should commit into the writer's existing repo (recommended) or keep a separate history of its own. The answer is stored in `.harness/settings.json`. With the existing repo, Skaldsaga makes one clearly labeled commit before moving anything ("Before Skaldsaga: your files as they were"), giving the writer an undo point. If the folder has no git at all, setup creates it.
- **General pattern (from the two choices above):** where writers will reasonably differ, setup asks in plain language, recommends a sane default, and stores the answer in `.harness/settings.json` so it can be changed later.
- **Only prose goes to `manuscript/`.** Non-prose material in an existing project (character sheets, old outlines, world-building notes, research) stays in the archive only. The harvester reads it from there and proposes spec updates through the normal approval flow, since that information belongs in the spec. `manuscript/` holds only the writer's story text.
- **Later sessions start on their own (guarded hook):** the plugin ships a SessionStart hook (matchers `startup`, `resume`, `clear`) that does nothing unless `.harness/` exists, so installing the plugin never starts questions in a non-Skaldsaga folder or in an existing project before the writer types `/skaldsaga`. Setup does not write any hook into the project. In a Skaldsaga project the hook adds context telling Claude to open with the orchestrator's recap. Firing on `clear` matches the session boundary (clear, then recap). *Research:* plugins can ship hooks, which run wherever the plugin is enabled; SessionStart can add hidden context before Claude's first reply. *Unverified:* whether the hook's `initialUserMessage` field can start the conversation without the writer typing; if not, the writer types anything and gets the recap.

**Proposed**

- The orchestrator recommends one default next step plus "something else."
- Shared primitives: plain-language translation reference, impact lookup (related items via relationship facts, typed fields, the backlinks index, and rules), spec read/write with status, a harvest primitive for drafted material, the coherence checker, and the length primitive.
- The housekeeper never edits on its own. It raises plain-language heads-ups. Keep the original premise and taste anchors as a versioned baseline, and supersede decisions with dated notes rather than overwriting.
- Even a single storyform needs weaving, to interleave its four threads across the book.
- When weaving surfaces a possible new storyform, add it to the map as a fog item automatically and raise it with the user only when it is ready to explore.
- The orchestrator should explain the settings change plainly ("I'm turning on a small progress meter so we know when to take a break") and let the Claude Code permission prompt act as the confirmation. If the writer declines the meter, the manual session boundary still works.

## Governance: how changes reach the spec

Two separate concerns, kept distinct:

- **The pending inbox** keeps unchecked data out of canon, so nothing ever reads or builds on a change that a checker later rejects. Quarantine reinforces this: strict-lane agents, including the housekeeper, read only cleared state.
- **The proposal format** decides how precisely the checkers and the committer can do their jobs: what a proposal touches, when its clearance is stale, and how the interviewer overlays pending changes on the spec.

**Decided**

- **Nothing in the spec is updated until all governance checks (dependency checker and housekeeper) have completed.** This avoids a race where the spec is updated and then the housekeeper rejects the change.
- **Pending inbox:** changes go to a pending inbox on disk. The dependency checker runs on each immediately. The interviewer reads the spec plus the pending inbox so it does not contradict uncommitted answers.
- **Single committer (new, was Proposed):** one committer applies proposals one at a time, bumps node and fact revisions, and rebuilds the generated index folder after each commit. The version check, uniqueness rule, merge handling, and index rebuild all depend on this.
- **Proposal format:** one JSON file per confirmed answer, made of field-level operations. Each proposal records which agent made it (which determines its lane), the writer's answer in their own words (for recaps), and the revisions it was checked against. Full schema and op list in Node format.
- **Dependency checker:** a deterministic script run instantly on every proposal, with three jobs:
  1. **Validity:** referenced IDs exist, references resolve, required fields are present, operations are well-formed. Delete is blocked when anything references the target (new).
  2. **Routing:** counts fan-out (how many nodes read what changed) and detects whether a confirmed decision is touched. This sets the lane and triggers live confirmation.
  3. **Hard ordering and uniqueness checks:** a character cannot learn a fact before it becomes true; the reader cannot learn a fact before it is revealed; an event cannot precede its cause in in-world time; no scene reads a fact after it has ended (new); at most one live fact per entity and attribute at any in-world moment (new). It enforces only relations that are actually pinned down. Loose in-world time it cannot resolve goes to the housekeeper as a possible issue.
- **Housekeeper scoped passes:**
  - **Scope:** the touched nodes plus their readers (anything that references or declares a read of them, found through the backlinks index). Above a cap of N readers (a setting), it recommends a full pass at the next session start instead.
  - **Timing:** rolling background passes. After every few saved answers (a setting), a scoped pass runs as a background sub-agent over the answers since the last pass. Only one pass is in flight at a time. The interviewer raises any flags at the next natural pause while the topic is still live. The end-of-topic step covers only the final few answers.
- **Flagged proposals: the writer decides.** A flagged proposal stays held in the inbox. The interviewer or orchestrator explains the flag in plain language, and the writer chooses:
  - **Revise:** the answer is updated and the proposal is re-checked.
  - **Keep anyway:** the proposal is committed and the flag is recorded as acknowledged.
  - **Not sure yet:** the proposal is committed as provisional and the flag stays open as an assumption to confirm.
  - **Hard script failures** (broken references, ordering or uniqueness violations, blocked deletes) are held and explained the same way, but "keep anyway" is not offered, because they cannot be true.
- **Changes to confirmed decisions are handled live.** When an answer would change a confirmed decision, the interviewer's consistency check catches it and confirms in the moment ("You'd confirmed Mara changes by the end; this sounds like she holds firm. Is that a change of mind?"). If the writer says yes, a housekeeper scoped pass starts immediately in the background. The interviewer steers to topics that do not depend on the changed decision until the pass returns, then raises any flags.
- **Removing and changing things (new):**
  - **Never referenced** (no relationship facts, typed-field references, scene reads, anchors, or evidence in the backlinks index): plain **delete**. Git underneath keeps history. Delete also runs the harvest drift check against drafts, because the index cannot see prose.
  - **Referenced:** delete is blocked and the interviewer asks what changed:
    - **In the story** ("Mara stops trusting Jonah"): the fact gets `ends` and `ended_by`; the change becomes a story moment.
    - **The writer's mind** ("she never trusted him"): **supersede with a required replacement**; scenes that read the old fact are flagged.
  - **Cutting something referenced** (for example, Jonah's sister): hard failures on the scenes that read it force revision first; once nothing references it, it can be deleted. No separate retire state exists.
  - **Series shelf:** unused facts are simply left in place for later books. The writer deletes only what they truly do not want.
  - **Merge** (two entities become one): redirect all references to the survivor, then run the attribute uniqueness check on the combined facts.
  - **Split** (one entity becomes two): no automatic redirect; the interviewer asks about each reference ("In scene 12, is this Jonah or his brother?").
  - **Truth becomes belief** (the reader was misled all along): reclassify the fact's kind in place and flag the scenes that relied on it.
  - **Deleting a scene:** facts anchored to it count as references; re-anchor them first.
  - **Moving a fact to a different home node:** keep its ID and its fact revision; bump both nodes' revisions. Never done as delete plus create.
- **Per-agent lanes:** cheap agents such as the interviewer accept backout costs and commit optimistically. Expensive or isolated parallel agents are held to stricter rigor (checks before commit). Any change to a confirmed decision, or with high fan-out, escalates to the strict lane regardless of which agent made it.
- **Quarantine:** optimistic commits are tagged unreviewed. Strict-lane agents read only cleared state, so a housekeeper rejection can only undo cheap work.
- **Two independent labels** on each fact: writer confidence (confirmed, provisional, unresolved) and review state (unreviewed, cleared).
- **Version check (settled for phase 1, may change in later phases; updated in v5):** each node carries a revision for its node-level fields, and each fact carries its own revision. A proposal records the revisions of (1) the nodes and facts it writes, (2) the nodes and facts its author read to form the answer, and (3) the nodes and facts each checker examined when clearing it. At commit, if any of those revisions has changed, the clearance is stale and the proposal is re-checked. Otherwise it commits, even if unrelated parts of the spec changed. In short: a clearance is only as fresh as everything its checks looked at.
- **Task readiness (future phases):** a task is ready only when every dependency is confirmed. In assisted mode the writer can opt in to "draft anyway"; the draft is tagged with its assumptions, and the housekeeper marks it stale if an assumption changes.
- **Scenes declare reads at the fact level; writes are derived (updated).** A scene lists the fact IDs it relies on. What a scene makes true, hints, or reveals is generated from the facts that point at it. Dependencies and parallel-conflict handling derive from this data.

**Proposed**

- The readiness gate means both checkers have run since the last relevant change (not stale) and there are no unacknowledged blocking findings. Findings the writer has acknowledged stop blocking.
- After drafting, the harvest step compares what a scene was expected to establish with what the draft actually established, and flags undeclared writes (including new attributes such as "a chipped front tooth") as possible spec drift.

## Spec and storage

**Decided**

- Plain files in the project folder, as a folder hierarchy where higher-level index files link to node Markdown files, to minimize context bloat. Not a flat file, and no issue tracker.
- **Organized by story element:** people, places, scenes, shape decisions, threads, each holding one small node per entity. The coverage grid and map are generated index views linking to those nodes. Reason: structure changes a lot during planning and entities mostly do not, so the layout should not be keyed to the most volatile thing.
- **Four top-level folders created at first run:** spec, manuscript (prose), reviews (findings per editor type), and style guide. Empty folders get a one-line README explaining what goes there later. Drafts a writer brings in before any structure exists go in the manuscript folder, where the harvest step reads them.
- **Stable IDs are opaque and type-prefixed:** for example `people/p-0007.md`, `scenes/s-0042.md`, facts `f-0031`. Names live in front matter and index lines, so renaming a character touches one field and breaks no links. IDs never change.
- **IDs are random, not counting up (decided in v7).** Reasons: collision safety if facts later move between projects (series, forks); no implied order, so shuffled scenes never look "out of order"; and agents can create IDs without a shared counter, so a proposal can name a new node before it is committed. The committer rejects duplicates. *Proposed format:* type prefix plus 8 lowercase characters from an alphabet without look-alikes (no `0/o`, `1/l`), for example `s-7kq2xmhd`. The sequential IDs used as examples elsewhere in this document are illustrative only.
- **Generated index folder (new):** `spec/_index/` holds `backlinks.json`, `attributes.json`, `grid.json`, `scenes.json`, and `status.json`. One script rebuilds all of them after every commit. Nothing in it is ever hand-edited; deleting the folder and re-running the script always restores it. Agents load only the file they need. Nodes store only their outgoing references (relationship facts and typed fields); backlinks are generated.
- **Facts are ID'd entries inside entity nodes:** each entity node holds a list of its facts (schema in Node format). A fact involving two entities is stored once on one home node; the generated index shows it from the other side. Scene reads, the setup-and-payoff ledger, and the ordering checks reference fact IDs. Each fact has its own revision (updated in v5), so editing one fact does not stale readers of other facts on the same node.
- **Labels per fact, node status derived:** writer confidence and review state sit on each fact (and on each shape decision's value). A node's status in the index is computed, never stored (for example "2 provisional, 1 unreviewed"). A quarantined fact does not quarantine its whole entity, and readiness can gate on exactly the facts a scene reads.
- **The spec is the source of truth. Flow is one-way (spec to draft).** If a draft introduces a new fact, the harvest step proposes a spec update for the writer's approval, never changing canon silently.
- **Two clocks tracked from the start:** in-world time (relative unless a date is given) and reading order (the order of scene IDs in the sequence file, never stored as position numbers). This lets an automated scene-writing agent see only the spec up to that point in the timeline.
- **The scene is the atomic unit.**
- **The story is one sequence of scenes; chapters belong to editions (decided this session).** Chapters are a written-novel idea, and the harness supports other forms (plays, web serials, comics). So the story itself is a single ordered series of scenes from start to end, and every grouping is presentation:
  - **The sequence** is stored in one file, `spec/sequence.md`: the scene IDs in reading order. Moving a scene is a one-line edit. The single committer serializes edits, so the shared file causes no conflicts, and any reorder correctly stales whatever checked reading order (reveals, knowledge). It is also the one file a writer might open to see the story's shape. Rejected: a sortable position key on each scene (no single place shows the whole story).
  - **Editions** are groupings laid over the same sequence: chapters (optionally inside parts) for a novel, acts for a play, updates with release days for a web serial. A project can hold several editions side by side, and publishing uses one.
  - **Flashbacks need no special structure:** a flashback is a scene whose in-world time is earlier than its place in the sequence (the two clocks above).
  - **Word budgets** apply to stages (pacing) and to each edition's groupings (for example, serial updates of about 2,500 words).
  - *Proposed:* each edition file lives in `spec/editions/` and lists its breaks by scene ID plus labels. Groupings break only at scene boundaries.

### Editions (decided in v7)

- **Separate files, pure sequence.** Each edition is its own file; `sequence.md` never holds breaks. Breaks inside the sequence would bump its revision and stale every reading-order check whenever a break moved, and would make CD write to a canon file. *Proposed:* a generated, read-only view in `spec/_index/` shows the sequence with one edition's breaks, so the writer still gets a single-file picture.
- **One true canon, shared by every edition.** An edition may leave scenes out or use alternative scenes, but facts are the same in every edition.
- **Alternative scenes** are real scene nodes in the spec that are not in the main sequence. They may read and reveal any canon fact. If one would make something new true, that fact enters canon through the normal approval flow and must not contradict anything. An alternative scene does not have to match what the scene it replaces revealed; the edition just has to be a complete story.
- **What is checked where:**
  - **Once, in canon:** what is true, when it is true, and what characters know.
  - **Per edition:** reader hints and reveals, setups and payoffs, and cause and effect between scenes the reader actually sees. Cutting the scene that plants a setup leaves an unpaid payoff in that edition only.
- **Retellings are forks.** If a new version needs canon to change (a genuinely different story), it becomes a new project that imports a copy of the facts with a note of where they came from (project and version). It never ties back to the original.
- **Lifecycle (three stages):**
  1. **Not published.** Planned or in draft. Breaks and groupings change freely; it always builds from current canon.
  2. **Publishing in progress.** For serials and anything released in parts. Each released grouping is pinned to the canon version it was built from. When a canon change touches something already released, those groupings are marked **needs republishing** and Skaldsaga strongly recommends bringing them up to current canon; a republished grouping is re-pinned to the new version. This keeps a live edition consistent for readers who are following along.
  3. **Published.** Finished and frozen: a historical snapshot. It is never re-checked; canon may move on, and Skaldsaga may note "differs from current canon" as information only. Republishing means starting a new edition.
- **The writer marks an edition published deliberately** (for example when a serial ends). A published edition never reopens. A novel usually goes straight from not published to published in a single release.
- **Review snapshots.** Copies handed out for comment (beta readers, an editor, a publisher, advance copies) belong to a not-published edition. Sharing a copy creates a snapshot pinned to its canon version (for example "Beta round 1, sent to 4 readers"). Feedback is filed in `reviews/` tied to that snapshot, so Skaldsaga can show what the reviewer actually read and translate the comment into a question about the current text. Snapshots never trigger republishing. *Proposed:* a soft, information-only heads-up when a change touches text in a late advance copy. Serial writers can use the same mechanism to beta-read an update before release.
- **Phase 1 proposes a first edition.** The outliner's last step suggests one not-published edition chosen from the writer's goal (chapters for a novel, updates for a web serial), labeled as a suggestion the writer can change at any time.
- **Harness machinery (new in v5)** lives in a hidden `.harness/` folder at the project root: `inbox/` (pending proposals), `settings.json`, `meter.json` (written by the status line), and `sessions/` (one plain-language recap per session). First run creates it alongside the four visible folders. Recaps could later be surfaced to the writer as a journal.
- Phase 1 captures a **viewpoint and style node**, asked in plain language and held as provisional (fields in Node format).
- **What-if scene prototypes** are visible variant files inside the project folder, promoted by an explicit step (promotion runs the harvest step). Git stays hidden underneath for history and the agents' parallel workspaces.

**Proposed**

- Index entries with a one-line summary, derived status, and links. A changelog of superseded decisions that is not loaded by default. Nodes about one entity or decision cluster, under about a page. A validation script for broken links and orphaned nodes.
- Scene packets: a compact bundle for a writer agent containing the scene's beats, the relevant people and places, the attribute facts the scene reads, the style rules resolved from its POV, what the reader and POV character know so far, and planting instructions for setups ("establish that the lock is old, don't explain why") rather than the payoff itself.

## Node format (new; all Decided unless marked)

Every node is a Markdown file. Structured data lives in YAML front matter; the body is free prose notes for the writer that no checker relies on.

### Common core (every node)

```yaml
id: p-0007
type: person        # person | place | scene | decision | thread | style | storyform
name: "Mara Vance"
aliases: ["Mara", "the courier"]
summary: "Courier who smuggles letters past the blockade"  # index line
revision: 12        # bumped by the committer when node-level fields change
facts: [...]        # fact entries, schema below
```

- **No node-level links list (v5).** Relationships in the story world are facts (see below). Spec bookkeeping uses typed fields per node type (scene `pov`, `location`, `participants`, `coverage`; thread `characters`; decision `evidence`, `pivot`; storyform `threads`, `decisions`).

### Fact entry

```yaml
- id: f-0031
  kind: truth              # truth | belief
  revision: 3              # bumped by the committer when this fact changes
  statement: "The cellar lock is old and brittle"
  attribute: null          # optional property key, e.g. eye_color
  value: null              # optional property value, e.g. "grey-green", or an entity ID for relationships
  holder: null             # belief only: a person ID or reader
  contradicts: null        # belief only: the truth's fact ID
  question: null           # unresolved only: what's undecided, plain words
  options: []              # unresolved only, optional; empty = TBD
  true_from: {after: f-0012, offset: "3 days", precision: approx}
                           # or {in: s-0042} or {date: ...} or unknown
  ends: {at: f-0040}       # optional, same anchor format
  ended_by: f-0040         # optional, the fact that replaced it
  reader:                  # list of moments; at most one reveal
    - {scene: s-0004, level: hinted}
    - {scene: s-0031, level: revealed}
  characters:
    - {who: p-0007, learns_in: s-0012, how: told}  # saw | told | worked_out
  links: [p-0002]          # other entities this fact involves
  confidence: provisional  # confirmed | provisional | unresolved
  review: unreviewed       # unreviewed | cleared
  source: interview        # interview | harvest:<file> | placeholder
```

Rules:

- **In-world time** is relative by default (anchored to a fact or a scene, with optional offset and precision). An absolute date is used only when the writer gives one.
- **Facts end explicitly.** In-world change is a new fact plus `ends` and `ended_by` on the old one. This is distinct from the writer changing their mind (supersede).
- **Beliefs** use the same time fields; a belief usually ends when its holder learns the truth. The **reader** can be a holder, which covers red herrings.
- **Reader hints and reveals:** a reveal with no earlier hint gets a soft "possible unearned twist"; a hint never revealed or used gets a soft "dangling setup."
- **Unresolved facts** keep one ID so scenes can still declare a read and readiness blocks on them. When settled, a set operation picks the winner and losing options go to history.
- **Properties are facts with an `attribute` key.** The attribute registry (`attributes.json`) lists keys in use per node type with a **cardinality**: single (one live value per entity at any in-world moment, e.g. `eye_color`), many (a list, e.g. `sibling_of`), or per-target (one live value per other entity, e.g. `bond`). The uniqueness check (hard) follows the cardinality. A new key that looks like an existing one is soft-flagged. Attribute writes are declared like any other write.
- **Relationships are facts (v5):** `attribute` names the relation and `value` is the other entity's ID (`attribute: sibling_of, value: p-0002`). Stored once on a home node (the node being discussed when the answer came up). The registry marks each relation symmetric (`sibling_of`, `spouse_of`), an inverse pair (`parent_of`/`child_of`, `located_in`/`contains`; only one direction stored), or one-way (`bond`; each direction is its own fact). The `related` fallback is a fact whose `statement` describes the connection. Knowledge of a relationship is tracked on the fact's `characters` and `reader` lists like any other fact.
- **Detail nodes were dropped in v5.** Per-fact revisions keep a scene that reads only Mara's eye color from going stale when a plot fact about her changes.

### Person and place

No extra fields beyond the common core. Facts (including relationship facts) cover them. Roles such as hero and challenger come from thread `characters` fields.

### Scene

```yaml
pov: p-0007
location: l-0003
participants: [p-0007, p-0002]
coverage: {thread: t-0001, stage: turning_point}
purpose: "Mara discovers Jonah lied"   # works for open slots too
when: {after: s-0011, offset: "next morning"}
beats: ["...", "..."]
reads: [f-0031, f-0050]     # declared: facts this scene relies on
status: outlined            # outlined | drafted | reviewed | approved
```

Writes, hints, and reveals are not stored on the scene. They are generated into `scenes.json` from facts that point at it (`true_from: {in: ...}`, reader moments, character learns).

### Shape decision

```yaml
question: "Does Mara change or hold firm by the end?"
area: hero          # hero | plot | theme | challenger | relationship
choices: [changes, holds_firm]   # empty = writer's own words (theme)
value: changes
options: []         # same unresolved pattern as facts
evidence: [s-0012, f-0044]       # scenes and facts the inference came from
pivot: f-0050       # optional: the belief whose ending marks the change
confidence: provisional
review: unreviewed
```

- **Evidence** shows the writer why, tells the housekeeper what to recheck if a scene is cut or rewritten, and seeds the soft clash warnings. Evidence can point at beliefs as well as truths.
- **Pivot checks:** for "changes," the pivot belief must end, ideally near the climax (an early end gets a soft "change happens too early?"). For "holds firm," the pivot belief is challenged but survives the climax. The scene where the pivot ends is a candidate for the personal thread's turning-point cell.

### Thread

```yaml
kind: personal          # personal | challenger | relationship | big_picture
storyform: sf-0001      # one per map now; future-proofs subplots
characters: [{who: p-0007, role: lead}]   # roles: lead, opposite, etc.
decisions: [d-0001, d-0002]
stages:
  - {stage: starting_condition, summary: "Mara trusts no one", confidence: provisional}
  - {stage: turning_point, summary: null}
```

Coverage grid cell states (generated into `grid.json`): **empty** (no summary, no scene; becomes a frontier question), **planned** (summary, no scene), **covered** (scenes linked via their `coverage` field). The housekeeper can soft-flag scenes that drift from their cell's summary. Stage names are still open (see Open items).

### Viewpoint and style

```yaml
defaults:
  person: third         # first | third | second
  tense: past
  distance: close       # close | distant | omniscient
  tone: "Tense, wry, hopeful underneath"
taste_anchors: ["...", "..."]
viewpoints:
  - {who: p-0007, overrides: {person: first}}
  - {who: p-0002}
voice_refs: []          # links into the style guide folder, later phases
```

Each scene's `pov` resolves its style rules. Voice samples, the flag-and-confirm typo list, and voice features live in the style guide folder, not the spec.

### Storyform

```yaml
status: main            # main | fog | active
threads: [t-0001, t-0002, t-0003, t-0004]   # one per kind; empty while fog
decisions: [d-0001, d-0002]                  # empty while fog
premise:                # writer's own words, versioned for drift checks
  current: "A courier smuggles letters past a blockade..."
  history: [{date: 2026-10-18, text: "..."}]
central_question: "Will Mara get the letters through?"   # what the ending must answer
characters: [p-0007, p-0002]   # fog only: who the subplot involves
```

- **Fog subplots are real storyform nodes (v5)** with `status: fog`, holding only a one-line premise and the characters involved. They get stable IDs that scenes, facts, and the weaver can reference before they are mapped. Mapping one flips it to active and fills its threads and decisions.
- `central_question` feeds the argument-coherence check; `premise.history` is the versioned baseline the housekeeper uses for drift.

### Proposal

```json
{
  "id": "pr-0123",
  "agent": "interviewer",
  "lane": "optimistic",
  "created": "2026-10-18T12:58",
  "answer": "Mara learns the truth from Jonah's sister",
  "ops": [
    {"op": "add_fact", "node": "p-0007", "fact": {"...": "..."}},
    {"op": "set", "target": "p-0007#f-0050.ends", "value": {"in": "s-0031"}},
    {"op": "add_item", "target": "s-0031.participants", "value": "p-0004"}
  ],
  "revisions": {"writes": {}, "read": {}, "checked": {}}   // node and fact revisions
}
```

Facts are addressed by ID (`node#fact.field`), never by list position, so parallel edits cannot shift each other.

**Op list:** `create`, `set`, `add_fact`, `add_item` and `remove_item` (for list-valued typed fields such as `participants`; *Proposed in v5*, replacing `add_link`/`remove_link` now that node-level links are gone), `supersede` (replacement required), `delete` (zero references plus draft check), `merge` (redirect references, then attribute uniqueness check), `move_fact` (keeps ID), `reclassify` (truth to belief or back). Split is handled by the interviewer as per-reference changes.

### Backlinks index

```json
{
  "f-0031": [
    {"from": "s-0012", "via": "reads"},
    {"from": "s-0004", "via": "reader.hinted"},
    {"from": "d-0002", "via": "evidence"}
  ],
  "p-0007": [
    {"from": "s-0031", "via": "pov"},
    {"from": "p-0002", "via": "fact:f-0060.sibling_of"}
  ]
}
```

Keyed by target ID. Each entry records how the target is referenced, which the delete check, housekeeper scope, and impact lookup all need.

## Story model and coherence checking

**Decided**

- Build our own validation as a skill primitive rather than depend on Dramatica's licensed tooling.
- Top priority: a "complete" story that addresses the four major throughlines per storyform.
- Cover all five plot-hole categories: (1) continuity and timeline, (2) knowledge and motivation, (3) setups without payoffs and payoffs without setups, (4) argument coherence (the ending answers the story's central question), (5) cause-and-effect gaps between scenes.
- One storyform per map for now. Potential subplots are recorded in the spec as fog items for future mapping sessions.
- Hard timeline, knowledge ordering, and attribute uniqueness violations are caught per proposal by the dependency checker (see Governance).
- **Reader-facing checks run per edition (v7).** Reveals, hints, setups and payoffs, and cause and effect between visible scenes depend on each edition's scene list; truth, timing and character knowledge are checked once in canon.
- **Knowledge checks use truth and belief facts (new)**, for example "Mara acts on knowing Jonah lied in scene 9, but only learns it in scene 14." The housekeeper soft-flags false beliefs that are never corrected or paid off, and facts that end with no cause (possible cause-and-effect gap).

**Proposed**

- Plain labels for the four threads: personal (the hero's inner struggle), challenger (the character pressing the hero), relationship (the bond between them), and big picture (the external problem and stakes).
- A coverage grid: threads by stages, each cell with a plain requirement (starting condition, escalating conflict, turning point, resolution). Add an interlock check: the climax resolves more than one thread, and every thread bears on the central question.
- Deterministic scripts for links, status, timelines, and a setup-and-payoff ledger. Rubric-guided model checks reported as "possible issues," not verdicts. A small argument-coherence rule set written in our own vocabulary, which tests the ending against the story-shape decisions.
- Evals using stories with seeded plot holes to test the checker (the skill-creator skill supports evals).
- Avoid reproducing Dramatica's proprietary terminology, dictionary, or matrices. Check the licensing page if borrowing anything specific. This is not legal advice.

## Weaver, frameworks, and length

**Decided**

- The weaver chooses a story framework (basic four-part novel, hero's journey, save the cat, and so on) based on the storyforms by asking the human questions, and teaches the tradeoffs.
- Stage granularity per thread will be settled when designing the weaver.
- A length primitive guides word-count goals from industry standards and is reusable by other skills.
- Authors may aim at traditional publishing, self-publishing, or just finishing a story. The skills ask the writer and adapt.
- Version one includes a serial length profile. Full serial support (a parent map plus arc maps) is deferred and flagged as a near-term phase update.

**Proposed**

- Frameworks sit on top of the coverage grid. The grid stays framework-agnostic and the framework supplies named beats at typical positions. Name frameworks but write original descriptions, since some beat sheets are trademarked or book-specific.
- A length data file with ranges by genre and publishing path, with sources and dates. It outputs a target length, a word budget per stage and per edition grouping (chapter, update, act), subplot-fit checks, pacing flags, and per-scene estimates. All of it is advice with reasons, never hard rules.

## Future phases (recorded ideas, not in scope now)

**Decided as ideas**

- **Automated multi-agent harness** with the outline split into discrete tasks with dependency blockers. A task moves from blocked to ready as its dependencies are settled. This also allows what-if prototyping and parallel agent activity.
- **Voice development:** the writer writes sample text for each voice (narration per viewpoint character, dialogue per character, and so on) so agents have a style to work from and any automated writing stays in the writer's own voice.
- Raw voice-sample files are kept as raw files. A basic spelling and grammar check runs on them before the style guide is built, **flag-and-confirm**: the raw file is never edited, suspected errors are listed, the writer marks each as a typo or a deliberate choice, typos are excluded from style analysis, and deliberate choices are recorded as voice features (for example, "Mara drops her g's and speaks in fragments").
- Writer and review agent pairs, plus editor agents for pacing, flow, grammar, and voice.
- The phase 1 version-check rule may need revisiting once parallel scene agents exist.
- **Parallel attribute safety (new):** if two writer agents propose the same entity attribute, the single committer applies one and the other fails the uniqueness check and is re-checked. No silent drift.
- **Series shelf (new):** unused facts carry over as material for later books.
- **Book series (raised in v7, not designed).** A book is a story-level unit with its own arc, central question, ending and shape decisions, so book splits do not belong in edition files (an omnibus could later be an edition spanning books). Phase 1 keeps the doors open by using random IDs and keeping storyforms as separate nodes, so one storyform could later cover a range of the sequence. Questions for a later phase:
  - One project with books as divisions of a single sequence, or one project per book sharing a fact library?
  - If separate projects, how are facts shared or migrated between them (unlike a fork, a series needs them kept in step)?
  - How a published book constrains later books (likely the same pattern as a serial in progress constraining canon changes).
- **CI/CD framing for the whole pipeline (decided this session as the long-term shape):**
  - **CI = making the story.** Proposals are like pull requests, the dependency checker and housekeeper are the tests, and the single committer is the merge. Outline (this phase), drafting and editing are CI stages. "Green" means a complete, approved story.
  - **The build artifact** is a pinned, git-tagged version of the approved sequence and manuscript. Every CD run builds from one pinned version, so all editions come from the same text. *Refined in v7:* for editions released in parts, each released grouping records its own pinned version (see Editions under Spec and storage).
  - **CD = one pipeline per edition** (the editions above are its deployment targets). Each holds format-specific agents and checks: Kindle (EPUB validity, table of contents), print (trim size, margins, page count, spine width), web serial (update lengths, release schedule, platform formatting), and so on.
  - **CD can block and send findings back to CI.** Some format needs are really story needs (a serial update should end on a hook, which is pacing). CD fails like a broken deploy and files proposals in the inbox; it never patches canon itself (the same one-way rule as the harvester).
  - **Deferred to the CD phase:** whether an edition may carry its own extra text (serial recaps, chapter titles, front matter) or every word must come from CI. The author leans toward allowing CD-specific rewrites. *Settled in v7:* editions may cut scenes and use alternative scenes (which are spec scene nodes), but never change canon facts.
- **Web front end (new, research):** a friendlier home than a terminal or VS Code for non-programmers. Research whether the agent harness itself can be embedded in a Node app that also serves a web interface to the agents and the manuscript. Leads to check: the Claude Agent SDK (runs Claude Code's agent loop from TypeScript or Node), how users would authenticate (API key versus subscription), Node single executable applications, and the licensing and usage terms for distributing such an app. The script delivery choice (open item 13) is decided alongside this.

**Proposed**

- An agent-drafted scene never becomes a voice sample unless the writer approves it, so the style does not drift toward the machine's habits.
- A writer sub-agent can be held to its scene packet: limit its tools, block reads outside the packet folder with a hook, and omit project instruction files (`omitClaudeMd`).
- Run parallel scene tasks in separate workspaces (sub-agents can run in a temporary git worktree). Scenes that write the same node are serialized. A scene that follows another in reading order waits for its outcome.

## Research notes (verify before relying)

- **Dramatica.** A storyform spans four throughlines (main character, influence character, relationship, objective story). The full theory has about seventy-five objective appreciations. The 12 essential questions are four about character, four about plot, and four about theme. Eight are binary and four ask you to pick from "quads" (the thematic ones), which is roughly 65,536 combinations before the questions constrain each other. Confirmed from sources: Resolve (change or steadfast), Growth (start or stop), Approach (do-er or be-er), Problem-Solving Style (linear or holistic), Driver (action or decision), Limit (timelock or optionlock). Outcome and Judgment (success or failure, good or bad) are the other two binary ones and together give four endings. *The exact four theme questions were not listed on the pages retrieved.* The Overall Story Concern is one of them, and the Issue layer is the basis of the central theme. The community uses "story weaving" for combining storyforms, and forum posts suggest no formal weaving theory exists.
- **Narrative Context Protocol (NCP).** An open interchange schema on GitHub (3.0.0-rc.1 at last check). It validates structure only and does not include valid-combination logic. Dramatica's engine, semantic validation, and certification are licensed separately. Since the project is moving to its own checker, NCP is optional.
- **Wayfinder.** Pocock's skill for projects too big for one session, built from a map (accumulated decisions), tickets (grilling, prototype, research, task), and sessions, with an orchestrator handling handoffs. User-invoked only. This comes from interviews and write-ups; the actual SKILL.md has not been read. Related skills: grill-me, /to-spec, /to-tickets.
- **Serial length (anecdotal).** Chapters run about 1,500 to 4,000 words, commonly 2,000 to 3,000. Royal Road rewards frequent updates, and many authors post weekly or twice weekly. Traditional and self-publishing length norms have not been researched yet.
- **Claude Code subagents (official docs, checked October 2026).**
  - `AskUserQuestion` is removed from every subagent. A subagent starts with a fresh context window, works independently, and returns only a summary. The docs recommend the main conversation for tasks needing frequent back-and-forth. This is why the interviewer cannot be a sub-agent.
  - Skills load into the main conversation context (they cost context while active), whereas subagents run in an isolated window.
  - A custom agent can run as the whole session (`claude --agent <name>`), and the `tools` field can restrict which subagent types it may spawn (`Agent(type, ...)`).
  - A subagent's frontmatter supports `tools` and `disallowedTools`, `hooks` (such as `PreToolUse` to block certain actions), `omitClaudeMd`, `skills` preloading, `isolation: worktree`, `memory`, and `maxTurns`.
  - Nested subagents are possible but limited by depth, and there is a concurrent subagent limit. Rolling housekeeper passes and live confirmed-decision passes run as background sub-agents alongside the main-thread interview.
- **Claude Code status line (official docs).** The status line script receives JSON on stdin including `context_window.used_percentage` (input tokens only), `context_window_size`, and `session_id`. It runs locally and does not use API tokens. A script can write that value to a file. *No documented way was found for a hook or skill to clear a session automatically.* *Unverified:* whether a plugin can ship or register the status line; whether hooks receive the context percentage.
- **Claude Code runtime and plugin facts (official docs, checked October 18, 2026):**
  - **No script runtime is guaranteed.** The recommended native installer ships Claude Code as a standalone binary; it brings neither Node nor Python (only the npm install path needs Node 22+). On Windows, Git for Windows is optional; without it Claude Code runs commands through PowerShell. Python on macOS is not guaranteed either (general knowledge, not from the docs: `python3` there is often a stub that prompts to install developer tools).
  - **Plugins cannot register the main status line.** A plugin's `settings` only honor `agent` and `subagentStatusLine`. The status line can be set in project settings (`.claude/settings.json`), so first-run setup must write it there, behind the permission prompt. On Windows the status line runs through Git Bash or PowerShell, and file paths need forward slashes. `context_window.used_percentage` counts input tokens only and may be null early in a session.
  - **A skill's `model` field lasts one turn only.** It applies for the rest of the current turn; the session model resumes on the next prompt. So the orchestrator and interviewer effectively run on the writer's chosen model. Skills also support `context: fork` with `agent`, which runs the skill in a sub-agent instead.
  - **Plugin facts:** components are namespaced under the plugin name; skills can bundle scripts in any language and reference them with `${CLAUDE_SKILL_DIR}`; hook commands get `${CLAUDE_PLUGIN_ROOT}` and `${CLAUDE_PROJECT_DIR}`; a top-level `bin/` folder blocks installation through claude.ai and Cowork, so bundled executables belong elsewhere (for example `scripts/`). A plugin's `settings.agent` can make an agent the main-thread agent, but it would apply in every project where the plugin is enabled, so it does not suit Skaldsaga's guarded start.
- **Anthropic harness guidance** ("Effective harnesses for long-running agents"): agents work in discrete sessions with no memory between them, so use a progress file plus git history to orient each fresh session, track features in JSON with a pass/fail flag, work one feature per session, and use separate checkers because agents judge their own work poorly.

## Open items

1. *Closed:* `ARCHITECTURE.md` reviewed this session (see What changed since v5).
2. *Closed:* sub-agents that need the writer use the hand-back pattern (see Architecture).
3. Finalize plain-English names for the story-shape decisions and write our own challenger and relationship decisions. Verify the four theme questions.
4. Settle the coverage-grid stages after the framework design.
5. Design the weaver and outliner in detail: macro plan versus scene beats, open slots described by purpose, and outline deepened act by act.
6. Specify the housekeeper's rubric checks (what it actually looks for in a scoped pass) and the completeness-check rubric. Several soft checks are now named (unearned twist, dangling setup, uncorrected belief, fact ending with no cause, early pivot, scene drifting from its cell summary, similar attribute keys).
7. Plan the evals with seeded plot holes, including a comparison of housekeeper models on scoped passes (to decide whether a cheaper default is safe). Also plan tests for the gating lanes, quarantine, version-checked commit, held-proposal flow, rolling passes, uniqueness rule (all three cardinalities), delete blocking, and merge.
8. Research traditional and self-publishing length norms for the length data file.
9. Read the wayfinder SKILL.md for patterns worth reusing.
10. *Partly closed:* plugins cannot ship the main status line, so first-run setup writes it into the project's `.claude/settings.json` (see Research notes). Still unverified: hook access to context usage (no longer needed), and the SessionStart `initialUserMessage` field. Also: what happens when the writer already has a personal status line, since the project setting would replace it. Choose starting values for all settings: meter thresholds, rolling-pass interval (every N answers), and the reader cap that escalates to a full pass.
11. Check the Dramatica licensing page before borrowing any specific term.
12. Starter relation vocabulary for the attribute registry (family, place, bond, related) with each relation's symmetry and cardinality. Confirm the `add_item`/`remove_item` ops.
13. *Deferred to a future phase:* how the Node scripts reach the writer (require a Node install, or ship single executables). Decide together with the web front end research in Future phases.
14. *Partly closed in v7:* phase 1 does propose a first edition (see Editions under Spec and storage). Still open: the edition file format, including breaks by scene ID, labels, lifecycle status, per-grouping status (planned, released, needs republishing) and pinned versions, alternative-scene substitutions and cuts, and review snapshots. Also: when a scene moves in the sequence, does a break follow the scene it is anchored to, or keep its position?
15. How reviewer and reader feedback is stored in `reviews/` and turned into questions about the current text (mostly later phases; the snapshot link is decided).
16. How forking a project for a retelling works in practice (later phase; the rule is decided).

## Suggested next steps

Start with the edition file format (open item 14). Then the story-shape names and theme questions (item 3), and the coverage-grid stages with the weaver and outliner design (items 4 and 5).
