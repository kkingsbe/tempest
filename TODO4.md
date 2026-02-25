- [ ] [BUILDFIX] Fix broken build / failing tests
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md, ./skills/test-driven-development/SKILL.md
  - 🎯 Goal: Build compiles with zero errors AND full test suite passes
  - 📂 Files: tempest-app/src/test_utils.rs, tempest-app/tests/e2e/gui_harness_test.rs
  - 🧭 Context: The test suite fails to compile with the following errors:
    
    **Error 1 [E0255]**: `PanDirection` is defined multiple times in test_utils.rs line 61 - conflicts with import
    **Error 2 [E0252]**: `Message` is reimported multiple times in test_utils.rs line 137
    **Error 3 [E0252]**: `Moment` is reimported multiple times in test_utils.rs line 138
    **Error 4 [E0433]**: Cannot find `test_utils` module in gui_harness_test.rs line 13 - module not properly exported
    **Error 5 [E0308]**: Type mismatch between `gui_harness::State` and crate `State` in test_utils.rs line 92

    These are compilation errors preventing tests from running. Fix the import conflicts in test_utils.rs and ensure the test_utils module is properly exported.
  - ✅ Acceptance: `cargo build` exits 0; `cargo test` exits 0

<!-- No tasks assigned this sprint -->
