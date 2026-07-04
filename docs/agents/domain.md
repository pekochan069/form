# Domain Docs

How the engineering skills should consume this repo's domain documentation when exploring the codebase.

## Layout

This repo uses a **multi-context** domain docs layout.

A root `CONTEXT-MAP.md` points to one `CONTEXT.md` per context, typically in directories such as `src/<context>/CONTEXT.md`. System-wide architectural decisions live in `docs/adr/`; context-specific decisions may live in `src/<context>/docs/adr/`.

## Before exploring, read these

- **`CONTEXT-MAP.md`** at the repo root. Use it to identify which context docs are relevant to the task.
- **Relevant per-context `CONTEXT.md` files** named by `CONTEXT-MAP.md`.
- **`docs/adr/`** for system-wide decisions that touch the area you're about to work in.
- **`src/<context>/docs/adr/`** for context-scoped decisions, when present.

If any of these files don't exist, **proceed silently**. Don't flag their absence; don't suggest creating them upfront. The `/domain-modeling` skill creates them lazily when terms or decisions actually get resolved.

## File structure

Multi-context repo:

```text
/
├── CONTEXT-MAP.md
├── docs/adr/                          ← system-wide decisions
└── src/
    ├── ordering/
    │   ├── CONTEXT.md
    │   └── docs/adr/                  ← context-specific decisions
    └── billing/
        ├── CONTEXT.md
        └── docs/adr/
```

## Use the glossary's vocabulary

When your output names a domain concept (in an issue title, a refactor proposal, a hypothesis, a test name), use the term as defined in the relevant `CONTEXT.md`. Don't drift to synonyms the glossary explicitly avoids.

If the concept you need isn't in the glossary yet, that's a signal — either you're inventing language the project doesn't use (reconsider) or there's a real gap (note it for `/domain-modeling`).

## Flag ADR conflicts

If your output contradicts an existing ADR, surface it explicitly rather than silently overriding:

> _Contradicts ADR-0007 (event-sourced orders) — but worth reopening because…_
