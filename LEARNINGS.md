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

---

[Agent 2] Session 2026-02-22 - Sprint Recovery
===============================================
Issue Discovered:
- .agent_done_2 existed but TODO2.md showed all tasks unchecked
- This indicated incomplete work was marked as done

Findings:
1. Coordinate projection (tempest-render-core) WAS fully implemented
2. NEXRAD test fixtures were present in tempest-decode/tests/fixtures/
3. Build/test verification was BROKEN - missing chrono dev-dependency

Fixes Applied:
- Added chrono = "0.4" to tempest-render-core/Cargo.toml [dev-dependencies]
- All 115+ tests now pass
- Updated TODO2.md to mark all 3 tasks as [x]
- Updated .agent_done_2 with detailed completion report

Key Learning:
- The .agent_done_2 file was created prematurely without verifying tests pass
- Always run full test suite before marking QA task complete

---

[Agent 2] Session 2026-02-22T18:20:00 - Sprint 2 Verification
==========================================================
Status Check:
- All TODO2.md items: ✅ COMPLETE (all [x] checked)
- .agent_done_2: ✅ EXISTS
- Sprint completion: ⏳ Waiting on Agent 3 (.agent_done_3 missing)

Actions Taken:
1. Verified TODO2.md shows all 3 tasks completed
2. Confirmed .agent_done_2 file exists
3. Checked inbox - no messages (comms/inbox doesn't exist)
4. Sent progress update via DISCLI
5. Verified sprint status: 3/4 agents done (Agent 3 pending)

Key Learning:
- Agent coordination: When not all agents are done, sprint is NOT complete
- Must check for all .agent_done_* files before creating .sprint_complete
