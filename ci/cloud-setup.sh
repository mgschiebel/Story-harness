#!/bin/sh
# Environment setup script for the author's Claude Code Project (phone testing).
# Paste into Project settings > Environment > Setup script. It runs before each
# cloud session starts.
#
# It no longer makes a Skaldsaga project. Earlier versions created
# /home/claude/.harness so the session-start hook had something to find, but
# that left a stray, empty project behind: claude.ai/code on the web starts the
# session inside the attached repository (for example /home/user/<repo>), and
# the mobile app starts it in /home/claude with the repository one level down.
# The hook handles both layouts on its own, and `/skaldsaga` sets up and wires
# the meter in the repository itself (0.3.2 re-test notes, docs/tests/).
set -eu

# Let cargo reach crates.io (see CLAUDE.md), so build sessions skip that step.
mkdir -p /root/.local/bin
cat > /root/.local/bin/cargo <<'EOF'
#!/bin/sh
NO_PROXY= no_proxy= exec "$HOME/.cargo/bin/cargo" "$@"
EOF
chmod +x /root/.local/bin/cargo
