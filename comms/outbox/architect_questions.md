# Architect Questions - PRD Ambiguities

> Generated: 2026-02-24
> Context: PRD Review - Items requiring product clarification

## Questions for Product Team

The following ambiguous PRD items need clarification before implementation can proceed:

| ID | Item | Ambiguity | Question |
|----|------|-----------|----------|
| PRD-1 | Super-resolution data (F3 line 89) | Performance targets unclear | Should super-resolution have separate decode/render latency targets? |
| PRD-2 | E2E test harness (line 277) | "Playwright or native harness" is ambiguous | Which harness should be used? |
| PRD-3 | Prefetch budget (F4 line 578) | No default value specified | What should be the default max concurrent fetches? |
| PRD-4 | Golden image coverage (lines 259-262) | Which fixtures/map views to cover? | Should all 10 fixtures be covered in visual regression? |
| PRD-5 | Range-folding corrections (F3 line 107) | No implementation detail | Simple "no data" mask or actual correction algorithm? |

---

## Detailed Context

### PRD-1: Super-resolution data (F3 line 89)
- **Current**: Performance requirements mention general decode/render latency targets
- **Issue**: Super-resolution data may require different performance characteristics
- **Impact**: Affects pipeline.rs and renderer.rs implementation decisions

### PRD-2: E2E test harness (line 277)
- **Current**: "Playwright or native harness" - two options mentioned
- **Issue**: No clear decision on which approach to use
- **Impact**: Affects tempest-app/tests/e2e/ implementation

### PRD-3: Prefetch budget (F4 line 578)
- **Current**: Prefetch algorithm defined but no default budget
- **Issue**: No max concurrent fetches value specified
- **Impact**: Affects prefetch.rs configuration

### PRD-4: Golden image coverage (lines 259-262)
- **Current**: Visual regression testing mentioned with golden images
- **Issue**: Which fixtures and map views should be covered?
- **Impact**: Affects tempest-render tests and fixture generation

### PRD-5: Range-folding corrections (F3 line 107)
- **Current**: "Apply beam height and range-folding corrections"
- **Issue**: No implementation detail provided
- **Impact**: Affects pipeline.rs algorithm implementation

---

Please advise on these items so implementation can proceed with clear requirements.
