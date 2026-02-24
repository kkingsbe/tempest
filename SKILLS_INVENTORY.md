# Skills Inventory Summary

This document maps each skill to what it governs and which task types it applies to. This is used by the Architect to annotate tasks with appropriate skills before distributing them to agents.

---

## Skill Inventory

| Skill | What it governs | Which task types it applies to |
|-------|-----------------|--------------------------------|
| [`DISCLI.md`](skills/DISCLI.md) | Discord CLI communication - sending text messages, image attachments, and notifications to Discord channels via the `discli` CLI tool | Bot notifications, status updates, progress reporting to Discord |
| [`coding-guidelines/SKILL.md`](skills/coding-guidelines/SKILL.md) | Rust code style and naming conventions - 50 core rules covering naming (no `get_` prefix, iterator conventions), data types, strings, formatting | Code style enforcement, code reviews, formatting, naming conventions |
| [`frontend-design/SKILL.md`](skills/frontend-design/SKILL.md) | Creating distinctive, production-grade frontend interfaces - web components, pages, applications with bold aesthetic direction | Building web components, websites, landing pages, dashboards, React components, HTML/CSS layouts, UI styling |
| [`iced-rs/SKILL.md`](skills/iced-rs/SKILL.md) | Building Rust desktop applications with the Iced GUI framework - design system with 8-point spacing scale, professional UI patterns | Desktop UI development, Iced-based applications, GUI components |
| [`rust-best-practices/SKILL.md`](skills/rust-best-practices/SKILL.md) | Idiomatic Rust code based on Apollo GraphQL's best practices - borrowing vs cloning, error handling with Result types, performance optimization | Writing new Rust code, reviewing/refactoring existing code, ownership patterns, error handling, performance tuning |
| [`rust-engineer/SKILL.md`](skills/rust-engineer/SKILL.md) | Senior Rust engineer patterns - systems programming, memory safety, zero-cost abstractions, ownership model, async/await with tokio | Systems-level Rust applications, ownership/borrowing implementations, trait hierarchies, async programming, performance-critical code |
| [`test-driven-development/SKILL.md`](skills/test-driven-development/SKILL.md) | Test-driven development methodology - write tests first, watch them fail, then write minimal code to pass | New features, bug fixes, refactoring, behavior changes (always apply except throwaway prototypes) |

---

## Task Type to Skill Mapping

### Development Tasks
- **New Rust feature implementation** → `rust-best-practices`, `rust-engineer`, `test-driven-development`, `coding-guidelines`
- **Bug fix in Rust code** → `test-driven-development`, `rust-best-practices`, `coding-guidelines`
- **Refactoring Rust code** → `rust-best-practices`, `coding-guidelines`
- **Desktop UI (Iced)** → `iced-rs`, `rust-best-practices`, `test-driven-development`
- **Web frontend** → `frontend-design`

### Communication Tasks
- **Discord notifications/bot messages** → `DISCLI`

### Quality Assurance
- **Code review** → `coding-guidelines`, `rust-best-practices`
- **Test implementation** → `test-driven-development`

---

## Skill Dependencies

- `test-driven-development` should be applied to **all** implementation tasks (features, bug fixes, refactoring)
- `rust-best-practices` and `coding-guidelines` are complementary - use both for Rust code quality
- `rust-engineer` is for more advanced/systems-level Rust work, while `rust-best-practices` covers general idiomatic code
- `iced-rs` is specific to the Iced GUI framework; use alongside `rust-best-practices` for Rust desktop apps
- `frontend-design` is for web technologies (HTML/CSS/JS, React, Vue) - separate from Rust desktop UI work
