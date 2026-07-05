## Rules

- Don’t fight errors. If same error appears twice, research 3-5 likely fixes, choose the smallest safe fix, then implement.
- Folow rules in `docs/rules/**`
- Do not perform unrequested side effects. Only change code/docs/files explicitly needed for user request.
- Do not mutate GitHub issues, PRs, labels, comments, project maps, git branches, commits, or other external state unless user explicitly asks for that specific mutation.
- If a skill workflow suggests tracker or git writes but user did not explicitly request them, stop before that step and ask.

## Workflow

My typical workflow goes like this:

`skill:wayfinder`
-> `skill:openspec-propose`
-> human review
-> `skill:openspec-apply-change` and `skill:implement`
-> `skill:code-review`
-> final humal review
-> `skill:ce-commit`
-> pr

If not asked with anything, follow this steps.

## Agent skills

### Issue tracker

Issues are tracked in GitHub Issues for `pekochan069/form`; external PRs are also treated as a triage request surface. See `docs/agents/issue-tracker.md`.

### Triage labels

The canonical triage labels use the default names, with supplemental type labels `bug` and `feature-request`. See `docs/agents/triage-labels.md`.

### Domain docs

This repo uses a multi-context domain docs layout. See `docs/agents/domain.md`.

`CONCEPTS.md` contains shared domain vocabulary for entities, named processes, and status concepts; relevant when orienting to the codebase or discussing domain concepts.

### Documented solutions

`docs/solutions/` contains documented solutions to past problems and workflow decisions, organized by category with YAML frontmatter (`module`, `tags`, `problem_type`). Relevant when implementing or debugging in documented areas.
