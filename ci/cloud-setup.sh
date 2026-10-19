#!/bin/sh
# Environment setup script for the author's Claude Code Project (phone testing).
# Paste into Project settings > Environment > Setup script. It runs before each
# cloud session starts, so the SessionStart hook finds a Skaldsaga project.
#
# Each cloud session starts on a fresh machine: nothing from an earlier session
# survives, so this has to recreate the test project every time.
set -eu

PROJECT=/home/claude

# 1. Make the folder a Skaldsaga test project (what the hook looks for).
mkdir -p "$PROJECT/.harness"

# 2. Wire the session meter into the project's personal status line
#    (.claude/settings.local.json, which stays out of git).
#    The plugin may not be synced yet when this runs, so the command finds the
#    launcher at run time. The session-start hook then rewrites it to the exact
#    plugin path (it updates any status line containing "meter statusline").
mkdir -p "$PROJECT/.claude"
SETTINGS="$PROJECT/.claude/settings.local.json"
if [ ! -s "$SETTINGS" ]; then
  cat > "$SETTINGS" <<'EOF'
{
  "statusLine": {
    "type": "command",
    "command": "for p in /root/.claude/plugins/*/*/skaldsaga/scripts/skaldsaga /root/.claude/plugins/cache/*/skaldsaga/*/scripts/skaldsaga; do [ -f \"$p\" ] && exec sh \"$p\" meter statusline; done; echo"
  }
}
EOF
fi

# 3. Let cargo reach crates.io (see CLAUDE.md), so build sessions skip that step.
mkdir -p /root/.local/bin
cat > /root/.local/bin/cargo <<'EOF'
#!/bin/sh
NO_PROXY= no_proxy= exec "$HOME/.cargo/bin/cargo" "$@"
EOF
chmod +x /root/.local/bin/cargo
