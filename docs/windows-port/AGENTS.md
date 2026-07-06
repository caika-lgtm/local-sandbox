# AGENTS.md: Windows Port Rules

This file scopes Codex-agent behavior for the LocalSandbox Windows backend.

## Read first

For Windows backend work, read these before editing:

1. `docs/windows-port/README.md`
2. `docs/windows-port/mvp-handoff.md`
3. `docs/windows-port/decisions.md`
4. `docs/windows-port/architecture.md`
5. Relevant source files and tests

The RFC at `docs/windows-port/rfc-qemu-whpx.md` is preserved as design history.
Use it for rationale, not as a fresher status source than the MVP handoff.

## Non-negotiable semantics

- The Windows backend runs the existing Linux guest model.
- QEMU + WHPX is the supported Windows backend. Do not introduce HCS, Hyper-V
  Manager VMs, WSL2, Docker, or raw WHP VMM work without a new accepted
  decision.
- Production Windows runs require WHPX. TCG may exist only behind an explicit
  hidden/debug diagnostic flag or environment variable.
- Public CLI, Rust SDK, and Node API shape should stay platform-neutral unless a
  new accepted decision permits otherwise.
- Default Windows sandboxes have no guest NIC and no arbitrary outbound network.
- Allowlisted egress and secret substitution must remain LocalSandbox policy,
  not QEMU NAT or Windows Firewall policy.
- Host source mounts are read-only from the product perspective; guest writes
  are isolated unless explicitly exported.
- QMP, control pipes, debug sockets, proxy links, and port-forward listeners
  must stay private or loopback-only unless a new accepted decision permits
  otherwise.
- Do not expose secrets in logs, QEMU argv examples, panic messages, tests,
  diagnostics, snapshots, or checkpoint metadata.

## Hardware testing

- Use `./scripts/win-gh-test check` after platform or portability changes.
- Use `./scripts/win-gh-test unit` before opening a PR that touches Windows code.
- Use `./scripts/win-gh-test smoke` after QEMU, WHPX, VM lifecycle, transport,
  guest-control, mount, checkpoint, or networking changes.
- Use `./scripts/win-gh-test e2e` when expanding the broader Windows hardware
  suite.
- The helper requires a clean committed working tree because GitHub Actions can
  only test pushed commits.
- Do not add automatic `pull_request` triggers for the self-hosted Windows
  hardware runner.

## Documentation updates

Update the durable Windows docs when behavior changes:

- `mvp-handoff.md`: current support status, limitations, validation evidence,
  and production-readiness gaps.
- `decisions.md`: only for accepted architectural/product decisions.
- `validation.md`: test commands, CI lanes, runner assumptions, and smoke scope.
- `diagnostics.md`: diagnostic artifacts, redaction, and failure triage.
- `risk-register.md`: active, accepted, retired, or newly discovered risks.
- `future-work.md`: follow-on work that is out of the current change scope.
