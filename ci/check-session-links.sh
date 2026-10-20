#!/bin/sh
# Fails when new commits or a pull request description contain a Claude
# session link (security requirement SR-03). Session links point at someone's
# private claude.ai session; once in public history they stay findable, and a
# session later switched to Public could expose code or credentials.
#
# Usage: sh ci/check-session-links.sh BASE HEAD
#   BASE, HEAD  commits bounding the new work (BASE excluded).
# The pull request description, if any, is read from $PR_BODY. It arrives
# through the environment, never pasted into a command, so its text can't run.
set -eu

base=$1
head=$2
pattern='claude\.ai/code/session_|Claude-Session:'
found=0

if git log --format='%H%n%B' "$base..$head" | grep -Eq "$pattern"; then
  for c in $(git rev-list "$base..$head"); do
    if git log -1 --format=%B "$c" | grep -Eq "$pattern"; then
      echo "::error::Commit $(git log -1 --format='%h %s' "$c") contains a Claude session link. Remove the Claude-Session line from its message (security requirement SR-03)."
    fi
  done
  found=1
fi

if [ -n "${PR_BODY:-}" ] && printf '%s\n' "$PR_BODY" | grep -Eq "$pattern"; then
  echo "::error::The pull request description contains a Claude session link. Edit the description to remove it (security requirement SR-03)."
  found=1
fi

if [ "$found" = 0 ]; then
  echo "No Claude session links in new commits or the description."
fi
exit "$found"
