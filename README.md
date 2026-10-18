# Skaldsaga

**A story mentor that helps first-time writers turn a rough idea into a complete, plot-hole-free outline.**

A *skald* was a Norse poet-storyteller, and *saga* means "story" (in modern Icelandic, *skáldsaga* is the everyday word for a novel). Skaldsaga is an open-source set of skills for [Claude Code](https://code.claude.com) that acts like an experienced writing mentor. It asks you questions about your story, quietly keeps track of everything you decide, catches contradictions and loose ends, and teaches craft as you go.

> **Status: in design.** Nothing is built or installable yet. This README describes how Skaldsaga is meant to work. The design lives in [`HANDOFF.md`](HANDOFF.md) and [`ARCHITECTURE.md`](ARCHITECTURE.md).

## Who it's for

People who have a story in their head but have never written one. You don't need to know story theory, writing jargon, or anything about programming. Skaldsaga explains every question in plain English and tells you why it matters to your story.

Experienced writers are welcome too. You can skip what you already know.

## What you get

At the end of the first phase, you have a **complete, detailed outline**:

- the people, places and facts of your story world, all consistent with each other
- the shape of your story: who changes, what's at stake, how it ends and why
- every scene in reading order, each with its purpose and beats

"Complete" means more than "finished." It means the big story questions are answered, every storyline has something happening at every stage, and the plot-hole checks run clean:

1. **Continuity and timeline:** nothing happens before its cause, and nothing contradicts what's already true.
2. **Knowledge:** no character acts on something they haven't learned yet, and no twist arrives before it's earned.
3. **Setups and payoffs:** every hint pays off, and every payoff was set up.
4. **The ending answers the story's question.**
5. **Cause and effect:** each scene follows from what came before.

## How it works

1. **Install** Skaldsaga as a Claude Code plugin. Install steps will appear here once it's built.
2. **Open a folder** for your story in Claude Code and type `/skaldsaga`. That's the only command you need to remember.
3. **Talk about your story.** Skaldsaga asks about concrete things, such as a scene you can picture or a character you know, and works out the bigger structure from your answers. It always checks with you ("It sounds like your hero ends up changing. Is that right?").
4. **Take breaks whenever you like.** Everything is saved after each answer you confirm. When it's time for a fresh start, Skaldsaga tells you. Next time, open the folder and it greets you with a recap of where you left off.

### Things it handles for you

- **You don't have to know the answers yet.** Unsure answers are kept as "to be decided" and settled later, when the story makes them clearer. If you're stuck, Skaldsaga offers "what if" options built from your own ideas.
- **You can change your mind.** If a new answer conflicts with an earlier one, Skaldsaga points it out while the topic is still fresh and lets you choose what to keep.
- **You can bring writing you already have.** Point it at an existing folder of drafts and notes. It looks before touching anything, recommends keeping your originals safe in an archive, and pulls the story facts out of your drafts for you to approve. It never changes your writing behind your back.
- **It isn't tied to novels.** Your story is kept as one sequence of scenes. Chapters, a play's acts, or a web serial's weekly updates are just different ways to group those scenes later.

## What it doesn't do (yet)

The first phase stops at the outline. Later phases are planned to help with drafting scenes in your own voice, editing, and preparing editions for publishing: an e-book, a print book, or a web serial. You stay the author throughout.

Skaldsaga is inspired by story-structure ideas, including Dramatica, but it is not affiliated with Dramatica and doesn't require you to learn it. The goal is a good story, not a theory-correct one.

## For contributors

The project is in its planning phase. Start with:

- [`HANDOFF.md`](HANDOFF.md): every decision so far, with its reasoning and open questions
- [`ARCHITECTURE.md`](ARCHITECTURE.md): a one-page sketch of the components and how they fit together

## License

[GNU General Public License v2](LICENSE).
