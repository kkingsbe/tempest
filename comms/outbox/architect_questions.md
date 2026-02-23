# Architect Questions - PRD Ambiguities

> Generated: 2026-02-23
> Context: Sprint 7 periodic architect review

## Questions for Product Team

The following requirements in PRD.md need clarification before implementation:

### 1. F3: Radar Rendering - Range-folding corrections
- **Line ~107**: "Apply beam height and range-folding corrections for accurate geographic placement"
- **Issue**: No specification of how range-folding should be detected or corrected algorithmically
- **Question**: What algorithm should be used for range-folding detection and correction?

### 2. F4: Timeline - Prefetching algorithm
- **Line ~129**: "Intelligently prefetch adjacent scans"
- **Issue**: No specific algorithm defined
- **Question**: How many scans ahead should we prefetch? Should it adapt to bandwidth or playback direction?

### 3. F1: Base Map - Dark theme implementation
- **Line ~70**: "Dark theme map style optimized for weather radar overlay visibility"
- **Issue**: No specific color scheme or definition provided
- **Question**: What specific colors or CSS filters should be used for the dark theme?

### 4. Testing - Git LFS setup
- **Line ~166**: Mentions "test fixtures stored via Git LFS"
- **Issue**: No `.gitattributes` file exists in the repository
- **Question**: Should we set up Git LFS? What files should be tracked?

### 5. Testing - Docker/MinIO
- **Lines ~223, ~279, ~429**: Requires MinIO in Docker for integration tests
- **Issue**: No Dockerfile or Docker Compose configuration exists
- **Question**: Should we create Docker setup for integration tests, or use a different approach?

### 6. Testing - Headless test harness
- **Line ~277**: "Use a headless test harness that drives the application without a visible window"
- **Issue**: No implementation specification
- **Question**: Should we use an existing library (headless_whale) or build custom?

### 7. Testing - Golden images
- **Lines ~257-269**: References golden reference images for visual regression
- **Issue**: No golden images exist in repository
- **Question**: Should we create golden images manually or generate them?

### 8. Phase Integration
- **Lines ~478-500**: Phase 5 defines Interactive Base Map
- **Issue**: No explicit final Phase 6 for complete app integration
- **Question**: Should there be a final integration phase combining Base Map + Radar + Timeline + S3 Fetch?

## Question: Ambiguous PRD Requirements

The following PRD requirements need clarification before they can be added to the BACKLOG:

### 1. E2E Test Harness (Related to PRD Phase 3-4)
The PRD mentions "headless test harness with programmatic input" but doesn't specify the approach. 
- Should we use an existing library (e.g., `specta`, `insta`) or build a custom solution?
- What specific user interactions need to be testable?

### 2. Memory Profiling Target
The PRD non-functional requirements mention "<500MB memory" for the overall application, but there's no specific target for memory profiling during development.
- What's the acceptable memory ceiling for the application during normal operation?
- Should we track memory usage per component (decode, render, fetch)?

Please advise so we can add these items to the BACKLOG with appropriate scope.
