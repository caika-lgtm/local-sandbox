# Windows Port Agent Workspace

This workspace coordinates sequential Codex-agent implementation of the LocalSandbox Windows backend described in `docs/windows-port/rfc-qemu-whpx.md`.

It is intentionally operational. The RFC explains the design. These files tell agents how to implement it without rediscovering scope, decisions, test expectations, or the current state of the port.

## Directory layout

```text
docs/windows-port/
  rfc-qemu-whpx.md                 # Canonical design document
  AGENTS.md                        # Codex-agent rules for this feature area
  agent-workspace/
    README.md                      # This file
    state.md                       # Live implementation state and handoff log
    decisions.md                   # Accepted decisions and change process
    code-map.md                    # Repo/crate map and expected boundaries
    architecture-boundaries.md     # Backend traits and layering rules
    playbook.md                    # Per-issue agent workflow
    validation.md                  # Test, smoke, and CI strategy
    diagnostics.md                 # Failure-mode debugging guide
    security-checklist.md          # Required security checks per milestone
    risk-register.md               # Tracked implementation risks
    traceability.md                # RFC section to milestone mapping
    milestones/
      00-index.md                  # Milestone sequence overview
      01-windows-compile-stubs.md
      02-qemu-discovery-preflight.md
      ...
    templates/
      codex-issue-prompt.md
      milestone-handoff.md
      decision-record.md
      test-report.md
```

## How to use this workspace

For each implementation issue:

1. Read the RFC section referenced by the milestone.
2. Read `state.md` to learn what is already done.
3. Read `decisions.md` and do not reopen accepted decisions unless blocked.
4. Work only on the current milestone document.
5. Keep changes small enough for review.
6. Add or update tests before declaring completion.
7. Update `state.md` and the milestone handoff before stopping.

Agents should not skip ahead. Later milestones rely on the tests, naming, and abstractions created by earlier milestones.

## Non-negotiable product semantics

The Windows port must preserve these semantics even if the implementation strategy differs from macOS:

- The guest is Linux.
- The public LocalSandbox CLI/SDK semantics remain stable.
- No guest network device is present by default.
- Allowlisted network access is enforced by LocalSandbox policy, not by trusting QEMU NAT.
- Secrets stay on the host and are substituted only through approved proxy policy.
- Host source data is read-only from the product perspective.
- Guest writes are isolated unless explicitly exported.
- Checkpoints are explicit product-level artifacts, not incidental QEMU process state.
- QEMU/QMP/control sockets are private to the owning user/session.

## Initial implementation strategy

The MVP is deliberately staged:

1. Make the repo compile on Windows with stubs and capability errors.
2. Discover and validate QEMU + WHPX.
3. Build deterministic QEMU argv generation.
4. Supervise QEMU with Windows process cleanup.
5. Boot the existing Linux guest with serial logs.
6. Add virtio-serial control transport using the existing `lsb-proto` framing.
7. Implement guest readiness, exec, file copy, mount MVP, port forwarding, policy-based networking, checkpoint MVP, packaging, and CI.

Do not implement VirtioFS, Hyper-V sockets, QEMU user networking by default, HCS, or Windows ARM64 as part of the MVP unless a new decision is recorded.
