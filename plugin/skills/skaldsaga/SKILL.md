---
name: skaldsaga
description: Skaldsaga story mentor (test build). Use when the person types /skaldsaga or asks to start or test Skaldsaga.
---

# Skaldsaga (test build)

This is an early test build. The story conversation isn't built yet; this skill sets up a story folder and checks that the installed pieces work. Keep every reply short and in plain language, with no file paths or command names unless the person asks.

Below, `skaldsaga` means `sh "${CLAUDE_PLUGIN_ROOT}/scripts/skaldsaga"`, and `METER` means the text `sh "${CLAUDE_PLUGIN_ROOT}/scripts/skaldsaga" meter statusline` (passed as one quoted argument).

## 1. Check the program

Run `skaldsaga --version`.

- If it prints a version, note it for the summary.
- If it fails, quote the message it printed and stop.

## 2. Set up this folder

Look for a `.harness` folder at the top of the current project.

- **It doesn't exist:** ask one question: "Want me to set this folder up for your story? I'll add four folders (your story notes, your writing, feedback, and style notes) and a small session meter that tells us when it's time for a break." Do nothing else until they answer. On a no, stop.
- **It exists**, or they said yes: run setup. It only adds what's missing and never replaces a file, so it's safe in a folder that's already set up.

  ```sh
  skaldsaga init --meter-command "METER"
  ```

Read the JSON it prints:

- `existing_files`: the folder already holds the person's own files (`existing` lists some of them). Say that bringing in writing they already have isn't built yet, and suggest opening an empty folder for now. Nothing was changed. Stop here.
- `already_set_up`: everything was in place. Skip to step 4.
- `created`: setup added the folders. Continue with step 3.

The `meter` part says what happened to the session meter:

- `wired`: the meter is on. If `wrapped` isn't null, mention that their own status line still shows.
- `already_wired` or `updated`: the meter was already on.
- `overridden_by_local`: their personal status line setting would hide the meter; nothing was changed. Ask before touching that file.

## 3. Save a first snapshot with git

Skaldsaga keeps its history in git, so the person can always go back.

1. If the folder has no `.git` folder, run `git init -q`.
2. Add only the paths listed in `commit_paths`, never anything else in the folder:

   ```sh
   git add -- <each path in commit_paths>
   ```

3. If anything is staged (`git diff --cached --quiet` exits non-zero), commit it:

   ```sh
   git commit -q -m "Skaldsaga: set up the story folders"
   ```

If git is missing or the commit fails (for example, git doesn't know the person's name yet), explain the problem in one plain sentence and say the folders are set up anyway; the snapshot can be saved later. Don't change git settings without asking.

## 4. Meter check

Run `skaldsaga meter check`.

Report the `state` in plain words: `ok` (plenty of room), `suggest_wrap_up` (time to think about a break), `wrap_up` (time to stop and start fresh), or `unknown` (no reading yet; normal at the start of a session, or where the status line doesn't run). Include `note` if there is one.

## 5. Summary

In three or four short lines, say:

- the program version;
- what setup did (folders added, or already in place) and whether the snapshot was saved;
- the meter state;
- whether the session-start hook ran: it adds a line to your context starting "This folder is a Skaldsaga project". Say yes or no; don't quote it. (It only runs when a session starts, so right after first-time setup it's normal for it to be missing; it will run next session.)
