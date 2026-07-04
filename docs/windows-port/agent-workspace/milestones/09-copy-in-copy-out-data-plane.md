# M09: Copy-In/Copy-Out Data Plane

Status: In progress
Depends on: See `00-index.md`
RFC sections: See `traceability.md`

## Objective

Implement safe host-to-guest import and guest-to-host export for files and directories.

## Scope

- Use existing file protocol operations where possible.
- Normalize and validate Windows host paths.
- Reject path traversal and unsafe destination escapes.
- Support files, directories, empty directories, and reasonable large files.
- Define symlink/junction behavior explicitly.

## Out of scope

- Do not implement live shared mounts.
- Do not support direct host writes.
- Do not follow dangerous reparse points without explicit policy.

## Likely files / crates

- `crates/lsb-vm` file APIs
- `crates/lsb-guest` file handlers
- `crates/lsb-platform/src/windows_x86_64/fs/copy.rs`

## Design notes

- Preserve existing macOS behavior unless the milestone explicitly states otherwise.
- Keep Windows-specific implementation behind platform/backend boundaries.
- Prefer precise capability errors over silent degradation.
- Update `state.md` when implementation reveals a better file layout or dependency.

## Tests to add or update

The specific tests should match the implementation, but this milestone must include enough validation to satisfy the acceptance criteria below. Prefer unit/golden/fake tests before requiring self-hosted integration tests.

## Acceptance criteria

- [ ] Copy-in/out tests for files/dirs/large files.
- [ ] Path traversal rejection tests.
- [ ] Windows symlink/junction behavior documented.
- [ ] Explicit export does not overwrite unexpected host paths.

## Coding-agent prompt

```text
You are implementing M09: Copy-In/Copy-Out Data Plane for the LocalSandbox Windows QEMU + WHPX port.

Read first:
- docs/windows-port/rfc-qemu-whpx.md
- docs/windows-port/AGENTS.md
- docs/windows-port/agent-workspace/state.md
- docs/windows-port/agent-workspace/decisions.md
- docs/windows-port/agent-workspace/milestones/09-copy-in-copy-out-data-plane.md

Implement only this milestone. Preserve public CLI/SDK/Node APIs and existing macOS behavior. Add tests required by the milestone. Do not implement later milestones opportunistically. Update state.md and this milestone handoff before finishing.
```

## Security checklist

Complete the checklist in `../security-checklist.md`. Record any new risk in `../risk-register.md`.

## Handoff

- Branch/PR: TBD
- Summary: TBD
- Tests run: TBD
- Debug artifacts: TBD
- New decisions: TBD
- New risks: TBD
- Next milestone: TBD
