# TODO1 - Agent 1

> Sprint: 2
> Focus Area: Decoder & Color Tables
> Last Updated: 2026-02-22

## Tasks

- [x] Implement radial data block parsing (REF, VEL, SW moments)
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md, ./skills/rust-best-practices/SKILL.md, ./skills/coding-guidelines/SKILL.md
  - Scope: Parse radial data blocks for Reflectivity, Velocity, and Spectrum Width from NEXRAD Archive2 data. Should be in tempest-decode/src/
  - Status: COMPLETE - 71 tests passing (implementation already existed)

- [x] Implement color tables for radar moments
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md
  - Scope: Define color lookup tables for REF, VEL, SW moments in tempest-render-core. Use NEXRAD standard color maps.
  - Status: COMPLETE - Implemented NWS standard color tables with 12 tests passing

- [x] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_1' with the current date.
  - Status: COMPLETE - 83 tests passing, build succeeds, format OK

- [x] Run download_fixtures.sh script to fetch NEXRAD test fixtures
  - Scope: Execute `./download_fixtures.sh` in a Linux/WSL environment to download 10 NEXRAD Archive2 files from AWS S3 to tempest-decode/tests/fixtures/. Script handles both noaa-nexrad-level2 and unidata-nexrad-level2 buckets, decompresses .gz files, and generates Truncated.ar2v (50KB from fixture 01).
  - Status: COMPLETE - Downloaded 10 fixtures (some with corrected S3 paths), decompressed 4 .gz files, generated Truncated.ar2v (50KB)
