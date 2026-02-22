# Learnings

## Agent 2 Learnings (2026-02-22)

### Key Discoveries

1. **Test Flakiness**: Initial test run showed 1 failure (`test_truncated_data_returns_error`), but re-running the full suite passed all 71 tests. This suggests potential test isolation issues or race conditions.

2. **Test Quality Issue**: The failing test `test_truncated_data_returns_error` in `tempest-decode/tests/synthetic_radial_test.rs` has NO assertions - it just prints results. The test was written as exploratory/incomplete and never had proper assertions added.

3. **Build/Test Stats**:
   - Workspace build: ✅ PASS
   - Unit tests: 59 passed
   - Integration tests: 12 passed
   - Total: 71 tests passing

4. **Agent Coordination**:
   - Agent 1 (Project Setup): ✅ Complete
   - Agent 2 (Core Decoder): ✅ Complete  
   - Agent 3 (Data Parsing): 🔄 In Progress (2 pending items)
   - Agent 4 (Testing & CI): ✅ Complete

5. **Sprint State**: Agent 3 still has work to do. My verification is complete but sprint is NOT complete - waiting on Agent 3.

### Patterns That Work

- Using `cargo test --workspace` to run all tests across the workspace
- The test structure with separate unit tests in lib.rs and integration tests in tests/ works well
- DISCLI skill successfully sends progress updates to Discord

### Gotchas Encountered

- Tests can appear to fail on first run but pass on retry (flaky tests)
- Test files without assertions will compile and "pass" but don't actually verify anything
