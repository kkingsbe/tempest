# TODO2 - Agent 2

> Sprint: 17
> Focus Area: Cache Verification + Design Debt
> Last Updated: 2026-02-24

## Tasks

- [ ] Verify cache management UI
  - 📚 SKILLS: `./skills/iced-rs/SKILL.md`
  - Scope: Verify CacheManager functionality in tempest-app
  - See BACKLOG.md for full context on this Sprint 11 carryover item.
  - Fix estimate: S

- [ ] [DD-018] Fix Timeline component — zero spacing values
  - 📚 SKILLS: `./skills/iced-rs/SKILL.md`
  - Component: `tempest-app/src/timeline.rs`
  - Scope: Replace spacing(0) with spacing(XXS) or spacing(XS), replace padding(0) with padding(XS)
  - Evidence:
    ```rust
    let mut ticks_content = row![].spacing(0).align_y(iced::Alignment::End);  // Line 415
    let(tick_with_label tick_button = button)
        .on_press(TimelineMessage::TimelineClicked(tick_position))
        .padding(0);  // Line 475
    ```
  - See DESIGN_DEBT.md DD-018 for full context.
  - Fix estimate: S

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_2' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
