# Skaldsaga: Planning Handoff (v9)

*Planning session summary, October 18, 2026 (late night). Supersedes v8 (October 18, night). Status: design phase, nothing built yet. Companion file: `ARCHITECTURE.md` (one-page sketch, reviewed in the v6 session). The project was called Story Harness until the v6 session; the GitHub repo has been renamed to Skaldsaga.*

## How to use this document

Drop it in the project root (for example as `HANDOFF.md`) and reference it at the start of a Claude Code session, or paste or upload it into a chat session. Suggested opening prompt: "Read HANDOFF.md. Continue from the open items, and grill me one question at a time before we build anything."

Labels used below:

- **Decided**: confirmed by the author.
- **Proposed**: suggested in discussion, not yet explicitly confirmed.
- **Research**: from web searches or documentation; verify before relying on it. Items marked *unverified* have not been checked at all.

## What changed since v8

A grilling session on the coverage-grid stages and the weaver and outliner (open items 4 and 5). All of the following are **Decided**; details are in "Coverage grid and movements" and "Weaver, outliner and frameworks."

- **Fixed four-stage grid for every thread** (closes open item 4): **Setup, Complications, Crisis, Resolution.** The grid stays framework-agnostic; frameworks layer named beats on top, so switching frameworks never invalidates answers or changes what "no empty cells" means.
- **Stages align across a storyform's threads by default**, giving the book four shared movements. Misalignment is a soft "possible pacing issue," explained once and respected.
- **Subplots float independently** of the main storyform but keep four aligned stages internally. A subplot's minimum is its own dramatic question, stakes, and at least one thread with all four stages; other threads accept "keep it simple."
- **Two tiers of beats, all soft:** a small set of universal **landmarks** owned by the grid (inciting incident, midpoint, low point, climax and others), and **framework beats** supplied by the chosen framework. Inferred and confirmed, never asked cold; missing ones become suggestions, never completeness blockers.
- **Movement boundaries are derived from landmark scenes**, never stored, so `sequence.md` stays pure.
- **The outline is layered:** a skeleton pass across the whole book, then deepening one movement at a time, starting with Setup. Layers set the work order, not a depth limit: detail the writer raises early is captured at whatever depth it arrives and becomes a pin the skeleton builds around.
- **Framework choice:** inferred any time and confirmed when obvious; if the writer is unsure, it waits for the weaver's late recommendation. Four-part is the default, switching is free, and "no framework" is valid.
- **The skeleton is sized by story need, not word counts.**
- **Removed from phase 1: the suggested first edition and the length primitive** (supersedes the v7 decision). Phase 1 has no prose, so word counts would be guesses; word budgets belong to editions and move to a later phase, where the edition step teaches length during revision. Phase 1 keeps only relative pacing notes in scene counts.
- **Fog subplots at weave time:** each is mapped, shelved, or folded into an existing thread, chosen by the writer.
- **Promote to subplot** stays in phase 1: the reverse of fold-in, for any thread content, writer-started, using existing operations.
- **Stage readings written for all four threads** (main plot and challenger added).
- **Landmarks and framework beats are stored on the storyform**, one entry per landmark, not as scene tags.
- **The weaver produces the skeleton directly**; no separate woven plan. The outliner only deepens.
- **Scenes gain a "turn"** (what the POV character wants, what gets in the way, how it ends differently), which catches flat scenes and feeds cause-and-effect checks.
- **Scene-count pacing heuristic** set (expected share from the framework; soft note when a movement is under half or over double it). Open item 5 is closed.
- **Skaldsaga stands on its own.** Dramatica was the starting point; the shape decisions and completeness criteria are now our own, in our own words.
- **The theme layer collapses to the thematic question.** "Where the conflict lives" and "what everyone cares about" are dropped (they were Dramatica's Domain and Concern chain, already not adopted in item 17); the root problem moves into the optional "what's really wrong" story point.
- **The 12 shape decisions are final, worded in plain language** using the story's own names, with the craft term taught afterward (see the table in Story-shape layer). Open item 3 is closed.
- **Optional story points need no new node types:** stakes sit with the dramatic question; strength and flaw are person facts; "what's really wrong" is a belief fact plus the truth it contradicts; pressure along the way is facts with a `role` tag; backstory is facts true before the first scene. Open item 17's node fields are settled.
- **Soft checks split between script and model.** Countable checks run free and instant in the `complete` script; the housekeeper keeps only checks that need judgment.
- **Flags are budgeted:** the housekeeper reports everything, the interviewer raises at most one or two per natural pause by impact, and the rest wait in a queue that must be cleared before weaving and outlining.
- **The completeness rubric makes four judgments:** cells meet their stage, decisions have support, the ending answers both questions, and the threads interlock (the interlock check is now Decided). Open item 6 is closed at design level.

## What changed in v8

A short side session revisited the script language, which had been Node since v6. Details are in Architecture, Future phases, Research notes and open items 13 and 18.

- **Decided: scripts move from Node to Rust.** The Rust source lives in the repo, and what the writer installs carries compiled binaries for each supported operating system. Reasons:
  - **Nothing for the writer to install.** A fresh Claude Code install guarantees neither Node nor Python (see Research notes). A compiled binary just runs, which matters for writers who have never used a terminal.
  - **Fast startup.** The status line script runs constantly, and a compiled binary starts in milliseconds.
  - **Types for the spec schema.** Facts, the two clocks, confidence and review labels, and scene reads and writes become typed structures, so the checker, committer and index rebuild are harder to get subtly wrong. The compiler also catches mistakes that coding agents would otherwise ship.
  - **A path to the long-term product:** eventually a single Rust binary with everything inside, including a friendly interface that uses Claude through the API. The local web viewer that motivated Node can be served from Rust too.
- **Proposed: one binary with subcommands** (`init`, `check`, `commit`, `index`, `meter`, `complete`) instead of one executable per script. Build it as a core library (working name `skaldsaga-core`) plus a thin command-line front end, so the future standalone app wraps the same core instead of rewriting it.
- **Proposed: compiled binaries stay out of the main git history.** CI builds every target and publishes them (GitHub Releases, and possibly a built distribution branch that the plugin installs from). How the right binary reaches the writer is open item 13.
- **Folder name caution.** The idea discussed was a `bin` directory, but a top-level `bin/` folder blocks plugin installation through claude.ai and Cowork (see Research notes), so the binaries need another home, for example `scripts/bin/` or inside a skill folder.
- **Proposed: the session meter is the pilot.** It is small but exercises the whole delivery chain: status line wiring, cross-platform builds, first-run delivery, and signing.
- **Known costs, accepted for now:** a build and release pipeline for macOS (Apple Silicon and Intel), Linux and Windows; unsigned binaries trigger macOS Gatekeeper and Windows SmartScreen warnings, which would scare novice writers, so signing likely needs an Apple developer account and Windows signing work; a higher bar for open-source contributors than scripts; and slower iteration while the schema is still changing.
- **Future front end note.** The Claude Agent SDK is aimed at TypeScript and Python (verify), so a Rust app would either run its own agent loop over the Claude API or drive Claude Code as a subprocess. Calling the API directly also raises the question of API keys and per-token billing versus the writer's Claude subscription. Both are recorded under Future phases.

## What changed in v7

This session worked through editions (open item 14) from the writer's day-to-day point of view. All of the following are **Decided**; details are in "Editions" under Spec and storage.

- **Editions are separate files; `sequence.md` stays pure.** Chapter, act and update breaks never go into the sequence. Reason: the sequence is canon, so breaks inside it would stale reading-order checks whenever a break moved, and CD would need to write to a canon file.
- **One true canon.** Every edition uses the same facts. An edition may leave scenes out or use alternative scenes, but it can never change what is true.
- **Reader checks run per edition.** Facts, timing and character knowledge are checked once, in canon. What the reader sees (hints, reveals, setups and payoffs, cause and effect between visible scenes) is checked per edition, because each edition has its own scene list.
- **Alternative scenes are ordinary scene nodes** that are not in the main sequence. They don't have to cover the same ground as the scene they replace; the edition just has to pass every check.
- **A retelling that changes canon is a new project,** forked by copying the facts with a note of where they came from (project and version), never synced back.
- **Edition lifecycle:** not published → publishing in progress (serials and anything released in parts) → published. Published editions are frozen snapshots; canon may move on and republishing means a new edition. While in progress, a canon change that touches released groupings marks them "needs republishing" and Skaldsaga strongly recommends bringing them up to current canon.
- **Review snapshots** (beta readers, editors, publishers) belong to not-published editions. Each is pinned to a canon version so feedback can be shown against the exact text the reviewer read. They never trigger republishing.
- *Superseded in v9 (removed from phase 1):* **Phase 1 ends with a suggested first edition** (closes the main question in open item 14): the outliner's last step proposes a not-published edition chosen from the writer's goal, clearly labeled as a suggestion, using a minimal edition file (breaks anchored by scene ID).
- **IDs are random instead of counting up** (see Spec and storage), for collision safety if facts later move between projects, and so shuffled scenes never look out of order.
- **Story-shape naming started (open item 3):** a terminology ladder (craft term, then abbreviation, then names); "main character"/MC; "challenger"; MC and POV character taught as different ideas; threads named main plot, inner journey, challenger, relationship.
- **Dramatic and thematic questions** are tracked on every storyform, inferred rather than asked cold, and block weaving and outlining until answered at least provisionally.
- **"Keep it simple" is a valid answer** for the thematic question, three of the MC's four decisions, story driver and limit, the challenger and relationship decisions, and the challenger thread itself. Always required: the dramatic question, the stakes, whether the MC changes or holds firm, outcome and judgment.
- **Dramatica research done** (see Research notes): the four theme questions are confirmed as a narrowing chain from a proprietary table, so our own-words theme approach stands; the full storyform (about 75 story points, 92 in one tally) was reviewed for novice value.
- **Stakes join the required core**, which is now: the dramatic question, the stakes, whether the MC changes or holds firm, and the ending type (outcome and judgment). Strength and flaw, pressure along the way (costs, small wins, warning signs, required steps), want versus need, and backstory are adopted as optional depth (open item 17).
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
- **Scripts are written for Node** *(superseded in v8: Rust)*, for room to grow (for example a local web viewer for the manuscript). How they reach the writer (require Node, or ship single executables) is deferred to a future phase, alongside research into embedding the agent harness in a Node app with a web front end.
- **No chapters in the story model.** The story is one ordered sequence of scenes, stored in `spec/sequence.md`. Chapters, acts and serial updates belong to **editions**, groupings laid over the sequence, and a project can hold several. Flashbacks need nothing special (the two clocks already cover them).
- **CI/CD as the long-term shape** (see Future phases): CI makes the story and produces a pinned version; CD runs one pipeline per edition with format-specific checks, can block and send proposals back, and never edits canon.
- **Research recorded** (see Research notes): a fresh Claude Code install guarantees no Node or Python; plugins cannot register the main status line, so setup writes it into the project's `.claude/settings.json`; a skill's `model` field lasts one turn, so the main-thread skills run on the writer's model.

## The project

A set of Claude Code skills (a "harness") that helps a novice aspiring author turn an incomplete story idea into a well-crafted, detailed novel outline with no plot holes. The skills ask questions, build the spec in the background, and teach craft along the way. The goal is a good story, not an officially Dramatica-correct one. Dramatica is inspiration only. The harness will be distributed for use by others.

**Decided**

- Runs in Claude Code. Building begins in future sessions.
- Targets novel-length fiction in this phase, but the story model stays medium-agnostic (one scene sequence, with chapters, acts or serial updates as editions), so plays, serials and other forms can follow.
- This phase delivers only a complete, detailed outline (spec, plot structure, and the scene sequence with beats). Editions, word budgets and the length primitive are not part of phase 1 (v9).
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
- **Terminology ladder (decided in v7):** introduce the common craft term a writer might meet in books or online, explained plainly (teaching the craft), but avoid terms that are too technical (for example "protagonist"). Once taught, shorten to the common abbreviation, then switch to the story's own names as soon as they exist. Applied to the central character: "main character" with an explanation, then "MC," then the character's name ("Does Mara change by the end?").
- **The challenger keeps that name (decided in v7)** for the character who presses on the MC's way of seeing things (Dramatica's influence character). A teaching aside mentions the related terms a writer may meet: foil (a character who contrasts with the MC; ours actively pushes back), impact character, and antagonist (whoever opposes the MC's goal, who may or may not be the challenger).
- **Main character and POV character are taught as different ideas (decided in v7).** The POV character is whose eyes a scene is seen through (one or many); the MC is whose inner struggle the story argues (usually one per storyform). Other POV characters' arcs may become subplots. Ladder: "point of view character," then "POV character," then names. Timing follows infer-and-confirm: each described scene notes whose eyes it is through, and a second viewpoint triggers the teaching moment and a check on which one is the MC.
- **Thread names (decided in v7):** the **main plot** (the external problem and stakes; "external plot" taught as the related term), the MC's **inner journey** ("internal arc" taught as the related term), the **challenger**, and the **relationship**. Main plot and inner journey give the writer the classic outer-versus-inner split used in most craft advice. Internal kind values in the thread format may stay as they are.
- **Two big questions, tracked on every storyform (decided in v7):**
  - **The dramatic question:** will the MC achieve the goal? (A common craft term.) Answered by the ending's outcome.
  - **The thematic question:** what the story examines, for example "Is loyalty worth lying for?" Answered by how the ending feels (better or worse off, and at what cost). It is the writer's own words for the theme layer's central tension.
  - **The threads examine the thematic question from different sides:** the MC lives one answer, the challenger embodies another, the relationship is where they collide, and the main plot shows what each costs at scale. The coherence checker tests that every thread bears on the thematic question and that the ending answers both questions.
  - **Asked late, inferred first.** Ask directly only if the writer raises it; otherwise infer once a few scenes and choices exist and confirm ("Your story keeps circling whether Mara should trust anyone. Is that the big question?"). Never ask "what's your theme?" cold.
  - **Blocker:** both questions must be answered (at least provisionally, like other shape decisions) before weaving or outlining can start. The orchestrator steers toward them as that stage approaches. For the thematic question, "keep it simple" is a valid answer (see below).
- **"Keep it simple" is a valid answer (decided in v7).** A writer may say they are writing a simple, fun story and do not want to deal with the deeper layers. The goal is a good story, not a Dramatica-correct one.
  - **What can be opted out of:** the thematic question, three of the MC's four decisions (start or stop, approach, problem-solving style) and two of the four plot decisions (story driver, story limit), all still inferred and offered when scenes make them obvious, the challenger and relationship decisions, and the challenger thread itself (some simple stories have none; without a challenger there is no relationship thread either). **Always required:** the dramatic question (even a simple story needs to know what the MC wants and whether they get it) the stakes (what happens if the MC fails), whether the MC changes or holds firm (the core of the inner journey), and the ending type: outcome (is the problem solved) and judgment (is the MC better or worse off), which together give the four classic endings.
  - **How it works:** the decision gets an explicit value (for example `not_explored`) recorded as the writer's choice, so it counts as answered and does not block weaving or completeness. Coverage-grid cells for an opted-out thread are not counted as empty.
  - **Explain once, then respect it:** Skaldsaga says briefly what the writer would gain, then stops raising it. Checks that depend on an opted-out decision are skipped, not failed. The writer can revisit it any time; if later scenes strongly suggest a theme anyway, Skaldsaga may mention it once, as an offer.
- **Challenger and relationship decisions (draft, inferred and confirmed, never asked cold):** does the challenger change or hold firm by the end (usually the opposite of the MC); does the bond between the MC and the challenger deepen, break, or stay unresolved. Both accept "keep it simple." Final wording is decisions 11 and 12 in the table below (v9).
- **Theme layer (collapsed in v9):** the thematic question is the theme layer's only decision, answered in the writer's own words, not from a fixed menu, with follow-ups guiding toward a non-conflicting answer. Dropped: "where the conflict lives" and "what everyone cares about" (Dramatica's Domain and Concern; their useful content appears in the main plot's setup and stakes). The root problem moved into the optional "what's really wrong versus what the MC thinks" story point.
- **Skaldsaga's criteria stand on their own (v9).** Dramatica was the starting point; the decisions below and the completeness bar are now our own and need no Dramatica concepts.
- **Challenger and relationship threads:** add our own shape decisions, one or two per thread (for example: does the challenger change or hold firm; does the bond deepen, break, or stay unresolved), answered in the writer's own words with guiding follow-ups.
- **No hard combination rules.** The checker raises soft "possible issue" warnings when story content seems to clash with a decision (for example, "your hero is steadfast, but scene 12 shows them changing their core belief; which did you mean?").
- **Evidence and pivot (new):** each decision records the scenes and facts its inference came from, and change-type decisions can name a pivot belief (see Node format). Character change is tied to a character learning the truth.

**The 12 shape decisions (Decided in v9; wording final)**

Questions use the story's own names as soon as they exist (shown here with Mara as the main character and Jonah as the challenger). Craft terms are taught only after the plain question. Most decisions are inferred and confirmed rather than asked ("It sounds like Mara is a different person by the end. Is that right?"); the plain question is the fallback when an inference is ambiguous and the decision is frontier.

| # | Plain question | Taught afterward as | Required? |
|---|---|---|---|
| 1 | By the end, has Mara become a different person, or is she still the same at heart? | The character arc; changes or holds firm | Required |
| 2 | Is Mara's trouble something she keeps doing that she needs to quit, or something she won't do that she needs to start? | Want versus need | Optional |
| 3 | When things go wrong, does Mara try to fix the situation, or does she try to fix herself? | Active versus reflective character | Optional |
| 4 | Does Mara figure things out one step at a time, or does she go with her gut? | Problem-solving style | Optional |
| 5 | What is Mara trying to do, and what's the worst that happens if she fails? | The dramatic question and the stakes | Required |
| 6 | In your story, do things happen to Mara that force her to choose, or do her choices make things happen? | Plot-driven versus character-driven | Optional |
| 7 | Does the story end because time runs out, or because there's nowhere left to turn? | The ticking clock | Optional |
| 8 | Does Mara get what she was after? | The outcome | Required |
| 9 | Is Mara happier or better off when it's over? | The ending's judgment; with 8, the four classic endings (happy, bittersweet, tragic, hollow win) | Required |
| 10 | Underneath it all, what is your story wondering about? | The theme (the thematic question) | Optional |
| 11 | Does Jonah end up seeing things Mara's way, or does he stick to his own? | The foil or impact character | Optional |
| 12 | By the end, are Mara and Jonah closer, broken apart, or still unsettled? | The relationship arc | Optional |

- **Required** matches the core: changes or holds firm, dramatic question with stakes, outcome, judgment. **Optional** accepts "keep it simple" and is still inferred and offered when scenes make it obvious.
- 11 and 12 disappear when the story has no challenger.
- The earlier working names (Resolve, Growth, Approach, Problem-Solving Style, Driver, Limit, Outcome, Judgment) are internal history only and never shown to the writer.

## Completeness bar

**Decided**

- A "complete" outline requires: all shape decisions answered at least provisionally (including the dramatic and thematic questions, which also gate the start of weaving), no empty cells in the coverage grid, and the five plot-hole checks run clean.
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
- **Scripts are written in Rust (decided in v8, replacing Node from v6).** Source in the repo; the writer gets compiled binaries for their operating system, so no runtime has to be installed (a fresh Claude Code install includes neither Node nor Python; see Research notes). Rust also gives fast status line startup, typed spec structures, and a path to a single standalone binary in later phases. A local web viewer for the manuscript, the reason Node was picked, can be served from Rust as well. *Proposed:* one binary with subcommands (`init`, `check`, `commit`, `index`, `meter`, `complete`), built as a core library plus a thin command-line front end. How the binaries reach the writer is open item 13; the build and release pipeline is open item 18.
- Scripts can ship inside skill folders (a skill is a folder with a markdown file plus scripts). For compiled binaries this means a per-platform executable inside the plugin, never in a top-level `bin/` folder (see Research notes).
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
- Shared primitives: plain-language translation reference, impact lookup (related items via relationship facts, typed fields, the backlinks index, and rules), spec read/write with status, a harvest primitive for drafted material, and the coherence checker. (The length primitive moved to a later phase in v9.)
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
  - **Word budgets** apply to each edition's groupings (for example, serial updates of about 2,500 words), in the later phase that builds editions (v9). Phase 1 paces by scene counts per movement only.
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
- *Superseded in v9:* ~~Phase 1 proposes a first edition.~~ Editions are not built in phase 1. The rules and minimal file format below stay decided for the phase that first needs editions (likely drafting or CD), where an edition step also runs the word-budget check and teaches length during revision.
- **Reorders and breaks.** Canon cares only about scene order and the ripple effects of a move. Breaks are anchored by scene ID, with no rules about following or staying put: published editions and released groupings are pinned and unaffected (a move touching released groupings falls under "needs republishing"), and a not-published edition is marked as needing a fresh look, after which the edition flow checks it and proposes new breaks if needed.
- **Phase 1 edition file (minimal):**

  ```yaml
  id: e-4hq8nzwt
  name: "Novel, first draft"
  kind: novel            # novel | serial | play ...
  status: not_published  # not_published | in_progress | published
  groupings:
    - {label: "Chapter 1", starts_at: s-7kq2xmhd}
    - {label: "Chapter 2", starts_at: s-p3rwe9ka}
  ```

  Cuts, alternative scenes, per-grouping release status and pinned versions, and review snapshots are added in the phases that need them; the format leaves room for them.
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
  role: null               # v9, optional: cost | small_win | warning | required_step (pressure along the way)
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
coverage: {thread: t-0001, stage: crisis}
purpose: "Mara discovers Jonah lied"   # works for open slots too
when: {after: s-0011, offset: "next morning"}
beats: ["...", "..."]
reads: [f-0031, f-0050]     # declared: facts this scene relies on
turn:                       # v9: the scene's change; inferred and confirmed
  wants: "Get the letters past the checkpoint"    # POV character's goal in this scene
  against: "The guard recognizes her"             # what gets in the way
  ends: "She gets through, but Jonah is arrested" # how it ends differently than it began
status: outlined            # slot | outlined | drafted | reviewed | approved (slot added in v9)
```

Writes, hints, and reveals are not stored on the scene. They are generated into `scenes.json` from facts that point at it (`true_from: {in: ...}`, reader moments, character learns). Landmark and framework-beat roles are also generated into `scenes.json`, from the storyform's `landmarks` and `beats` maps (v9).

- **Slots (v9):** the weaver's skeleton creates scenes with `status: slot`, holding at least `purpose` and `coverage`, and optionally a partial `turn`. Deepening fills the rest and moves them to `outlined`.
- **The turn (v9):** goal, conflict and outcome (taught with plain names; "scene and sequel" and "value shift" are related craft terms). A scene whose end matches its beginning gets a soft "possible flat scene: what changes here?" One scene's `ends` should lead into a later scene's `wants`, which supports the cause-and-effect check.

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
stages:               # always these four keys (v9)
  - {stage: setup, summary: "Mara trusts no one", confidence: provisional}
  - {stage: complications, summary: null}
  - {stage: crisis, summary: null}
  - {stage: resolution, summary: null}
```

Coverage grid cell states (generated into `grid.json`): **empty** (no summary, no scene; becomes a frontier question), **planned** (summary, no scene), **covered** (scenes linked via their `coverage` field). The housekeeper can soft-flag scenes that drift from their cell's summary. Stage keys are settled in v9 (see Coverage grid and movements). A thread opted out with "keep it simple" has no counted cells.

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
status: main            # main | fog | active | shelved (shelved Proposed in v9: fog kept out of this outline)
threads: [t-0001, t-0002, t-0003, t-0004]   # one per kind; empty while fog
decisions: [d-0001, d-0002]                  # empty while fog
premise:                # writer's own words, versioned for drift checks
  current: "A courier smuggles letters past a blockade..."
  history: [{date: 2026-10-18, text: "..."}]
dramatic_question: "Will Mara get the letters through?"  # plot: answered by the outcome
thematic_question: "Is loyalty worth lying for?"         # theme: answered by how the ending feels
framework: {value: hero_journey, confidence: provisional} # v9; value may be undecided or none
landmarks:              # v9: tier 1, one entry each; null = not found yet (becomes a suggestion)
  inciting_incident: {scene: s-7kq2xmhd, confidence: provisional}
  mc_commits:        {scene: s-p3rwe9ka, confidence: confirmed}
  midpoint:          null
beats:                  # v9: tier 2, from the chosen framework; replaced when the framework changes
  theme_stated:      {scene: s-4hq8nzwt, confidence: provisional}
characters: [p-0007, p-0002]   # fog only: who the subplot involves
```

- **Fog subplots are real storyform nodes (v5)** with `status: fog`, holding only a one-line premise and the characters involved. They get stable IDs that scenes, facts, and the weaver can reference before they are mapped. Mapping one flips it to active and fills its threads and decisions.
- **Landmarks live on the storyform (v9),** not on scenes: one entry per landmark makes "one inciting incident per storyform" hold by shape, avoids ambiguity when a scene is a landmark for both the main story and a subplot, lets movement boundaries be derived from one small map, and makes missing landmarks visible as empty entries.
- `dramatic_question` and `thematic_question` (v7; replace `central_question`) feed the argument-coherence check; `premise.history` is the versioned baseline the housekeeper uses for drift.

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

## Checks: who catches what (new in v9; all Decided)

**Script soft checks** (in `complete`, run on every commit; no model, no tokens):

- Missing landmarks (empty entries in a storyform's landmark map)
- Scene-count pacing per movement
- Unearned twist (a reveal with no earlier hint) and dangling setup (a hint never revealed or used)
- Early pivot (the pivot belief ends well before the climax)
- A subplot resolving after the main climax
- Threads out of alignment across movements
- Similar attribute keys (name matching)

**Housekeeper judgment checks** (scoped, rolling, full and session-open passes):

- Premise and idea drift against the versioned baseline
- Motivation: does an action make sense given what the character knows and wants?
- Cause and effect: does one scene's ending actually lead into the next?
- Flat scenes (the turn's start and end differ in words but not in substance)
- A scene drifting from its grid cell's summary
- A thread that seems to carry two separate stories (the promote-to-subplot note)
- Soft clashes between story content and shape decisions
- False beliefs never corrected or paid off, and facts that end with no cause

Script findings use the same plain-language flag flow, so the writer never sees the difference. Keeping countable checks out of the housekeeper makes it cheaper and focuses the cheaper-model evals (item 7) on judgment alone.

**Completeness rubric** (sub-agent; the deep half of `complete`; run at the weave and outline gates and on request):

1. **Cells meet their stage.** A filled cell must do what its stage requires (a Crisis cell where nothing reverses is "filled but not met").
2. **Decisions have support.** Each answered decision is backed by real scenes or facts in its evidence.
3. **The ending answers both questions.** Resolution answers the dramatic question and reflects the thematic question, consistent with decisions 8 and 9 (plot-hole check 4).
4. **The threads interlock.** The climax resolves more than one thread, and every thread bears on the thematic question (adopted from Proposed).

Checks tied to anything opted out with "keep it simple" are skipped. At the gates, any "not met" must be resolved or acknowledged before weaving or outlining continues.

**Flag budget** (applies to housekeeper, rubric and script soft findings):

- Each finding carries a confidence level, the IDs it rests on, and a suggested plain-language question.
- Low-confidence findings are logged, not raised; later passes can see them, so a strengthening pattern can surface later.
- The interviewer raises at most one or two per natural pause, ranked by impact: anything touching a confirmed decision first, then highest fan-out, then the rest.
- The rest wait in a queue of open findings. The orchestrator offers them when momentum slows, and all must be resolved or acknowledged before weaving and outlining (the existing gates).
- Hard script failures are never budgeted: they are raised immediately because the proposal cannot commit until they are fixed.

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

- *Superseded in v7:* plain labels for the four threads are now main plot, inner journey, challenger and relationship (see Story-shape layer).
- *The coverage grid itself is Decided in v9* (see Coverage grid and movements). Still proposed: an interlock check that the climax resolves more than one thread and every thread bears on the thematic question.
- Deterministic scripts for links, status, timelines, and a setup-and-payoff ledger. Rubric-guided model checks reported as "possible issues," not verdicts. A small argument-coherence rule set written in our own vocabulary, which tests the ending against the story-shape decisions.
- Evals using stories with seeded plot holes to test the checker (the skill-creator skill supports evals).
- Avoid reproducing Dramatica's proprietary terminology, dictionary, or matrices. Check the licensing page if borrowing anything specific. This is not legal advice.

## Coverage grid and movements (new in v9; all Decided unless marked)

- **Four fixed stages for every thread**, whatever framework is used. Reasons: completeness means the same thing in every story, so the checker, evals and housekeeper never change with the framework; four stages is one idea a novice can hold; it mirrors Dramatica's four signposts per throughline in our own words.

  | Stage (key) | Plain requirement, any thread |
  |---|---|
  | **Setup** (`setup`) | How things stand, and what disturbs them. |
  | **Complications** (`complications`) | Pressure builds; first attempts fail or cost something. |
  | **Crisis** (`crisis`) | A reversal or discovery changes the direction; ends at the lowest point. |
  | **Resolution** (`resolution`) | The climax and its aftermath; the thread's question gets answered. |

  "Crisis" was chosen over "turning point," which craft writing uses loosely (sometimes for the midpoint).
- **Per-thread readings** of the four stages (stage 4 of each is tested against that thread's shape decisions):

  | Thread | Setup | Complications | Crisis | Resolution |
  |---|---|---|---|---|
  | **Main plot** | The world as it is, the problem arrives, and what is at stake if it isn't solved. | Efforts toward the goal meet growing obstacles; costs pile up. | A major reversal; the goal looks out of reach and the consequence is close. | The final confrontation; the dramatic question is answered (outcome). |
  | **Inner journey** | The MC's belief and its cost. | The belief is strained. | It cracks, or is doubled down on. | The MC changes or holds firm (the pivot belief ends or survives). |
  | **Challenger** | Who they are, and how their view differs from the MC's. | Their pressure on the MC grows, through example, argument or conflict. | Their challenge peaks; the MC can't ignore it anymore. | They change or hold firm, usually the opposite of the MC. |
  | **Relationship** | How the bond stands. | It is tested. | It reaches a breaking point. | It deepens, breaks, or stays unresolved. |
- **Alignment within a storyform:** each thread's stage N falls in roughly the same part of the book, giving four shared **movements**, each advancing every thread. Soft guidance only: a thread running early or late gets one "possible pacing issue," then the writer's choice is respected. Lined-up resolutions support the proposed interlock check.
- **Subplots float independently** of the main storyform (they may span the whole book or only part of it), but their own threads align with each other and they get their own four movements.
- **Subplot minimum:** its own dramatic question, stakes, and at least one thread with all four stages. Other threads accept "keep it simple"; infer-and-confirm suggests which threads the subplot really has. Every thread that exists must have all four stages filled. A subplot can grow into a full storyform by adding threads.
- **Cross-storyform soft checks:** a subplot resolving after the main climax gets a possible pacing issue; every subplot should touch the main storyform's thematic question.
- **Two tiers of beats, all soft (never completeness blockers):**
  - **Tier 1, landmarks (owned by the grid, every story):** Setup: show the normal world, the inciting incident, the MC commits (end of setup). Complications: rising stakes, the midpoint shift (end of complications). Crisis: things fall apart, the low point (end of crisis). Resolution: the climax, the new normal.
  - **Tier 2, framework beats:** supplied only by the chosen framework (for example "theme stated," "refusal of the call") and slotted into the same four stages.
  - **Behavior:** inferred from described scenes and confirmed ("That sounds like the moment that kicks off the story; is that right?"), which tags the scene and teaches the craft term. Missing ones become a suggestion, explained once, then respected. Attached to the main plot by default, except where a landmark plainly belongs to another thread. Subplots get landmarks too, more lightly. Beats tied to theme (such as "theme stated") are suggested only once the thematic question exists.
- **Movement boundaries are derived, never stored:** end of Setup = the scene tagged "MC commits"; end of Complications = the midpoint scene; end of Crisis = the low-point scene; Resolution = everything after. Moving a landmark scene moves the boundary; `sequence.md` stays pure. Before landmarks exist, scenes fall back on their `coverage` stage and the skeleton's provisional placements. A generated index view shows the sequence split into movements. Subplots use the same rule with their own landmarks.
- **Pacing in phase 1 uses scene counts only,** in plain words ("your middle may feel rushed"), as soft notes from the housekeeper or completeness rubric. Relative balance across movements is canon in any edition; word counts are not (see below).
  - **Expected share** per movement comes from the framework's proportions (four-part: about a quarter each; other frameworks supply their own).
  - **A soft note when a movement is under half or over double its expected share** (both thresholds are settings), phrased as a story effect with one sentence of why ("your Crisis has 3 of 40 scenes; the low point may arrive before readers feel the pressure build").
  - **Only once the skeleton exists**; explained once, then respected. Subplots get the same check against their own span, more loosely.

## Weaver, outliner and frameworks

**Decided**

- **Framework choice** (refined in v9): the weaver chooses a framework (four-part, hero's journey, Save the Cat, and so on) with the writer and teaches the tradeoffs.
  - **Inferred any time.** If the writer's scenes clearly follow a framework, Skaldsaga confirms ("this looks like the hero's journey; is that what you're going for?"). A confident yes is recorded as provisional; confusion or "not sure" parks it until weave time.
  - **Recommended late,** at weave time, from taste anchors, genre and the writer's goal.
  - **Four-part is the default** if nothing points elsewhere (it maps one-to-one onto the four stages). **Switchable any time** (only tier-2 beats re-map). **"No framework" is valid** (landmarks only).
  - Stored on the storyform node (`framework`).
- **Layered outline (v9):** a **skeleton pass** across the whole book creates scene slots described by purpose, each linked to a grid cell and any landmarks; then a **deepening pass, one movement at a time, starting with Setup,** adds beats, POV, location, reads and the scene turn. One movement per session fits the clear-and-recap rhythm, keeps sub-agent work bounded, and keeps hand-back questions topical. Later movements may send revisions back to earlier ones; the housekeeper catches the ripple.
- **The weaver produces the skeleton; no separate woven plan (v9).** The weaver chooses the framework (hand-back), resolves fog subplots, then interleaves every thread's cells, landmarks and beats into ordered slot scenes in `sequence.md`, built around pins. The outliner only deepens, one movement per dispatch. Everything weaving decides already lives in slots, their `coverage`, their order, and the storyform's landmark and beat maps, so no intermediate artifact can go stale. Split of judgment: the weaver does whole-book structure, the outliner does scene craft. Full housekeeper passes gate both the weaver and the start of deepening.
- **Layers set the work order, not a depth limit (v9).** Anything the writer raises early (a detailed climax in session two, say) is captured at whatever depth it arrives, provisional like any answer. It becomes a **pin**: the skeleton pass builds around existing detailed scenes rather than over them, and conflicts are flagged as usual. This is the outline-level version of allowing drafts before structure.
- **The skeleton is sized by story need (v9):** at least one slot per grid cell, plus landmarks and framework beats, plus pinned scenes. No word targets.
- **Fog subplots at weave time (v9):** before weaving, the writer chooses for each fog subplot: **map it now** (interview until it meets the subplot minimum, then weave it), **shelve it** (left out of this outline and kept for later), or **fold it in** (it is really part of an existing thread; its facts move there with `move_fact` and the fog node is deleted). The orchestrator raises these as weave time approaches, in plain words. Shelved subplots are not gaps.
- **Promote to subplot (v9, phase 1):** the reverse of fold-in, for any thread content, including content folded in earlier. A new storyform node is created (or a shelved one revived), facts move with `move_fact` (IDs kept, so reads and links survive), and the interviewer asks about each reference as in a split. Emptied cells in the original thread are flagged by the grid; the new subplot must meet the subplot minimum before weaving. Writer-started; the only automatic part in phase 1 is a soft housekeeper note when a thread seems to carry two separate stories. Automatic promotion suggestions are deferred.
- Authors may aim at traditional publishing, self-publishing, or just finishing a story. The skills ask the writer and adapt.
- **Length moves out of phase 1 (v9; supersedes the v1 serial length profile in phase 1).** Word budgets belong to editions (a novel's total, a serial's per-update target), and phase 1 has no prose, so counts would be guesses. A later phase's edition step compares real length with the edition's target and drives revisions that teach why length matters (too fast here, too slow there, over or under budget). Full serial support (a parent map plus arc maps) stays deferred.

**Proposed**

- Name frameworks but write original descriptions, since some beat sheets are trademarked or book-specific.
- For the later length phase: a length data file with ranges by genre and publishing path, with sources and dates, producing a target length, budgets per edition grouping, subplot-fit checks and per-scene estimates. All advice with reasons, never hard rules.

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
- **Web front end (research; retargeted to Rust in v8):** a friendlier home than a terminal or VS Code for non-programmers. The long-term aim is a single Rust binary that embeds the skill and agent prompts, serves a web interface to the agents and the manuscript, and uses Claude through the API. Leads to check:
  - **Agent loop.** The Claude Agent SDK runs Claude Code's agent loop from TypeScript (and Python), not Rust (verify). Options: write our own loop over the Claude API (tool use, prompt caching), drive Claude Code as a subprocess in headless mode, or a community Rust client.
  - **Authentication and billing.** Calling the API means an API key and per-token billing, which a novice may not expect; whether a writer's Claude subscription can be used instead must be checked against Anthropic's current terms.
  - **Licensing and usage terms** for distributing such an app.
  - The binary delivery choice (open item 13) should leave room for this app, so the phase 1 binary and the later app share one core library.

**Proposed**

- An agent-drafted scene never becomes a voice sample unless the writer approves it, so the style does not drift toward the machine's habits.
- A writer sub-agent can be held to its scene packet: limit its tools, block reads outside the packet folder with a hook, and omit project instruction files (`omitClaudeMd`).
- Run parallel scene tasks in separate workspaces (sub-agents can run in a temporary git worktree). Scenes that write the same node are serialized. A scene that follows another in reading order waits for its outcome.

## Research notes (verify before relying)

- **Dramatica.** A storyform spans four throughlines (main character, influence character, relationship, objective story). The full theory has about seventy-five objective appreciations. The 12 essential questions are four about character, four about plot, and four about theme. Eight are binary and four ask you to pick from "quads" (the thematic ones), which is roughly 65,536 combinations before the questions constrain each other. Confirmed from sources: Resolve (change or steadfast), Growth (start or stop), Approach (do-er or be-er), Problem-Solving Style (linear or holistic), Driver (action or decision), Limit (timelock or optionlock). Outcome and Judgment (success or failure, good or bad) are the other two binary ones and together give four endings. *The exact four theme questions were not listed on the pages retrieved.* The Overall Story Concern is one of them, and the Issue layer is the basis of the central theme. The community uses "story weaving" for combining storyforms, and forum posts suggest no formal weaving theory exists.
- **Dramatica's four theme questions and full story-point list (researched in v7, October 18, 2026).**
  - **The four thematic essential questions are a narrowing chain**, each picked from four options the previous answer allows: Domain (Universe, Physics/Activity, Mind, Psychology), then Concern, then Issue (also called Range), then Problem (storymind.com, "12 essential questions"). The choices come from Dramatica's proprietary table of story elements. Skaldsaga keeps its own-words approach instead; our thematic question corresponds roughly to the Issue and its Counterpoint.
  - **The full storyform** is usually described as about 75 story points; a community tally of the Dramatica software's report counts 92, because counts differ in what they include (discuss.dramatica.com, "Table of Scenes & every possible story point"). Groups: overview (backstories for MC, impact character and relationship, plus synopses); audience appreciations (nature, essence, tendency, reach); per throughline (throughline, concern, issue, counterpoint, thematic conflict, problem, solution, symptom, response, benchmark, four signposts, plus unique ability and critical flaw for the two characters, catalyst and inhibitor for the other two); plot (the eight dynamics plus impact character resolve, and goal, consequence, cost, dividend, requirements, prerequisites, preconditions, forewarnings).
  - **Plot points in plain terms** (dramatica.com theory book, chapter 17): goal (what everyone pursues), requirements (what must be done to reach it), consequence (what happens if it isn't reached), forewarnings (signs the consequence is getting close), dividends (small rewards along the way), costs (setbacks paid along the way), prerequisites (resources needed first), preconditions (strings attached by whoever controls them).
  - **Candidate additions for Skaldsaga** are under open item 17.
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
- **Rust delivery (general knowledge, v8; verify before relying).** cargo-dist can automate cross-platform builds, GitHub Releases and installers. macOS quarantines downloaded unsigned binaries (Gatekeeper); signing and notarization need an Apple developer account. Windows SmartScreen warns on unsigned executables. Downloading a binary on first run needs a download tool: curl ships with macOS and recent Windows, and PowerShell can download too. *Unverified:* whether a plugin installed through Claude Code keeps the executable permission on bundled files, and whether macOS quarantines files a plugin install writes.
- **Anthropic harness guidance** ("Effective harnesses for long-running agents"): agents work in discrete sessions with no memory between them, so use a progress file plus git history to orient each fresh session, track features in JSON with a pass/fail flag, work one feature per session, and use separate checkers because agents judge their own work poorly.

## Open items

1. *Closed:* `ARCHITECTURE.md` reviewed this session (see What changed since v5).
2. *Closed:* sub-agents that need the writer use the hand-back pattern (see Architecture).
3. *Closed in v9:* the 12 shape decisions are final with plain-language wording (see Story-shape layer); the theme layer collapsed to the thematic question; node fields for the adopted story points are settled (see item 17).
4. *Closed in v9:* four fixed stages (Setup, Complications, Crisis, Resolution), aligned within a storyform, with landmarks and framework beats on top (see Coverage grid and movements).
5. *Closed in v9* (see Weaver, outliner and frameworks, and Coverage grid and movements): layered outline, weaver produces the skeleton, outliner deepens, pins, framework timing, fog subplot outcomes, promote to subplot, stage readings for all threads, storyform-level landmarks, the scene turn, and the scene-count pacing heuristic.
6. *Closed at design level in v9* (see Checks: who catches what): script versus housekeeper split, the four completeness-rubric judgments, and the flag budget. Exact prompt text, confidence scales and message wording are build work.
7. Plan the evals with seeded plot holes, including a comparison of housekeeper models on scoped passes (to decide whether a cheaper default is safe). Also plan tests for the gating lanes, quarantine, version-checked commit, held-proposal flow, rolling passes, uniqueness rule (all three cardinalities), delete blocking, and merge.
8. *Deferred in v9 (length left phase 1):* research traditional and self-publishing length norms for the length data file, when the edition phase begins.
9. Read the wayfinder SKILL.md for patterns worth reusing.
10. *Partly closed:* plugins cannot ship the main status line, so first-run setup writes it into the project's `.claude/settings.json` (see Research notes). Still unverified: hook access to context usage (no longer needed), and the SessionStart `initialUserMessage` field. Also: what happens when the writer already has a personal status line, since the project setting would replace it. Choose starting values for all settings: meter thresholds, rolling-pass interval (every N answers), and the reader cap that escalates to a full pass.
11. Check the Dramatica licensing page before borrowing any specific term.
12. Starter relation vocabulary for the attribute registry (family, place, bond, related; plus the `strength` and `flaw` attributes added in v9) with each relation's symmetry and cardinality. Confirm the `add_item`/`remove_item` ops.
13. *Reopened in v8 (scripts are now Rust):* how the compiled binaries reach the writer. Options: bundle every platform's binary inside the plugin (simple, but large and puts binaries in git); have CI publish a built distribution branch or separate marketplace repo that the plugin installs from (keeps main clean); or have first-run setup download the right binary from GitHub Releases and verify its checksum (smallest plugin, but needs a network and a download tool). Also: where the binaries live (not a top-level `bin/`), and what happens if a binary is missing or blocked by the operating system. Leave room for the later standalone app (see Future phases).
14. *Revised in v9:* phase 1 no longer proposes a first edition. The minimal edition file format and edition rules stay decided for the later phase that builds editions, which also extends the format with cuts, alternative scenes, per-grouping release status and pins, and review snapshots.
15. How reviewer and reader feedback is stored in `reviews/` and turned into questions about the current text (mostly later phases; the snapshot link is decided).
16. How forking a project for a retelling works in practice (later phase; the rule is decided).
17. *Closed in v7:* extra story points adopted, in our own words, inferred and confirmed like everything else. **Stakes are required** (part of the required core); the rest are optional depth that accepts "keep it simple." Node fields settled in v9 (no new node types; see the notes after this list):
    - **Stakes (required):** the goal plus what happens if it fails (goal, consequence).
    - **Pressure and payoff along the way:** costs, small wins, warning signs that the consequence is near, and the steps required to reach the goal (costs, dividends, forewarnings, requirements). Useful to the outliner for pacing.
    - **Strength and flaw** for the MC and challenger (unique ability, critical flaw); "fatal flaw" is a familiar craft term.
    - **What's really wrong versus what the MC thinks is wrong**, and what they try versus what would fix it (problem, symptom, response, solution). Ties to the pivot belief and to common craft ideas such as "want versus need."
    - **Backstory** for the MC, the challenger and their relationship, feeding facts.
    - Not adopted (lower value or too complex for novices): the Domain/Concern/Issue chain, thematic conflict, catalyst and inhibitor, benchmark, prerequisites and preconditions, audience appreciations, and anything tied to the table of story elements.
    - **Storage (v9):** stakes sit with the dramatic question on the storyform (decision 5); strength and flaw are person facts with registered attributes `strength` and `flaw` (single cardinality, so they can change over time with `ends`); "what's really wrong versus what the MC thinks" is a belief fact (the mistaken idea, often the pivot belief) contradicting a truth fact (the real problem), building on decision 2; pressure along the way is facts with a `role` tag (`cost`, `small_win`, `warning`, `required_step`) linked to the main plot thread, which the outliner spreads across movements; backstory is ordinary facts whose `true_from` is before the first scene. All of them get time tracking, knowledge tracking and plot-hole checks for free.
18. *New in v8:* the Rust build and release pipeline. Which targets to support at launch (macOS Apple Silicon and Intel, Linux x86 and ARM, Windows), code signing and notarization (cost and who holds the keys), CI tooling (for example cargo-dist), versioning so the plugin and its binaries never mismatch, and the crate layout (core library plus command-line front end). Confirm the Proposed items from "What changed since v7": one binary with subcommands, binaries out of the main history, and the session meter as the pilot.

## Suggested next steps

Next candidates: the evals plan (item 7), starting values for all settings (item 10), and the starter relation vocabulary (item 12). With items 3 to 6 closed, the phase 1 design is close to buildable.

The Rust move (items 13 and 18) can run as a separate track: confirm the Proposed items, then build the session meter as the pilot once building starts.
