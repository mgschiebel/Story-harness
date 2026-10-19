# Skaldsaga: notes for build sessions

Read `HANDOFF.md` and `ARCHITECTURE.md` for the design. Don't load `docs/background.md` (it keeps outside story-theory terms out of prompts and code).

## First step in a cloud session: let cargo reach crates.io

Cloud sessions list crates.io as "no proxy", but it can only be reached through the session proxy, so plain `cargo` fails with "Could not resolve host: index.crates.io". Before the first cargo command, install this wrapper (`~/.local/bin` comes before `~/.cargo/bin` on the PATH):

```sh
mkdir -p ~/.local/bin && cat > ~/.local/bin/cargo <<'EOF'
#!/bin/sh
# Clear the proxy bypass list for cargo only; the session proxy reaches crates.io.
NO_PROXY= no_proxy= exec "$HOME/.cargo/bin/cargo" "$@"
EOF
chmod +x ~/.local/bin/cargo && which cargo
```

`which cargo` should print `~/.local/bin/cargo`. Editing `~/.bashrc` does not work here: the shell loads a snapshot taken when the session started. If crates.io still fails, the environment's network allowlist needs `index.crates.io`, `static.crates.io` and `crates.io`.

## Pipeline (phase 1)

- One branch per task (for example `build/meter-pilot`); never push to `main`.
- Before every push: `cargo fmt --all`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo test --locked`.
- Open pull requests with the REST API (`gh api repos/mgschiebel/skaldsaga/pulls --method POST ...`); GraphQL, and so `gh pr create`, is blocked from cloud sessions. The PR summary is in plain language; the author reviews on a phone.
- After pushing, check CI with `gh api repos/mgschiebel/skaldsaga/commits/<sha>/check-runs` and fix failures in the session. Job logs can't be downloaded from here; read check-run annotations instead.
- Phase 1 builds for Linux x64 only.
- **Release for phone tests (Decided October 18, 2026):** when a PR changes something the author can try from the phone (a new command, a skill change, a hook change), bump the version in that PR (both files below) and, once the author merges it, start the Release workflow. Internal-only PRs (no visible change) skip the release. The testing Project doesn't pick up a release on its own right away: after the release finishes, remind the author to sync the marketplace manually (Project plugin settings > Manage Marketplaces > Sync), then start a fresh session.
- **Releasing:** cloud sessions can't push tags (the git proxy returns 403). Instead, with the author's go-ahead, bump the version in both `Cargo.toml` and `plugin/.claude-plugin/plugin.json` (merged to main), then start the Release workflow: `gh api repos/mgschiebel/skaldsaga/actions/workflows/release.yml/dispatches --method POST -f ref=main`. It creates tag `v<version>`, the GitHub Release and the `dist` branch, and refuses if that tag already exists.

## Code

- `crates/skaldsaga-core`: all logic, kept portable (file access goes through `fsio`). `crates/skaldsaga`: the thin command-line front end.
- `plugin/`: the Claude Code plugin (manifest, marketplace file, skills, hooks, launcher). `ci/assemble-dist.sh` adds the binary to make the installable folder; CI validates and smoke-tests it on every push, and the Release workflow publishes it to the `dist` branch. Keep `plugin.json`'s version equal to the crate version.
- `json` wraps `serde_json` with `preserve_order`, so settings files we edit for the writer keep their key order.
- Anything that runs automatically (status line, hooks) does nothing unless `.harness/` exists, and never shows the writer an error.
