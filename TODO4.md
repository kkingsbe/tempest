# Sprint 24 - Agent 4

## Tasks

- [x] [BUILDFIX] Fix broken build / failing tests
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md, ./skills/test-driven-development/SKILL.md
  - 🎯 Goal: Build compiles with zero errors AND full test suite passes
  - 📂 Files: tempest-app/tests/e2e/gui_harness_test.rs
  - 🧭 Context: The test suite fails to compile with the following error:

    **Error [E0599]**: `no method named '_is_online' found for struct 'tempest_app::OfflineIndicator'` in gui_harness_test.rs line 659

    ```
    let initial_online = harness.state.offline_indicator._is_online();
    ```

    The compiler suggests using `is_online` instead of `_is_online`.

    Fix: Change `_is_online()` to `is_online()` in the test file.
  - ✅ Acceptance: `cargo build` exits 0; `cargo test` exits 0

- [ ] AGENT QA: Run cargo build FIRST to verify compilation. Fix ALL build errors. Then run full test suite. If ALL errors fixed and tests pass, create '.agent_done_4' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
