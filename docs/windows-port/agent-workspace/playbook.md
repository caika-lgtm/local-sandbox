# Codex Agent Playbook

Use this workflow for every Windows-port issue.

## 1. Intake

Read, in order:

1. `docs/windows-port/rfc-qemu-whpx.md`
2. `docs/windows-port/AGENTS.md`
3. `docs/windows-port/agent-workspace/state.md`
4. `docs/windows-port/agent-workspace/decisions.md`
5. The assigned milestone file
6. Existing code for every file you plan to edit

Then restate the issue in one paragraph in your working notes. Do not ask the maintainer to repeat decisions that are already recorded.

## 2. Scope control

Before editing, identify:

- required files,
- likely tests,
- public API surfaces touched,
- platform-specific code paths,
- documentation updates required,
- risks from `risk-register.md` relevant to this milestone.

Do not opportunistically implement later milestones.

## 3. Implementation pattern

Preferred sequence:

1. Add narrow types/traits behind compile gates.
2. Add failing unit or golden tests where practical.
3. Implement minimal code to satisfy the milestone.
4. Add diagnostics and structured errors.
5. Verify macOS behavior is unchanged.
6. Verify Windows compile or smoke objective for the milestone.
7. Update workspace docs.

## 4. Review hygiene

Keep diffs reviewable:

- One concept per commit where possible.
- Do not reformat unrelated files.
- Do not rename public APIs unless required by the milestone.
- Add comments only where they explain backend-specific constraints.
- Prefer explicit capability errors over silent no-ops.

## 5. Test expectations

Every milestone should have at least one of:

- unit tests,
- golden argv tests,
- fake process tests,
- compile tests,
- guest protocol tests,
- boot smoke logs,
- security behavior tests,
- docs-only acceptance checks for scaffolding milestones.

Record all commands and results in `state.md` and, when useful, `templates/test-report.md`.

## 6. Handoff

At the end of the issue:

1. Fill the milestone handoff section.
2. Append a test evidence row to `state.md`.
3. Update milestone status.
4. Record new decisions only if approved.
5. List exact next milestone and any blockers.

Use `templates/milestone-handoff.md` when creating PR descriptions.

## 7. Stop conditions

Stop and report instead of improvising when:

- implementation requires changing an accepted decision,
- a public API change seems necessary,
- the milestone cannot be completed without a later milestone,
- WHPX/QEMU behavior contradicts the RFC,
- a security property cannot be preserved,
- tests require infrastructure not yet available.

Record the blocker in `state.md` and `risk-register.md`.
