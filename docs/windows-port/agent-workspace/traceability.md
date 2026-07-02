# RFC Traceability Matrix

Use this matrix to connect implementation work back to the RFC.

| RFC section | Implementation milestones | Notes |
|---|---|---|
| 3 Executive Summary | All | MVP path and non-changing product semantics. |
| 4 Goals | All | Each milestone should advance at least one goal. |
| 5 Non-goals | All | Especially no HCS, no Windows guest, no default bridged/user networking. |
| 6 Current LocalSandbox Architecture | M01, M08-M13 | Preserve existing abstractions and behavior. |
| 7 Proposed Windows Architecture | M01-M06 | Backend shape, QEMU process, guest asset integration. |
| 8 Why QEMU + WHPX | M02-M05 | Preflight and WHPX-only execution. |
| 9 Hyper-V/WHPX/QEMU Mental Model | M02, M04, M05 | Diagnostics and error messages. |
| 10 Boot Design | M03-M05 | Direct Linux boot, rootfs, serial logs. |
| 11 Control Plane Design | M06-M08, M11 | Virtio-serial, readiness, exec, forwarding. |
| 12 Data Plane / File and Mount Design | M09-M10, M13 | Copy-in/out, mount MVP, checkpoint interactions. |
| 13 Networking and Security Design | M11-M12 | No network default, proxy egress, secrets. |
| 14 Checkpointing and Store Design | M13 | Simple Windows artifact path first. |
| 15 Rust Architecture Changes | M01-M04, all | Traits, module boundaries, errors. |
| 16 Implementation Plan | M01-M15 | Milestone source. |
| 17 Testing Strategy | All, M15 | Unit/golden/integration/CI expectations. |
| 18 Debugging and Diagnostics | M02, M04-M07, M15 | Doctor command, artifacts, serial logs. |
| 19 Security Considerations | All | Checklist required at handoff. |
| 20 Open Questions | Experiments | Track future virtiofs/vsock/ARM64 decisions. |
| 21 Alternatives Considered | M02, future | Do not implement rejected alternatives in MVP. |
| 22 Appendix A: Example QEMU Commands | M03-M05 | Convert to golden argv tests. |
| 23 Appendix B: Coding-Agent Task Backlog | All | This workspace decomposes backlog into issues. |
| 24 Appendix C: Glossary | All | Shared terms for review and docs. |
