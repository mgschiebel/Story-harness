---
name: skaldsaga
description: Skaldsaga story mentor (test build). Use when the person types /skaldsaga or asks to start or test Skaldsaga.
---

# Skaldsaga (test build)

This is an early test build. The story conversation isn't built yet; this skill only checks that the installed pieces work. Keep every reply short and in plain language, with no file paths unless the person asks.

## 1. Check the program

Run:

```sh
sh "${CLAUDE_PLUGIN_ROOT}/scripts/skaldsaga" --version
```

- If it prints a version, say Skaldsaga's program is working and give the version.
- If it fails, quote the message it printed and stop.

## 2. Check this folder

Look for a `.harness` folder at the top of the current project.

- **It exists:** this is a Skaldsaga test project. Run the meter check below and report.
- **It doesn't exist:** ask one question: "Want me to turn this folder into a Skaldsaga test project? I'll add a hidden `.harness` folder and a small session meter to the status line." Do nothing else until they answer. Only on a yes, run:

  ```sh
  mkdir -p .harness
  sh "${CLAUDE_PLUGIN_ROOT}/scripts/skaldsaga" meter wire --command "sh \"${CLAUDE_PLUGIN_ROOT}/scripts/skaldsaga\" meter statusline"
  ```

  Then explain the result in one sentence:
  - `wired`: the meter is on. If `wrapped` isn't null, add that their own status line still shows.
  - `already_wired` or `updated`: the meter was already on.
  - `overridden_by_local`: their personal status line setting would hide the meter; nothing was changed. Ask before touching that file.

Full first-run setup (story folders and the rest) isn't built yet, so don't create anything else.

## 3. Meter check

Run:

```sh
sh "${CLAUDE_PLUGIN_ROOT}/scripts/skaldsaga" meter check
```

Report the `state` in plain words: `ok` (plenty of room), `suggest_wrap_up` (time to think about a break), `wrap_up` (time to stop and start fresh), or `unknown` (no reading yet; normal at the start of a session, or where the status line doesn't run). Include `note` if there is one.

Finish by saying whether the session-start hook ran: it adds a line to your context starting "This folder is a Skaldsaga project". Say yes or no; don't quote it.
