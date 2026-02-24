# TODO1 - Agent 1

> Sprint: 22
> Focus Area: Coverage Enforcement & Performance Benchmarks
> Last Updated: 2026-02-24T15:08Z

## Tasks

- [ ] Coverage Ratchet Policy
  - 📚 SKILLS: ./skills/test-driven-development/SKILL.md, ./skills/rust-best-practices/SKILL.md
  - Scope: Define and implement coverage ratchet policy - automated enforcement that prevents coverage from decreasing. Set threshold at 1.5% decrease allowed.

- [ ] Performance Benchmark Baseline
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md, ./skills/test-driven-development/SKILL.md
  - Scope: Establish performance baselines per PRD: decode time <100ms, render <50ms, pipeline <500ms p95, memory <500MB. Document in project.

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_1' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
