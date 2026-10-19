# Session meter

The meter tells the interviewer when the conversation's context window is getting full, so a session can end at a natural pause before the model loses sharpness. It is the first Rust subcommand and the pilot for the whole delivery chain.

## How it fits together

1. **First-run setup** wires the status line once:
   `skaldsaga meter wire --command "sh <plugin>/scripts/skaldsaga meter statusline"`
   This writes `statusLine` into the project's `.claude/settings.json` (a plugin can't register it). If the writer already has a status line, in the project file or in `~/.claude/settings.json`, it is recorded in `.harness/settings.json` under `meter.wrapped_statusline` and keeps running: the writer still sees their own line.
2. **Claude Code** runs the status line after each turn. `meter statusline` records `.harness/meter.json` and prints a line.
3. **The interviewer** runs `skaldsaga meter check` after each saved answer and acts on `state`.
4. **The SessionStart hook** (`skaldsaga hook session-start`) clears the last reading when a fresh session starts, so a full reading from the last session can't end a new one early. It also re-points the status line at the current plugin folder after a plugin update.

Everything is guarded by `.harness/`: outside a Skaldsaga project, `statusline` prints an empty line and `reset` does nothing.

## Commands

| Command | Output | Notes |
|---|---|---|
| `meter statusline` | One status line | Reads Claude Code's JSON on stdin. Never fails visibly: always exits 0. |
| `meter check [--session ID] [--project DIR]` | JSON (below) | `--session` ignores a reading left by another session. |
| `meter reset [--project DIR]` | Nothing | Deletes `.harness/meter.json`. |
| `meter wire --command CMD [--project DIR] [--user-settings FILE]` | JSON `outcome` | `wired`, `updated`, `already_wired`, or `overridden_by_local`. |

`DIR` defaults to `$CLAUDE_PROJECT_DIR`, then the current folder. Exit codes: 0 success, 1 failure, 64 usage mistake, 69 subcommand not built yet.

### `meter check` output

```json
{
  "state": "suggest_wrap_up",
  "used_percentage": 55.2,
  "suggest_wrap_up_percent": 50,
  "wrap_up_percent": 70,
  "session_id": "…",
  "note": null
}
```

| `state` | Interviewer does |
|---|---|
| `ok` | Carries on. |
| `suggest_wrap_up` | Recommends wrapping up at the next natural pause; the writer may decline once. |
| `wrap_up` | Finishes the current question, saves, writes the recap, and tells the writer to start fresh. |
| `unknown` | Carries on; `note` says why (no reading yet, usage not reported yet, or a different session). The manual session boundary still works. |

### `overridden_by_local`

A status line in `.claude/settings.local.json` overrides the project file, so wiring changes nothing and reports the existing line. Setup should explain this in one plain sentence and ask the writer before touching their personal file.

## Settings (`.harness/settings.json`)

```json
{
  "meter": {
    "suggest_wrap_up_percent": 50,
    "wrap_up_percent": 70,
    "wrapped_statusline": { "type": "command", "command": "~/.claude/statusline.sh" }
  }
}
```

Values outside 1 to 100 fall back to the defaults, and a suggestion threshold above the hard one is lowered to match, so a hand-edited typo never stops the meter. Other keys in the file are preserved.

## The launcher

`plugin/scripts/skaldsaga` (installed as `scripts/skaldsaga` in the plugin) picks `scripts/bin/<platform>/skaldsaga`, restores a missing executable bit, and (except for the status line, which must stay fast) checks the binary's version against `scripts/VERSION`. Always invoke it through `sh`, so it works even if a plugin install drops its executable bit. `SKALDSAGA_BIN` points it at another binary during development.
