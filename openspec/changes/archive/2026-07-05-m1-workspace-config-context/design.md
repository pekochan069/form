## Context

`form-cli` is still a minimal help/version binary. M1 needs internal runtime foundation modules before provider, session, and read/search tools can safely use the workspace.

## Goals / Non-Goals

**Goals:**
- Detect the canonical workspace root as the git root when available, otherwise the current directory.
- Load config from `~/.form/config.toml`, `OPENAI_API_KEY`, and defaults.
- Load context files from `FORM.md` then `AGENTS.md` at the workspace root.
- Provide reusable path-safety helpers for later read/search/write/edit tools.

**Non-Goals:**
- No provider calls, chat loop, sessions, read/search tool execution, mutation tools, approval prompts, WIT, Wasmtime, or plugin execution.
- No recursive context discovery or `OTHER_AGENT.md` loading.

## Decisions

- Keep modules inside `form-cli`: `workspace`, `config`, and `resources`. Existing architecture says internal modules stay in `form-cli` until a crate boundary earns itself.
- Detect git root by walking ancestors for a `.git` entry, then canonicalizing that directory. This avoids shelling out to git and works for normal repositories and test fixtures.
- Parse only the config keys M1 needs. A tiny `key = "value"` parser for `model` avoids adding a TOML dependency before config complexity exists.
- Resolve paths with `std::fs::canonicalize` for existing targets. For new files, canonicalize the nearest existing parent and reject escaping parents before any future mutation creates missing segments.
- Track secret-deny names and prefixes in constants so later tools can reuse the exact list.

## Risks / Trade-offs

- Minimal config parsing does not implement full TOML semantics → acceptable while only `model = "..."` is needed; replace with `toml` crate when nested config exists.
- Walking for `.git` does not consult `git rev-parse` worktree metadata → enough for M1 root detection; improve when worktree edge cases become product-critical.
- Secret-deny list is conservative but incomplete → tests cover the named M1 patterns; later read/search can add allowlists or richer policy when real tools exist.
