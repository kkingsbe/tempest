# TODO1 - Agent 1

> Sprint: 5
> Focus Area: Phase 2 - Color Tables for Radar Moments
> Last Updated: 2026-02-22T22:50:46Z

## Tasks

- [ ] Define REF (Reflectivity) color table for dBZ values
  - 📚 SKILLS: `./skills/rust-engineer.md`, `./skills/frontend-design.md`
  - Scope: Create dBZ color ramp from -30 to 75 dBZ with NWS standard colors
  
- [ ] Define VEL (Velocity) color table for radial velocity
  - 📚 SKILLS: `./skills/rust-engineer.md`, `./skills/frontend-design.md`
  - Scope: Create velocity color ramp with blue (-100 m/s) to red (+100 m/s) gradient

- [ ] Define ZDR (Differential Reflectivity) color table
  - 📚 SKILLS: `./skills/rust-engineer.md`, `./skills/frontend-design.md`
  - Scope: Create ZDR color ramp from -7.5 to +7.5 dB

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_1' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
