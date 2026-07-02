# M13: Checkpoint and Store MVP

Status: Not started
Depends on: See `00-index.md`
RFC sections: See `traceability.md`

## Objective

Implement Windows-safe product checkpoint semantics using simple disk artifacts before CAS/NBD porting.

## Scope

- Define immutable base rootfs plus per-sandbox writable/checkpoint artifact strategy.
- Create/list/restore/delete checkpoints on Windows.
- Avoid Unix-socket NBD dependency for MVP.
- Keep checkpoint metadata compatible enough for future migration or clearly versioned.
- Protect base image immutability.

## Out of scope

- Do not port full CAS/NBD unless separately scoped.
- Do not rely on QEMU live snapshots as the product checkpoint contract.
- Do not mutate base rootfs in place.

## Likely files / crates

- `crates/lsb-store` Windows path
- `crates/lsb-cli/src/checkpoint.rs`
- `Windows backend disk config`

## Design notes

- Preserve existing macOS behavior unless the milestone explicitly states otherwise.
- Keep Windows-specific implementation behind platform/backend boundaries.
- Prefer precise capability errors over silent degradation.
- Update `state.md` when implementation reveals a better file layout or dependency.

## Tests to add or update

The specific tests should match the implementation, but this milestone must include enough validation to satisfy the acceptance criteria below. Prefer unit/golden/fake tests before requiring self-hosted integration tests.

## Acceptance criteria

- [ ] Create/list/delete checkpoint tests.
- [ ] Restore smoke test after exec/file mutation.
- [ ] Base image remains unchanged.
- [ ] Checkpoint errors mention Windows MVP limitations clearly.

## Coding-agent prompt

```text
You are implementing M13: Checkpoint and Store MVP for the LocalSandbox Windows QEMU + WHPX port.

Read first:
- docs/windows-port/rfc-qemu-whpx.md
- docs/windows-port/AGENTS.md
- docs/windows-port/agent-workspace/state.md
- docs/windows-port/agent-workspace/decisions.md
- docs/windows-port/agent-workspace/milestones/13-checkpoint-store-mvp.md

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
