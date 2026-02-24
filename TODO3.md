# TODO3 - Agent 3

> Sprint: 22
> Focus Area: Cache Module Design Debt
> Last Updated: 2026-02-24T15:08Z

## Tasks

- [x] [DD-035] Fix expect() in Production Code
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md
  - Scope: Replace `expect()` with proper error handling in cache.rs line 101. Use `ok_or_else()` or similar to handle the Result properly. Fix LruCache capacity initialization.

- [x] [DD-034] Fix Cloning in Loop
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md
  - Scope: Fix cloning in loop in cache.rs lines 161-162. Create owned string once: `let key_owned = key.to_string();` then reuse for both CacheEntry::new and lru.put calls.

- [x] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_3' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
