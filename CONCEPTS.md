# Concepts

Shared domain vocabulary for this project — entities, named processes, and status concepts with project-specific meaning. Seeded with core domain vocabulary, then accretes as ce-compound and ce-compound-refresh process learnings; direct edits are fine. Glossary only, not a spec or catch-all.

## Form Runtime

### Form
A Rust-native coding-agent harness where the host owns model calls, tool mediation, session persistence, approval decisions, and audit evidence.

### Form Core
The shared contract layer for Form domain data that other Form components use to agree on messages, sessions, tools, approvals, provider exchange, audit evidence, limits, patches, and plugin metadata.

### Form CLI
The command-line host surface for Form, responsible for local workspace orientation, configuration, resource loading, session storage, and user-facing runtime commands.

### Session Entry
An append-only record in a Form conversation timeline with stable tree identity plus a kind-specific payload.

A Session Entry always carries schema, session, branch, entry, and parent identity so later branching features can be added without rewriting old history.

### Host Tool
A capability exposed by the Form host to the agent loop, mediated by the host rather than executed directly by model or plugin code.

### Provider
The adapter boundary between Form and an LLM API, translating Form's internal request/result contract to an external model service.

## Safety and Extension Boundaries

### Approval
A host decision record that allows or denies a proposed mutation or shell action before the action can affect the workspace.

### Patch Proposal
A requested workspace change represented as data for host validation and approval before any mutation occurs.

### Plugin Manifest
The declaration an extension provides so Form can inspect its identity, commands, component artifact, and requested permissions before considering execution.
