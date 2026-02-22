# Rust Testing Requirements

Master specification for testing strategy, coverage requirements, and quality standards across Rust codebases. Applies to CLI tools, web services, game projects (Bevy), libraries, and automation systems. The goal is **high first-try success in production** — tests must closely mirror production behavior wherever possible, not just exercise code paths in isolation.

---

## Testing Philosophy

Tests exist to catch bugs before production, not to hit coverage numbers. Every test should answer the question: "Would this have caught a real bug?" If the answer is no, the test is ceremony.

Three principles guide all testing decisions:

1. **Production fidelity** — prefer tests that exercise real behavior over mocked behavior. Mocks are a compromise, not a goal.
2. **Failure specificity** — when a test fails, the developer should immediately know what broke and where. Vague failures waste more time than no test at all.
3. **Determinism** — flaky tests erode trust in the entire suite. A test that passes 99% of the time is worse than no test because it teaches developers to ignore failures.

---

## Test Tiers

### Tier 1: Unit Tests

Unit tests validate individual functions, methods, and modules in isolation. They are fast, deterministic, and form the foundation of the test pyramid.

**Scope:** A single function or a small cluster of tightly-coupled functions within the same module.

**Location:** Inline `#[cfg(test)]` modules at the bottom of each source file.

```rust
// src/config/mod.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_timeout_valid_minutes() {
        assert_eq!(parse_timeout("5m"), Ok(Duration::from_secs(300)));
    }

    #[test]
    fn parse_timeout_rejects_negative() {
        assert!(parse_timeout("-5m").is_err());
    }
}
```

**Coverage requirement:** 80% line coverage minimum, 90% target.

**What to test:**
- All public API functions with valid inputs
- All documented error conditions and edge cases
- Boundary values (empty strings, zero, max values, off-by-one)
- Parsing and validation logic (config parsing, cron expressions, timeout formats)
- State transitions and business logic
- Serialization/deserialization round-trips
- Pure computation (math, transforms, algorithms)
- ECS component data transformations (for Bevy projects)

**What NOT to test at this level:**
- Trivial getters/setters with no logic
- Direct pass-throughs to external crates (test the integration instead)
- `main()` function wiring
- Rendering output, GPU behavior, or windowing (for game projects)
- Framework-managed scheduling (Bevy system ordering, async runtime internals)

**Naming convention:** `<function_under_test>_<scenario>_<expected_outcome>`

```rust
#[test]
fn validate_config_duplicate_agent_names_returns_error() { ... }

#[test]
fn parse_cron_every_six_hours_succeeds() { ... }

#[test]
fn velocity_component_clamps_to_max_speed() { ... }
```

---

### Tier 2: Integration Tests

Integration tests validate that modules work together correctly and that interactions with real external dependencies (filesystem, Docker, network, ECS worlds) behave as expected.

**Scope:** Multiple modules or a module + real external dependency.

**Location:** `tests/` directory at the project root.

```
tests/
├── common/
│   └── mod.rs              # Shared test utilities, fixtures, helpers
├── config_integration.rs
├── docker_integration.rs
├── scheduler_integration.rs
├── hooks_integration.rs
└── ecs_integration.rs      # For Bevy: system interaction tests
```

**Coverage requirement:** 60% line coverage minimum, 75% target. Integration tests cover the gaps that unit tests cannot — real I/O, cross-module data flow, and dependency interactions.

**What to test:**
- Config file loading from disk (real files, real filesystem)
- Cross-module data flow (config → scheduler → docker → metrics)
- Docker container lifecycle (build, start, stop, logs) against a real daemon
- Cron schedule evaluation with real time calculations
- Log file creation, rotation, and reading
- CLI argument parsing → config loading → command execution chains
- HTTP API interactions against a mock server (wiremock)
- Hook trigger matching → processing → action execution pipelines
- Metrics/state file persistence across restarts
- File-based coordination between modules (TODO files, signal files)
- Prompt file resolution relative to config directory
- ECS system integration (for Bevy: systems reading/writing shared resources and components in a real `World`)
- Plugin composition (for Bevy: multiple plugins interacting in a real `App`)
- Asset loading pipelines against real asset files

**What NOT to test at this level:**
- Individual function logic already covered by unit tests
- Third-party crate internals
- Visual/rendering correctness (use snapshot or visual regression tools separately)

**Feature gating:** Integration tests that require Docker, network access, GPU, or other external infrastructure must be gated behind feature flags so they don't break CI in constrained environments.

```rust
// tests/docker_integration.rs
#![cfg(feature = "integration")]

#[test]
fn build_agent_image_succeeds() {
    // Requires Docker daemon
}
```

```bash
# Run only unit tests (fast, no external deps)
cargo test --lib

# Run integration tests (requires Docker, network, etc.)
cargo test --features integration

# Run a specific integration test
cargo test --features integration --test docker_integration
```

**Timeout policy:** Every integration test must have an explicit timeout or be wrapped in a timeout harness. Default: 30 seconds. Docker tests: 120 seconds. Network tests: 60 seconds. ECS world tests: 10 seconds.

---

### Tier 3: End-to-End Tests

E2E tests validate the system from the user's perspective — running the actual binary or application with real configuration against real infrastructure. These are the highest-fidelity tests and the most expensive to run.

**Scope:** The compiled binary or application, invoked as a subprocess or launched as a full process, performing real work.

**Location:** `tests/e2e/` directory.

```
tests/e2e/
├── common/
│   └── mod.rs              # E2E helpers (binary runner, temp dir setup)
├── cli_e2e.rs              # CLI command smoke tests
├── scheduler_e2e.rs        # Full scheduler lifecycle
├── hooks_e2e.rs            # Discord hooks end-to-end
├── game_e2e.rs             # Headless game simulation (Bevy)
└── fixtures/
    ├── valid_config.toml
    ├── invalid_config.toml
    └── prompts/
        └── test-prompt.md
```

**Coverage requirement:** No line coverage target. E2E tests are measured by scenario coverage — every documented user workflow must have a corresponding E2E test.

**What to test (CLI/service projects):**
- Full command execution with valid and invalid inputs
- Scheduler lifecycle: start → trigger agent → collect metrics → stop
- Error paths: missing config, invalid arguments, unavailable dependencies
- Signal handling (SIGTERM, SIGINT graceful shutdown)
- Output formatting (log output, table display, metrics reports)

**What to test (game/Bevy projects):**
- Headless simulation runs (no GPU required) using Bevy's `MinimalPlugins` or headless mode
- Game state progression through multiple frames/updates
- Save/load round-trips with real files
- Deterministic replay from recorded inputs
- Startup → steady state → shutdown lifecycle

**Execution pattern (CLI):**

```rust
use std::process::Command;

fn gastown_cmd(workspace: &Path) -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_gastown"));
    cmd.current_dir(workspace);
    cmd
}

#[test]
fn validate_reports_missing_agent_name() {
    let output = gastown_cmd(&test_workspace)
        .arg("validate")
        .arg("--config")
        .arg("fixtures/invalid_config.toml")
        .output()
        .expect("failed to run gastown");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("agent name cannot be empty"));
}
```

**Execution pattern (Bevy headless):**

```rust
#[test]
fn game_simulation_runs_100_frames_without_panic() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(GameLogicPlugin);
    // No DefaultPlugins — no window, no renderer, no GPU

    for _ in 0..100 {
        app.update();
    }

    let world = app.world();
    // Assert game state after 100 frames
}
```

**E2E tests run in CI on every merge to `main` and are optional on PRs** (gated behind a label or manual trigger) to keep PR feedback fast.

---

## Coverage Requirements Summary

| Tier | Min Coverage | Target Coverage | Metric |
|------|-------------|----------------|--------|
| Unit | 80% lines | 90% lines | `cargo llvm-cov --lib` |
| Integration | 60% lines | 75% lines | `cargo llvm-cov --tests` |
| E2E | N/A | 100% scenario | Manual checklist against documented workflows |
| New code (per PR) | 80% lines | 90% lines | Diff coverage in CI |

**Per-module expectations:**

| Module Type | Unit Target | Integration Target | Rationale |
|------------|-----------|-------------------|-----------|
| Config parsing/validation | 90% | 80% | Pure logic, highly testable, critical correctness |
| CLI argument handling | 85% | 70% | Mostly declarative (clap), test edge cases |
| Scheduler/cron | 85% | 75% | Core logic with time-dependent behavior |
| Docker/container client | 60% | 80% | Thin wrapper; integration tests are more valuable |
| Metrics/persistence | 80% | 70% | Persistence logic needs real filesystem tests |
| Logger/telemetry | 60% | 60% | Lower priority, mostly delegating to tracing |
| Hooks/event routing | 85% | 75% | Complex matching/routing logic |
| HTTP/API client | 50% | 70% | Thin client; mock at unit level, real at integration |
| ECS components/resources | 85% | N/A | Data types — unit tests for transforms/validation |
| ECS systems (game logic) | 70% | 80% | Systems need real World tests to catch query issues |
| Game state machines | 90% | 75% | State transitions are critical correctness |
| Physics/math utilities | 95% | N/A | Pure functions, exhaustively testable |
| Asset loading/processing | 60% | 75% | Integration with real files is more valuable |
| Rendering/shaders | N/A | N/A | Test via visual regression tooling, not line coverage |
| Plugin composition | N/A | 70% | Integration-only: test plugins work together in real App |

---

## Mocking and Test Doubles

### When to Mock

Mock **only** when the real dependency is one of:
- **Slow** (network calls > 100ms, Docker builds > 5s)
- **Non-deterministic** (time, randomness, external API state)
- **Destructive** (sends real messages, modifies production data, costs money)
- **Unavailable in CI** (requires GPU, special hardware, paid API keys)
- **Requires a window/display** (rendering, input devices, audio)

If none of these apply, **use the real thing**.

### When NOT to Mock

Do not mock:
- The filesystem for config/asset loading — use real temp files via `tempfile` crate
- TOML/JSON/YAML parsing — use real parsers with real input strings
- Cron expression evaluation — use real cron library with controlled timestamps
- Data structures and in-memory state — no reason to mock what you own
- ECS `World` — use a real Bevy `World` instance, they're cheap to create
- Math and physics computations — test with real numbers

### Mock Implementation Pattern

Use trait-based dependency injection. Define traits for external boundaries, implement them for production, and provide test doubles.

```rust
// src/docker/mod.rs

/// Trait defining the Docker client interface.
/// Production code uses `RealDockerClient`.
/// Tests use `MockDockerClient`.
#[async_trait]
pub trait DockerClient: Send + Sync {
    async fn build_image(&self, dockerfile: &Path, tag: &str) -> Result<()>;
    async fn run_container(&self, config: &ContainerConfig) -> Result<ContainerOutput>;
    async fn stop_container(&self, container_id: &str) -> Result<()>;
    async fn get_logs(&self, container_id: &str) -> Result<String>;
}

pub struct RealDockerClient { /* ... */ }

#[async_trait]
impl DockerClient for RealDockerClient {
    async fn build_image(&self, dockerfile: &Path, tag: &str) -> Result<()> {
        // Real Docker API calls
    }
    // ...
}
```

```rust
// tests/common/mocks.rs

pub struct MockDockerClient {
    pub build_result: Result<()>,
    pub run_result: Result<ContainerOutput>,
    pub stop_result: Result<()>,
    pub logs_result: Result<String>,
    pub calls: Arc<Mutex<Vec<MockCall>>>,
}

#[derive(Debug, Clone)]
pub enum MockCall {
    Build { tag: String },
    Run { config: ContainerConfig },
    Stop { container_id: String },
    GetLogs { container_id: String },
}

#[async_trait]
impl DockerClient for MockDockerClient {
    async fn build_image(&self, _dockerfile: &Path, tag: &str) -> Result<()> {
        self.calls.lock().unwrap().push(MockCall::Build { tag: tag.to_string() });
        self.build_result.clone()
    }
    // ...
}
```

### Mock Boundaries by Module

| Dependency | Unit Tests | Integration Tests | E2E Tests |
|-----------|-----------|-------------------|-----------|
| Docker daemon | Mock via trait | Real Docker daemon | Real Docker daemon |
| Discord/HTTP APIs | Mock via trait | Test server (wiremock) | Real API (sandbox) |
| Filesystem | Real (`tempfile`) | Real (`tempfile`) | Real (temp workspace) |
| System clock | Mock via `Clock` trait | Real or `tokio::time::pause` | Real |
| HTTP endpoints | Mock via trait | `wiremock` server | Real endpoints |
| stdin/stdout | Captured buffers | Captured via subprocess | Captured via subprocess |
| Environment vars | `temp_env` or manual | `temp_env` | Set in subprocess env |
| GPU/Renderer | Not tested | Not tested | Headless mode or skip |
| Window/Input | Mock via trait or events | Synthetic events into ECS | Headless + synthetic input |
| Audio | Mock via trait | Skip | Skip |
| Random number gen | Seeded RNG | Seeded RNG | Seeded RNG |

### Time Mocking

Time-dependent code (scheduler, cron evaluation, timeouts, game tick timing) must accept an injectable clock:

```rust
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Tz>;
}

pub struct SystemClock;
impl Clock for SystemClock {
    fn now(&self) -> DateTime<Tz> {
        Utc::now()
    }
}

pub struct FixedClock(pub DateTime<Tz>);
impl Clock for FixedClock {
    fn now(&self) -> DateTime<Tz> {
        self.0
    }
}
```

For Bevy projects, game time is controlled through `Time` resource manipulation:

```rust
#[test]
fn cooldown_system_decrements_each_tick() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_systems(Update, cooldown_system);

    let entity = app.world_mut().spawn(Cooldown { remaining: 2.0 }).id();

    // Bevy's MinimalPlugins includes TimePlugin — advance with update()
    app.update();
    app.update();

    let cooldown = app.world().get::<Cooldown>(entity).unwrap();
    assert!(cooldown.remaining < 2.0);
}
```

### Randomness Control

Any system using randomness must accept a seedable RNG for deterministic testing:

```rust
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub struct GameRng(pub ChaCha8Rng);

impl Default for GameRng {
    fn default() -> Self {
        Self(ChaCha8Rng::from_entropy())
    }
}

impl GameRng {
    pub fn seeded(seed: u64) -> Self {
        Self(ChaCha8Rng::seed_from_u64(seed))
    }
}

// In tests:
app.insert_resource(GameRng::seeded(42));
```

### HTTP Mocking with wiremock

For integration tests against HTTP APIs (Discord, webhooks, REST services):

```rust
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, body_json};

#[tokio::test]
async fn discord_send_message_posts_to_channel() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v10/channels/123456/messages"))
        .and(body_json(serde_json::json!({
            "content": "Hello from test"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!({"id": "msg_1"})
        ))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = DiscordClient::new(&mock_server.uri(), "fake-token");
    client.send_message("123456", "Hello from test").await.unwrap();
}
```

---

## Project-Type-Specific Guidance

### CLI Tools (Gastown, discli)

**Key concerns:** Argument parsing correctness, config validation, subprocess management, output formatting, signal handling.

**Testing approach:**
- Unit test all parsing and validation logic exhaustively
- Integration test CLI commands with `std::process::Command` or `assert_cmd`
- E2E test documented workflows as subprocess invocations
- Snapshot test formatted output (tables, help text, error messages) with `insta`

```rust
#[test]
fn list_command_output_matches_snapshot() {
    let output = Command::new(env!("CARGO_BIN_EXE_gastown"))
        .args(["list", "--config", "tests/fixtures/configs/multiple_agents.toml"])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&output.stdout);
    insta::assert_snapshot!(stdout);
}
```

### Bevy / Game Projects

**Key concerns:** System correctness, state machine transitions, deterministic simulation, component data integrity, plugin compatibility.

**Testing approach:**
- Unit test all pure computation (math, physics formulas, damage calculation, pathfinding)
- Integration test systems against a real `World` — this is where Bevy bugs actually live (wrong queries, missing components, system ordering)
- E2E test headless simulation runs for N frames, asserting game state
- Never unit test rendering — use visual regression tools (screenshot comparison) as a separate pipeline

**Headless testing pattern:**

```rust
/// Helper to build a minimal testable Bevy app with game logic only.
/// No window, no renderer, no GPU required.
fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    // Add only the plugins under test — not DefaultPlugins
    app
}

#[test]
fn health_system_removes_entity_at_zero_hp() {
    let mut app = test_app();
    app.add_systems(Update, health_system);

    let entity = app.world_mut().spawn((Health(0), Enemy)).id();
    app.update();

    assert!(app.world().get_entity(entity).is_err());
}

#[test]
fn score_resource_increments_on_enemy_death() {
    let mut app = test_app();
    app.insert_resource(Score(0));
    app.add_systems(Update, (health_system, scoring_system).chain());

    app.world_mut().spawn((Health(0), Enemy));
    app.update();

    assert_eq!(app.world().resource::<Score>().0, 1);
}
```

**System ordering tests:**

```rust
#[test]
fn damage_applies_before_death_check() {
    let mut app = test_app();
    app.add_systems(Update, (
        apply_damage_system,
        check_death_system,
    ).chain());

    let entity = app.world_mut().spawn((Health(1), PendingDamage(5))).id();
    app.update();

    // Entity should be dead — damage applied first, then death check removed it
    assert!(app.world().get_entity(entity).is_err());
}
```

**Event testing:**

```rust
#[test]
fn collision_event_triggers_damage() {
    let mut app = test_app();
    app.add_event::<CollisionEvent>();
    app.add_systems(Update, collision_damage_system);

    let target = app.world_mut().spawn(Health(100)).id();
    app.world_mut().send_event(CollisionEvent {
        target,
        damage: 25,
    });
    app.update();

    assert_eq!(app.world().get::<Health>(target).unwrap().0, 75);
}
```

### Libraries / Crates

**Key concerns:** Public API contract stability, backward compatibility, documentation accuracy, edge case handling.

**Testing approach:**
- Unit test every public function and method
- Doc tests for all public API examples (these compile and run with `cargo test`)
- Property-based tests for parsers and transformations
- No integration or E2E tests unless the library wraps external services

```rust
/// Parses a duration string like "5m", "1h", "30s".
///
/// # Examples
///
/// ```
/// use mylib::parse_duration;
///
/// let d = parse_duration("5m").unwrap();
/// assert_eq!(d.as_secs(), 300);
/// ```
pub fn parse_duration(s: &str) -> Result<Duration> { ... }
```

### Web Services / APIs

**Key concerns:** Request routing, middleware behavior, database interactions, authentication, response formatting.

**Testing approach:**
- Unit test request handlers with mock services
- Integration test against real database (SQLite in-memory or testcontainers)
- E2E test with real HTTP requests to a spawned server instance
- Test error responses as carefully as success responses

---

## Test Infrastructure

### Required Crates

```toml
[dev-dependencies]
# Assertions and test utilities
assert_cmd = "2"              # CLI binary testing (CLI projects)
predicates = "3"              # Rich assertion matchers
assert_fs = "1"               # Filesystem assertions

# Temp resources
tempfile = "3"                # Temp files and directories
temp-env = "0.3"              # Scoped environment variable changes

# Async testing
tokio = { version = "1", features = ["test-util", "macros"] }

# HTTP mocking
wiremock = "0.6"              # HTTP mock server

# Snapshot testing
insta = "1"                   # Snapshot testing for output regression

# Property-based testing (recommended for parsers)
proptest = "1"                # Property-based test generation

# Coverage (install as tool, not dev-dep)
# cargo install cargo-llvm-cov
```

**Bevy-specific:**
```toml
[dev-dependencies]
bevy = { version = "0.15", default-features = false, features = ["bevy_state"] }
# Use MinimalPlugins for headless testing — no window, no renderer
```

### Test Fixtures

Store reusable test data in `tests/fixtures/`:

```
tests/fixtures/
├── configs/
│   ├── minimal.toml
│   ├── full.toml
│   ├── invalid_cron.toml
│   └── duplicate_names.toml
├── prompts/
│   ├── simple.md
│   └── complex.md
├── hooks/
│   ├── valid_hooks.yaml
│   └── invalid_hooks.yaml
├── assets/                    # For game projects
│   ├── test_level.json
│   └── test_tilemap.png
└── expected_output/
    ├── list_output.txt
    └── metrics_output.txt
```

Every fixture file must have a comment at the top explaining what it tests and why it exists.

### Shared Test Utilities

```rust
// tests/common/mod.rs

use tempfile::TempDir;
use std::path::PathBuf;

/// Creates a temporary workspace with a config file.
/// Returns (temp_dir, config_path). temp_dir must be kept alive
/// for the duration of the test (dropping it deletes the directory).
pub fn setup_workspace(config_content: &str) -> (TempDir, PathBuf) {
    let dir = TempDir::new().expect("failed to create temp dir");
    let config_path = dir.path().join("gastown.toml");
    std::fs::write(&config_path, config_content).expect("failed to write config");
    (dir, config_path)
}

/// Creates a temporary workspace with config and prompt files.
pub fn setup_workspace_with_prompts(
    config_content: &str,
    prompts: &[(&str, &str)],
) -> (TempDir, PathBuf) {
    let (dir, config_path) = setup_workspace(config_content);
    let prompts_dir = dir.path().join("prompts");
    std::fs::create_dir_all(&prompts_dir).expect("failed to create prompts dir");
    for (name, content) in prompts {
        std::fs::write(prompts_dir.join(name), content).expect("failed to write prompt");
    }
    (dir, config_path)
}
```

For Bevy projects:

```rust
// tests/common/mod.rs

use bevy::prelude::*;

/// Creates a minimal Bevy App suitable for headless testing.
/// Includes MinimalPlugins (time, scheduling) but no rendering.
pub fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app
}

/// Creates a test app with the game logic plugin but no rendering.
pub fn test_app_with_game_logic() -> App {
    let mut app = test_app();
    app.add_plugins(GameLogicPlugin);
    app.insert_resource(GameRng::seeded(42));
    app
}

/// Runs the app for N frames.
pub fn run_frames(app: &mut App, n: usize) {
    for _ in 0..n {
        app.update();
    }
}
```

---

## Error Path Testing

Error paths are where production bugs hide. Every function that returns `Result<T, E>` must have tests for its failure modes.

### Requirements

1. **Every `Result`-returning public function** must have at least one test that triggers each documented error variant.
2. **Error messages must be tested** — assert on the message content, not just that an error occurred. Users see these messages.
3. **Error propagation chains** must be tested — if module A calls module B and B fails, verify A surfaces the error with appropriate context.

```rust
#[test]
fn config_with_empty_agent_name_returns_descriptive_error() {
    let result = parse_config(r#"
        version = "0.1.0"
        [[agent]]
        name = ""
        schedule = "* * * * *"
        prompt = "test"
    "#);

    let err = result.unwrap_err();
    assert!(
        err.to_string().contains("agent name cannot be empty"),
        "Expected descriptive error about empty name, got: {err}"
    );
}
```

### Graceful Degradation

Test that the system degrades gracefully when dependencies are unavailable:

- Corrupted state files → system continues, state resets
- External service unreachable → clear error message, non-zero exit or retry
- Invalid input files → validation catches it before execution
- Rate limited by external API → retry with backoff or clear message
- Malformed config → refuses to start with diagnostic

For game projects:
- Missing asset files → fallback or clear error, not a panic
- Invalid save data → reject load with user-facing message
- Component not found on entity → handle gracefully in system, don't `unwrap()`

---

## Async Testing

All async code must be tested using `tokio::test`:

```rust
#[tokio::test]
async fn scheduler_fires_agent_at_scheduled_time() {
    tokio::time::pause();

    let clock = FixedClock(/* 1 minute before scheduled time */);
    let scheduler = Scheduler::new(config, mock_docker, clock);

    tokio::time::advance(Duration::from_secs(60)).await;

    // Assert agent was triggered
}
```

**Rules for async tests:**
- Always use `#[tokio::test]`, never block on futures with `.block_on()` inside async context
- Use `tokio::time::pause()` for time-dependent tests
- Set explicit timeouts on all async tests to prevent hangs
- Test cancellation behavior — what happens when a future is dropped mid-execution?

---

## CI Pipeline Requirements

### Fast Feedback (every PR)

```yaml
- cargo fmt --check              # Formatting
- cargo clippy -- -D warnings    # Linting
- cargo test --lib               # Unit tests only (~30s)
- cargo llvm-cov --lib           # Unit coverage check
```

### Full Validation (merge to main)

```yaml
- cargo test --lib                          # Unit tests
- cargo test --features integration         # Integration tests
- cargo test --test e2e_*                   # E2E tests
- cargo llvm-cov --all-features --workspace # Full coverage report
```

### Coverage Gates

- **PR coverage diff** must be ≥ 80% on changed lines. PRs that reduce overall coverage are flagged for review.
- **Main branch** coverage must not drop below 70% overall.
- **Coverage report** is uploaded to Codecov on every push to `main`.

---

## Test Hygiene Rules

1. **No `#[ignore]` without a linked issue.** Ignored tests are invisible failures. If a test can't run, file an issue and reference it: `#[ignore = "blocked by #42"]`.

2. **No `unwrap()` in test setup.** Use `.expect("descriptive message")` so failures in setup are diagnosable.

3. **No sleeping in tests.** Use `tokio::time::pause()` + `advance()` for async, inject a `Clock` for sync code, or use `app.update()` for Bevy frame advancement. `thread::sleep` makes tests slow and flaky.

4. **No shared mutable state between tests.** Each test gets its own temp directory, its own mock instances, its own config, its own `World`. Parallel test execution must be safe by default.

5. **Clean up after yourself.** Use `TempDir` (auto-cleanup on drop), not manual file creation. If a test creates Docker containers, it must stop and remove them in a drop guard or cleanup block.

6. **Test names are documentation.** A developer reading test names should understand the module's contract without reading the test body. Use the `<function>_<scenario>_<outcome>` pattern.

7. **One assertion per test (conceptually).** A test can have multiple `assert!` calls if they're verifying different aspects of the same behavior. Don't test unrelated behaviors in one test.

8. **Snapshot tests for formatted output.** Use `insta` for testing CLI output, serialized state, or any structured text. Snapshots catch unintentional regressions and are easy to update intentionally.

9. **Property-based testing for parsers.** Config parsing, expression parsing, and format parsing benefit from `proptest`. These find edge cases humans miss.

10. **Tests must pass on a clean checkout.** No reliance on local state, installed tools (beyond Docker and Rust), or developer-specific configuration.

11. **Seed all randomness.** Any test involving random behavior must use a seeded RNG. Record the seed in test output so failures are reproducible.

12. **Feature-gate expensive tests.** Tests requiring Docker, network, GPU, or other external infra must be behind feature flags. `cargo test` with no flags must always pass on a bare machine with only Rust installed.