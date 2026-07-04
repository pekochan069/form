# Triage Labels

The skills speak in terms of five canonical triage roles. This file maps those roles to the actual label strings used in this repo's issue tracker.

## State labels

| Label in mattpocock/skills | Label in our tracker | Meaning                                  |
| -------------------------- | -------------------- | ---------------------------------------- |
| `needs-triage`             | `needs-triage`       | Maintainer needs to evaluate this issue  |
| `needs-info`               | `needs-info`         | Waiting on reporter for more information |
| `ready-for-agent`          | `ready-for-agent`    | Fully specified, ready for an AFK agent  |
| `ready-for-human`          | `ready-for-human`    | Requires human implementation            |
| `wontfix`                  | `wontfix`            | Will not be actioned                     |

When a skill mentions a role (e.g. "apply the AFK-ready triage label"), use the corresponding label string from this table.

## Supplemental type labels

These labels are available for issue/PR type classification in addition to the state labels above:

| Type | Label in our tracker | Meaning |
| ---- | -------------------- | ------- |
| Bug | `bug` | Something is broken or behaving incorrectly |
| Feature request | `feature-request` | A request for new functionality or a product improvement |

Edit the right-hand column to match whatever vocabulary you actually use.
