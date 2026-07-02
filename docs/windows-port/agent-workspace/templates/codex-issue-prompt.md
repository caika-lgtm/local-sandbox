# Codex Issue Prompt Template

Copy this into a Codex-agent issue. Replace bracketed fields.

```text
You are working on the LocalSandbox Windows QEMU + WHPX port.

Milestone: [MNN - title]
Branch: [suggested branch]

Read first, in order:
1. docs/windows-port/rfc-qemu-whpx.md
2. docs/windows-port/AGENTS.md
3. docs/windows-port/agent-workspace/state.md
4. docs/windows-port/agent-workspace/decisions.md
5. docs/windows-port/agent-workspace/milestones/[milestone-file].md

Task:
[One-paragraph objective copied from the milestone.]

Constraints:
- Implement only this milestone.
- Preserve public CLI, Rust SDK, and Node API semantics unless this milestone explicitly says otherwise.
- Preserve existing macOS behavior.
- Keep Windows/QEMU details below platform/backend boundaries.
- Production Windows execution must require WHPX. Do not add normal TCG fallback.
- Do not enable guest networking by default.
- Do not log or expose secrets.

Likely files:
[List likely files from milestone.]

Required tests:
[List tests from milestone acceptance criteria.]

Acceptance criteria:
[Paste checklist from milestone.]

Before finishing:
- Run relevant tests and record commands/results.
- Update docs/windows-port/agent-workspace/state.md.
- Update the milestone handoff section.
- Add a decision record only if a new approved decision was required.
- Add or update risk-register.md if new risks were found.
```
