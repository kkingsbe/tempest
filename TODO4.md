# TODO4 - Agent 4

> Sprint: 9
> Focus Area: Phase 8 - Offline Mode & Application Polish
> Last Updated: 2026-02-23

## Tasks

- [ ] Implement Cache Management UI
  - 📚 SKILLS: `./skills/frontend-design/SKILL.md`, `./skills/rust-engineer/SKILL.md`
  - Scope: Display current cache size, manual clear button, configurable cache limit. Integrate with tempest-fetch's existing cache.

- [ ] Implement Offline Mode Detection
  - 📚 SKILLS: `./skills/rust-engineer/SKILL.md`
  - Scope: Auto-detect network connectivity changes. Allow browsing cached data when offline. Show clear offline indicator in UI.

- [ ] Implement Application Polish
  - 📚 SKILLS: `./skills/frontend-design/SKILL.md`, `./skills/rust-best-practices/SKILL.md`
  - Scope: Keyboard shortcuts (pan, zoom, play/pause), proper window title, config file support (~/.tempest/config.toml).

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_4' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
