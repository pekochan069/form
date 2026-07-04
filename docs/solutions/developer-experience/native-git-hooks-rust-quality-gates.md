---
title: Native Git Hooks for Rust Quality Gates
date: 2026-07-05
category: developer-experience
module: Form
problem_type: developer_experience
component: development_workflow
severity: low
applies_when:
  - "A Rust repository needs local pre-push quality gates without requiring a hook-manager dependency."
  - "Contributors can opt in with a one-time repository setup command."
  - "Checks are standard Cargo commands available to Rust contributors."
tags: [git-hooks, pre-push, rust, cargo, local-checks]
---

# Native Git Hooks for Rust Quality Gates

## Context

The project needed lefthook-style local pre-push checks, but not every contributor can be assumed to install lefthook. Git cannot auto-enable repository hooks on clone for security reasons, so local hooks need an explicit one-time setup step.

Session history found no directly relevant prior solution beyond general Form planning context.

## Guidance

Use native Git hooks with a tracked hook directory and installer script. Keep the hook boring: run the same Rust checks contributors already use locally.

`.githooks/pre-push`:

```sh
#!/bin/sh
set -eu

cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

`scripts/install-hooks.sh`:

```sh
#!/bin/sh
set -eu

git config core.hooksPath .githooks
echo "Git hooks installed: core.hooksPath=.githooks"
```

Make both scripts executable:

```sh
chmod +x .githooks/pre-push scripts/install-hooks.sh
```

Each contributor runs this once per clone:

```sh
./scripts/install-hooks.sh
```

## Why This Matters

This gives contributors a dependency-free local safety net without adding lefthook or another hook manager to the repo. It also keeps setup explicit, which matches Git's security model: cloned repositories should not silently install executable hooks.

CI is still the enforcement layer. Local hooks improve feedback speed; they do not replace required checks before merge.

## When to Apply

- Use this when checks are simple shell commands and every contributor already has the toolchain.
- Use this when avoiding a hook-manager dependency matters more than auto-install convenience.
- Do not rely on this alone when checks must be mandatory for all contributors; use CI for that.

## Examples

Verify the setup after adding the files:

```sh
./scripts/install-hooks.sh
./.githooks/pre-push
```

Expected checks:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

## Related

- GitHub issue #2: M0 Rust workspace skeleton required local paths for `cargo fmt`, `cargo clippy`, and `cargo test`.
- `openspec/changes/m0-create-rust-workspace-skeleton/` records the Rust quality-gate commands this hook runs.
