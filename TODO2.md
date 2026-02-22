# TODO2 - Agent 2

> Sprint: 6
> Focus Area: Storage & Resilience - Cache & Retry Logic
> Last Updated: 2026-02-22T23:51:15Z

## Tasks

- [ ] Implement local disk cache with LRU eviction
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Cache to ~/.config/tempest/cache/, configurable size limit
  
- [ ] Implement retry logic with exponential backoff
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md
  - Scope: Handle transient failures gracefully
  
- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_2' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
