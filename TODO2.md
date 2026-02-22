# TODO2 - Agent 2

> Sprint: 3
> Focus Area: Polling & Caching
> Last Updated: 2026-02-22

## Tasks

- [ ] Implement real-time polling for new volume scans
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md, ./skills/rust-best-practices/SKILL.md
  - Scope: Configurable poll interval, stream new scans. Monitor S3 bucket for new data.

- [ ] Implement local disk cache with LRU eviction
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Cache to ~/.config/tempest/cache/, configurable size limit. Use LRU eviction policy.

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_2' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
