# TODO1 - Agent 1

> Sprint: 2
> Focus Area: Decoder & Color Tables
> Last Updated: 2026-02-22

## Tasks

- [ ] Implement radial data block parsing (REF, VEL, SW moments)
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md, ./skills/rust-best-practices/SKILL.md, ./skills/coding-guidelines/SKILL.md
  - Scope: Parse radial data blocks for Reflectivity, Velocity, and Spectrum Width from NEXRAD Archive2 data. Should be in tempest-decode/src/

- [ ] Implement color tables for radar moments
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md
  - Scope: Define color lookup tables for REF, VEL, SW moments in tempest-render-core. Use NEXRAD standard color maps.

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_1' with the current date.
