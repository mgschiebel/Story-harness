# Skaldsaga: Background and Attribution

*Not loaded by build sessions.* `HANDOFF.md` and `ARCHITECTURE.md` use only Skaldsaga's own vocabulary, so that outside terminology never leaks into skill prompts, schemas, code or anything a writer sees. This file records where the ideas came from, the research behind them, and what was deliberately left out.

## Where the ideas came from

Skaldsaga's early design was inspired in part by **Dramatica**, the theory of story created by Melanie Anne Phillips and Chris Huntley, and particularly by its idea that a complete story examines its central problem from four perspectives. Skaldsaga is **not affiliated with or endorsed by The Dramatica Co.** It uses none of Dramatica's licensed semantic model, its table of story elements, or its valid-combination logic, and its checker, terminology and wording are its own. Dramatica® is a registered trademark of its owner.

Other influences include common craft advice on story structure (four-part structure, the hero's journey, beat sheets), Matt Pocock's wayfinder skill for multi-session planning, and Anthropic's guidance on harnesses for long-running agents.

**Practices that follow from this** (see the licensing notes below; not legal advice):

1. Never describe Skaldsaga as Dramatica-compatible, certified or endorsed, and don't use the name in the product name or as a selling point. A plain acknowledgment like the paragraph above is ordinary factual use; confirm the wording before public release.
2. Never copy definitions or passages from Dramatica texts.
3. Keep Dramatica terms out of the writer-facing product, the build documents, public code and schemas.

## How our terms map to the original inspiration

For maintainers only; never surfaced to writers or used in code.

| Skaldsaga | Inspiration (Dramatica term) |
|---|---|
| Arc (internal node type; one complete argument with its threads) | Storyform (Skaldsaga's internal name until v9) |
| Thread: main plot, inner journey, challenger, relationship | Throughline: objective story, main character, influence (impact) character, relationship |
| Challenger | Influence character / impact character |
| Stages: Setup, Complications, Crisis, Resolution | Loosely, the four signposts per throughline (our stages and requirements are our own) |
| Decision 1, changes or holds firm | Main character resolve (change or steadfast) |
| Decision 2, quit something or start something | Growth (stop or start) |
| Decision 3, fix the situation or fix herself | Approach (do-er or be-er) |
| Decision 4, step by step or gut | Problem-solving style (linear or holistic) |
| Decision 6, events force choices or choices make events | Story driver (action or decision) |
| Decision 7, time runs out or nowhere left to turn | Story limit (timelock or optionlock) |
| Decisions 8 and 9, gets what she was after; happier or better off | Outcome (success or failure); judgment (good or bad) |
| Decision 10, the thematic question | Roughly the issue and its counterpoint |
| Decisions 11 and 12 (challenger and relationship) | No direct equivalent; our own |
| Stakes | Goal and consequence |
| Pressure along the way: costs, small wins, warnings, required steps | Costs, dividends, forewarnings, requirements |
| Strength and flaw | Unique ability and critical flaw |
| What's really wrong versus what the MC thinks | Problem, symptom, response, solution |
| Weaving | The community's informal "story weaving" |

## Research notes (from v1 to v9 sessions; verify before relying)

- **Dramatica.** A storyform spans four throughlines (main character, influence character, relationship, objective story). The full theory has about seventy-five objective appreciations. The 12 essential questions are four about character, four about plot, and four about theme. Eight are binary and four ask you to pick from "quads" (the thematic ones), which is roughly 65,536 combinations before the questions constrain each other. Confirmed from sources: Resolve (change or steadfast), Growth (start or stop), Approach (do-er or be-er), Problem-Solving Style (linear or holistic), Driver (action or decision), Limit (timelock or optionlock). Outcome and Judgment (success or failure, good or bad) are the other two binary ones and together give four endings. *The exact four theme questions were not listed on the pages retrieved.* The Overall Story Concern is one of them, and the Issue layer is the basis of the central theme. The community uses "story weaving" for combining storyforms, and forum posts suggest no formal weaving theory exists.
- **Dramatica's four theme questions and full story-point list (researched in v7, October 18, 2026).**
  - **The four thematic essential questions are a narrowing chain**, each picked from four options the previous answer allows: Domain (Universe, Physics/Activity, Mind, Psychology), then Concern, then Issue (also called Range), then Problem (storymind.com, "12 essential questions"). The choices come from Dramatica's proprietary table of story elements. Skaldsaga keeps its own-words approach instead; our thematic question corresponds roughly to the Issue and its Counterpoint.
  - **The full storyform** is usually described as about 75 story points; a community tally of the Dramatica software's report counts 92, because counts differ in what they include (discuss.dramatica.com, "Table of Scenes & every possible story point"). Groups: overview (backstories for MC, impact character and relationship, plus synopses); audience appreciations (nature, essence, tendency, reach); per throughline (throughline, concern, issue, counterpoint, thematic conflict, problem, solution, symptom, response, benchmark, four signposts, plus unique ability and critical flaw for the two characters, catalyst and inhibitor for the other two); plot (the eight dynamics plus impact character resolve, and goal, consequence, cost, dividend, requirements, prerequisites, preconditions, forewarnings).
  - **Plot points in plain terms** (dramatica.com theory book, chapter 17): goal (what everyone pursues), requirements (what must be done to reach it), consequence (what happens if it isn't reached), forewarnings (signs the consequence is getting close), dividends (small rewards along the way), costs (setbacks paid along the way), prerequisites (resources needed first), preconditions (strings attached by whoever controls them).
  - **Candidate additions for Skaldsaga** are under open item 17.
- **Dramatica licensing (checked v9, October 18, 2026; not legal advice).**
  - **Dramatica® is a registered trademark**, and the site footer cites U.S. Patents 5,734,916 and 6,105,046. Both are 1990s patents and are very likely expired (US utility patents last 20 years from filing), *verify*. Patents would not cover our own independently built checker anyway unless it practiced the claimed method.
  - **The Narrative Context Protocol is MIT licensed** (github.com/narrative-first/narrative-context-protocol, `governance/licensing.md`). The MIT grant covers the schema and reference code only. It explicitly excludes the **Dramatica Semantic Model** (valid combinations, private matrices, algorithms, completion, diagnosis, semantic validation, certification) and grants **no rights to Dramatica trademarks, compatibility marks or certification marks**. Third parties may publish mappings in their own namespace.
  - **The theory book, dictionary and website text are copyrighted** like any publication, even where free to read.
  - **What this means for Skaldsaga:** our design already sits on the safe side. We use none of the semantic model (no table of story elements, no valid-combination logic, no Domain/Concern/Issue chain), our checker is our own, and our names and wording are our own. Remaining practices: (1) never call Skaldsaga Dramatica-compatible, certified or endorsed, and don't use the name in the product name or as a selling point; a plain, factual acknowledgment ("inspired in part by ideas from Dramatica theory, by Melanie Anne Phillips and Chris Huntley; not affiliated") is ordinary nominative use, but confirm wording before release; (2) never copy definitions or passages from Dramatica texts; (3) keep Dramatica terms out of the writer-facing product and, ideally, out of public code and schemas too (see the storyform naming question in Open items).
- **Narrative Context Protocol (NCP).** An open interchange schema on GitHub (3.0.0-rc.1 at last check). It validates structure only and does not include valid-combination logic. Dramatica's engine, semantic validation, and certification are licensed separately. Since the project is moving to its own checker, NCP is optional.

## Story points considered and not adopted

Lower value or too complex for novices: the Domain, Concern and Issue chain; thematic conflict; catalyst and inhibitor; benchmark; prerequisites and preconditions; audience appreciations; anything tied to the table of story elements. The four Dramatica theme questions were collapsed into Skaldsaga's single own-words thematic question in v9.
