# Iced GUI Development Skill

> Comprehensive patterns and best practices for building Rust desktop applications with the Iced GUI framework.

## Version Compatibility

This skill targets **Iced 0.13.x and 0.14.x** (the current function-based API). The older trait-based `Sandbox`/`Application` API (≤0.12) is **deprecated** — do NOT use it.

**Always check the project's `Cargo.toml` for the actual iced version before writing code.** If the version is `0.12` or earlier, note that the API is significantly different and this skill does not apply directly.

---

## Design System: Making Iced UIs Look Professional

This section defines the visual design rules that MUST be followed when building any Iced UI. These are not suggestions — they are hard requirements for producing polished, professional output. AI agents consistently produce cramped, ugly UIs without explicit design constraints. Follow every rule in this section.

### The 8-Point Spacing Scale

ALL spacing values in your application MUST come from this scale. Do NOT use arbitrary pixel values.

| Token | Value | Usage |
|-------|-------|-------|
| `XXS` | 2 | Icon-to-label gaps inside compact elements |
| `XS` | 4 | Tightest meaningful spacing, inner checkbox/radio gaps |
| `SM` | 8 | Spacing between closely related items (label and its field, icon and text) |
| `MD` | 12 | Spacing between sibling elements within a group (list items, form fields) |
| `BASE` | 16 | Default spacing between UI groups, standard padding |
| `LG` | 24 | Spacing between distinct sections of a view |
| `XL` | 32 | Major section separators, panel padding |
| `XXL` | 48 | Page-level margins, hero section breathing room |
| `XXXL` | 64 | Maximum spacing, used for dramatic separation |

**Implement as constants:**

```rust
mod spacing {
    pub const XXS: u16 = 2;
    pub const XS: u16 = 4;
    pub const SM: u16 = 8;
    pub const MD: u16 = 12;
    pub const BASE: u16 = 16;
    pub const LG: u16 = 24;
    pub const XL: u16 = 32;
    pub const XXL: u16 = 48;
    pub const XXXL: u16 = 64;
}
```

### The Golden Rule: Let Things Breathe

The single most common AI-generated UI failure is **cramped layouts with insufficient whitespace**. When in doubt, use MORE space, not less.

**Mandatory minimums:**
- **Window/page padding:** At least `XL` (32px) on all sides. Never less than `LG` (24px).
- **Section spacing:** At least `LG` (24px) between distinct content groups.
- **Element spacing within a group:** At least `SM` (8px), typically `MD` (12px).
- **Button padding:** At least 12px vertical, 24px horizontal. Buttons must never feel cramped.
- **Form field spacing:** At least `MD` (12px) between consecutive inputs.
- **Container internal padding:** At least `BASE` (16px). Cards and panels use `LG` (24px) or `XL` (32px).

**The proximity rule:** Space BETWEEN groups must always be LARGER than space WITHIN groups. This is how humans perceive visual grouping (Gestalt principle of proximity). If items inside a card are spaced at 8px, the space between cards must be at least 16px.

```rust
// ❌ WRONG — Everything jammed together, no visual hierarchy
fn view(state: &State) -> Element<'_, Message> {
    column![
        text("Settings"),
        text_input("Name", &state.name).on_input(Message::Name),
        text_input("Email", &state.email).on_input(Message::Email),
        button("Save").on_press(Message::Save),
    ]
    .into()
}

// ✅ CORRECT — Proper spacing, breathing room, visual hierarchy
fn view(state: &State) -> Element<'_, Message> {
    container(
        column![
            text("Settings").size(24),

            // Form group — internal spacing SM/MD
            column![
                column![
                    text("Name").size(14),
                    text_input("Enter your name", &state.name)
                        .on_input(Message::Name)
                        .padding(10),
                ].spacing(spacing::XS),

                column![
                    text("Email").size(14),
                    text_input("Enter your email", &state.email)
                        .on_input(Message::Email)
                        .padding(10),
                ].spacing(spacing::XS),
            ]
            .spacing(spacing::MD),

            // Action area — separated from form by larger gap
            row![
                Space::with_width(Fill),
                button("Cancel").on_press(Message::Cancel)
                    .padding([8, 20]),
                button("Save").on_press(Message::Save)
                    .style(button::primary)
                    .padding([8, 24]),
            ]
            .spacing(spacing::SM),
        ]
        .spacing(spacing::LG)   // Between major sections (title, form, actions)
    )
    .padding(spacing::XL)       // Page-level breathing room
    .into()
}
```

### Typography Scale

Use a consistent type scale. Do NOT pick arbitrary font sizes.

| Role | Size (px) | Usage |
|------|-----------|-------|
| Display / Hero | 36-48 | Landing screens, splash, large counters |
| Page Title (H1) | 28-32 | Main screen title |
| Section Title (H2) | 22-24 | Section headings |
| Subsection (H3) | 18-20 | Card titles, group labels |
| Body Large | 16-18 | Primary content, important descriptions |
| Body / Default | 14-16 | Standard text (iced default is ~16) |
| Caption / Small | 12-13 | Secondary info, timestamps, help text |
| Micro | 10-11 | Badges, status labels, legal fine print |

**Rules:**
- NEVER use more than 3-4 different text sizes in a single view. Restraint creates hierarchy.
- Titles should be visually distinct from body text — use both size AND weight (or color) to differentiate.
- Use `.size()` consistently. Define size constants alongside your spacing constants.
- Pair size changes with color intensity: titles in full-opacity text, captions in muted/secondary color.

```rust
mod font_size {
    pub const DISPLAY: f32 = 40.0;
    pub const H1: f32 = 28.0;
    pub const H2: f32 = 22.0;
    pub const H3: f32 = 18.0;
    pub const BODY: f32 = 15.0;
    pub const CAPTION: f32 = 12.0;
}
```

### Color System

**Never use raw hex/RGB values scattered throughout your view code.** Define a semantic color palette and use it everywhere.

```rust
use iced::Color;

mod colors {
    use iced::Color;

    // Surface colors (backgrounds)
    pub const BG_PRIMARY: Color = Color::from_rgb(0.11, 0.11, 0.14);
    pub const BG_SECONDARY: Color = Color::from_rgb(0.15, 0.15, 0.19);
    pub const BG_ELEVATED: Color = Color::from_rgb(0.18, 0.18, 0.23);

    // Text colors
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.93, 0.93, 0.95);
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.6, 0.6, 0.65);
    pub const TEXT_MUTED: Color = Color::from_rgb(0.4, 0.4, 0.45);

    // Accent / interactive
    pub const ACCENT: Color = Color::from_rgb(0.35, 0.55, 1.0);
    pub const ACCENT_HOVER: Color = Color::from_rgb(0.45, 0.65, 1.0);

    // Semantic
    pub const SUCCESS: Color = Color::from_rgb(0.3, 0.8, 0.4);
    pub const WARNING: Color = Color::from_rgb(1.0, 0.75, 0.2);
    pub const DANGER: Color = Color::from_rgb(1.0, 0.35, 0.35);
}
```

**OR better: leverage the built-in Theme palette:**

```rust
fn styled_container(theme: &Theme) -> container::Style {
    let p = theme.extended_palette();
    container::Style {
        background: Some(p.background.weak.color.into()),
        border: Border::default().rounded(8).width(1).color(p.background.strong.color),
        ..Default::default()
    }
}
```

**Color rules:**
- Use the theme's `extended_palette()` whenever possible — this ensures your app respects the user's chosen theme.
- Semantic color naming (`danger`, `success`, `accent`) rather than literal naming (`red`, `green`, `blue`).
- Text on colored backgrounds must have sufficient contrast. Light text on dark backgrounds, dark text on light backgrounds.
- Never use pure black (`#000`) on pure white (`#FFF`) — it's too harsh. Soften both slightly.
- Use opacity/alpha to create hierarchy: primary text at full opacity, secondary at 60-70%, muted at 40%.
- Interactive elements (buttons, links) must have a visually distinct color from static text.
- Hover/active states must be visibly different from the default state (change background, not just cursor).

### Visual Hierarchy Checklist

Every view must answer these questions clearly through visual design alone:

1. **What is this screen?** → A clear, prominent title (H1/H2 size, strong color).
2. **What are the main sections?** → Visual grouping via spacing, containers, or rules.
3. **What can I interact with?** → Buttons styled distinctly from text, inputs have visible borders/backgrounds.
4. **What should I do first?** → Primary action button uses accent color and is visually dominant. Secondary actions are styled more subtly.
5. **What is less important?** → Captions, metadata, and help text are smaller and use muted colors.

### Layout Patterns for Professional UIs

#### Sidebar + Content

```rust
fn view(state: &State) -> Element<'_, Message> {
    row![
        // Sidebar — fixed width, darker background
        container(
            column![
                text("Navigation").size(font_size::H3),
                // nav items...
            ]
            .spacing(spacing::SM)
        )
        .width(240)
        .height(Fill)
        .padding(spacing::LG)
        .style(sidebar_style),

        // Divider
        rule::vertical(1),

        // Main content — fills remaining space
        container(
            scrollable(
                column![
                    text("Dashboard").size(font_size::H1),
                    // content...
                ]
                .spacing(spacing::LG)
            )
            .height(Fill)
        )
        .width(Fill)
        .padding(spacing::XL),
    ]
    .into()
}
```

#### Card Component

```rust
fn card_view(item: &Item) -> Element<'_, Message> {
    container(
        column![
            text(&item.title).size(font_size::H3),
            text(&item.description)
                .size(font_size::CAPTION)
                .style(text::secondary),
            Space::with_height(spacing::SM),
            button("Open")
                .on_press(Message::Open(item.id))
                .style(button::primary)
                .padding([8, 20]),
        ]
        .spacing(spacing::SM)
    )
    .padding(spacing::LG)
    .style(container::rounded_box)
    .width(280)
    .into()
}
```

#### Form Layout with Labels

```rust
fn labeled_field<'a>(
    label: &str,
    field: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    column![
        text(label).size(font_size::CAPTION).style(text::secondary),
        field.into(),
    ]
    .spacing(spacing::XS)
    .into()
}

fn form_view(state: &FormState) -> Element<'_, Message> {
    container(
        column![
            text("Create Account").size(font_size::H2),

            labeled_field("Username",
                text_input("Enter username", &state.username)
                    .on_input(Message::Username)
                    .padding(10)),

            labeled_field("Password",
                text_input("Enter password", &state.password)
                    .on_input(Message::Password)
                    .password()
                    .padding(10)),

            Space::with_height(spacing::SM),

            row![
                Space::with_width(Fill),
                button("Create Account")
                    .on_press_maybe(state.is_valid().then_some(Message::Submit))
                    .style(button::primary)
                    .padding([10, 32]),
            ],
        ]
        .spacing(spacing::MD)
        .max_width(400)
    )
    .center_x(Fill)
    .center_y(Fill)
    .padding(spacing::XL)
    .into()
}
```

### Container Styling Patterns

Cards, panels, and sections should be visually distinct from the background:

```rust
fn card_style(theme: &Theme) -> container::Style {
    let p = theme.extended_palette();
    container::Style {
        background: Some(p.background.weak.color.into()),
        border: Border::default()
            .rounded(12)
            .width(1)
            .color(p.background.strong.color),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..Default::default()
    }
}
```

**Border radius guidelines:**
- Buttons: 6-8px
- Cards/panels: 8-12px
- Modals/dialogs: 12-16px
- Input fields: 4-8px
- Pill/tags: use height/2 for fully rounded

### Button Hierarchy

Every screen should have at most ONE primary action button. Other actions use secondary or text styles.

```rust
// Primary — main action (accent color, filled)
button("Save Changes")
    .on_press(Message::Save)
    .style(button::primary)
    .padding([10, 28]);

// Secondary — alternative action (outlined or muted)
button("Cancel")
    .on_press(Message::Cancel)
    .style(button::secondary)
    .padding([10, 20]);

// Danger — destructive action (red/danger color)
button("Delete")
    .on_press(Message::Delete)
    .style(button::danger)
    .padding([10, 20]);

// Text/ghost — minimal emphasis (no background)
button("Learn more")
    .on_press(Message::Info)
    .style(button::text)
    .padding([6, 12]);
```

**Button padding rule:** Horizontal padding should be at least 2x vertical padding. A button with 10px top/bottom should have at least 20px left/right. This prevents buttons from looking like squares.

### Disabled and Empty States

**Disabled states:** Use `on_press_maybe(None)` — never hide interactive elements. Users should understand what actions exist even when unavailable. Style disabled elements with reduced opacity or muted colors.

**Empty states:** When a list or section has no content, ALWAYS show a helpful empty state, not just a blank area:

```rust
fn list_view(items: &[Item]) -> Element<'_, Message> {
    if items.is_empty() {
        container(
            column![
                text("No items yet").size(font_size::H3).style(text::secondary),
                text("Click the button above to add your first item.")
                    .size(font_size::CAPTION)
                    .style(text::secondary),
            ]
            .spacing(spacing::SM)
            .align_x(Center)
        )
        .center_x(Fill)
        .center_y(Fill)
        .padding(spacing::XXL)
        .into()
    } else {
        scrollable(
            Column::with_children(
                items.iter().map(|item| item_row(item)).collect()
            )
            .spacing(spacing::SM)
        )
        .height(Fill)
        .into()
    }
}
```

### Loading States

Show clear visual feedback during async operations:

```rust
fn view(state: &State) -> Element<'_, Message> {
    if state.loading {
        container(
            column![
                text("Loading...").size(font_size::BODY),
                // Or use a progress_bar if you have progress info:
                // progress_bar(0.0..=100.0, state.progress),
            ]
            .spacing(spacing::SM)
            .align_x(Center)
        )
        .center_x(Fill)
        .center_y(Fill)
        .into()
    } else {
        main_view(state)
    }
}
```

### Design Anti-Patterns — DO NOT DO THESE

- **❌ No padding on the outermost container** — Content should NEVER touch the window edge.
- **❌ Uniform spacing everywhere** — Using the same spacing value for everything destroys hierarchy. Vary spacing to create groups.
- **❌ Tiny text on giant buttons (or vice versa)** — Maintain proportional relationships between text size and container size.
- **❌ More than 2 primary-colored buttons on one screen** — One primary CTA per view. Everything else is secondary or text.
- **❌ Rainbow colors** — Stick to 1 accent color, 1-2 semantic colors (success, danger), and neutral grays. More colors = more chaos.
- **❌ No visual distinction between sections** — Use spacing, background color changes, or horizontal rules to separate sections.
- **❌ Text walls with no hierarchy** — Break up text with headings, spacing, and size variation.
- **❌ Cramped form fields** — Inputs need padding inside AND spacing between them.
- **❌ Invisible clickable elements** — Every button/interactive element must look obviously interactive through color, border, or background.
- **❌ Ignoring alignment** — All elements in a column should share the same left edge. All elements in a row should share the same vertical center. Misalignment looks amateur instantly.

---

## Core Architecture: The Elm Architecture (TEA)

Every Iced application has exactly four parts:

1. **State** — A struct holding all application data
2. **Message** — An enum representing every possible user interaction or event
3. **Update** — A function `fn update(&mut self, message: Message)` that mutates state in response to messages
4. **View** — A function `fn view(&self) -> Element<'_, Message>` that produces the UI from current state

This is non-negotiable. All UI state lives in the State struct. All mutations happen in `update`. The `view` function is **pure** — it reads state and returns widgets, nothing else.

---

## Application Bootstrap (0.13+)

### Minimal Application (no async, no theming)

```rust
use iced::widget::{button, column, text};
use iced::Element;

fn main() -> iced::Result {
    iced::run(update, view)
}

#[derive(Default)]
struct State {
    count: i64,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Increment => state.count += 1,
        Message::Decrement => state.count -= 1,
    }
}

fn view(state: &State) -> Element<'_, Message> {
    column![
        button("+").on_press(Message::Increment),
        text(state.count).size(40),
        button("-").on_press(Message::Decrement),
    ]
    .spacing(10)
    .into()
}
```

**Key points:**
- `iced::run(update, view)` is the simplest entry point. State must implement `Default`.
- The `column![]` and `row![]` macros are syntactic sugar for building layouts.
- Always call `.into()` on the outermost widget to convert to `Element`.

### Full Application (with title, theme, subscriptions, custom init)

```rust
use iced::widget::{button, column, text, Column};
use iced::{Element, Fill, Subscription, Task, Theme};

fn main() -> iced::Result {
    iced::application("My App", update, view)
        .theme(theme)
        .subscription(subscription)
        .window_size((800.0, 600.0))
        .centered()
        .run()
}

struct State {
    // fields...
}

// Custom initialization (replaces Default)
impl Default for State {
    fn default() -> Self {
        Self { /* ... */ }
    }
}

// OR use a boot function for async init:
// iced::application(boot, update, view)
// fn boot() -> (State, Task<Message>) { ... }

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        // return Task::none() when no side effects needed
        // return Task::perform(...) for async operations
        _ => Task::none(),
    }
}

fn view(state: &State) -> Element<'_, Message> {
    // ...
    column![].into()
}

fn theme(state: &State) -> Theme {
    Theme::TokyoNight
}

fn subscription(state: &State) -> Subscription<Message> {
    Subscription::none()
}
```

**Key points:**
- `iced::application(title_or_boot, update, view)` gives full builder access.
- The first argument can be a `&str` title (State must be `Default`) OR a boot function `fn() -> (State, Task<Message>)`.
- `update` can return either `()` or `Task<Message>` — both work.
- Chain `.theme()`, `.subscription()`, `.window_size()`, `.centered()`, `.antialiasing()` etc. before `.run()`.

---

## Widget Reference

### Layout Widgets

| Widget | Purpose | Key Methods |
|--------|---------|-------------|
| `column![]` / `Column` | Vertical stack | `.spacing()`, `.padding()`, `.width()`, `.height()`, `.align_x()` |
| `row![]` / `Row` | Horizontal stack | `.spacing()`, `.padding()`, `.width()`, `.height()`, `.align_y()` |
| `container()` | Single-child wrapper for alignment/styling | `.width()`, `.height()`, `.center_x()`, `.center_y()`, `.padding()`, `.style()` |
| `scrollable()` | Scrollable content area | `.width()`, `.height()`, `.direction()` |
| `stack![]` / `Stack` | Z-axis layering (overlapping) | Children stack on top of each other |
| `space()` | Empty spacer | `Space::with_width()`, `Space::with_height()` |
| `responsive()` | Responsive layout based on available space | Takes closure `\|size\| -> Element` |

### Interactive Widgets

| Widget | Purpose | Key Methods |
|--------|---------|-------------|
| `button()` | Clickable button | `.on_press(Message)`, `.on_press_maybe(Option<Message>)`, `.style()`, `.padding()`, `.width()` |
| `text_input()` | Single-line text field | `.on_input(fn(String) -> Message)`, `.on_submit(Message)`, `.placeholder()`, `.password()`, `.size()` |
| `text_editor()` | Multi-line text editor | `.on_action(fn(Action) -> Message)`, `.highlight()` |
| `checkbox()` | Boolean toggle box | `.on_toggle(fn(bool) -> Message)`, `.size()`, `.spacing()` |
| `toggler()` | Boolean toggle switch | `.on_toggle(fn(bool) -> Message)`, `.size()`, `.spacing()` |
| `radio()` | Radio button (one of many) | `radio(label, value, selected, on_click)` |
| `pick_list()` | Dropdown select | `pick_list(options, selected, on_select)` |
| `combo_box()` | Searchable dropdown | Requires `combo_box::State` in your app state |
| `slider()` | Range slider | `slider(range, value, on_change)` |
| `text()` | Display text | `.size()`, `.color()`, `.font()`, `.style()` |
| `progress_bar()` | Progress indicator | `progress_bar(range, value)` |
| `tooltip()` | Hover tooltip | `tooltip(content, tooltip_text, position)` |
| `canvas()` | Custom 2D drawing | Implement `canvas::Program` trait |
| `image()` | Raster image display | `image(handle)`, `.width()`, `.height()`, `.content_fit()` |
| `svg()` | Vector image display | `svg(handle)`, `.width()`, `.height()` |

### Display Widgets

| Widget | Purpose |
|--------|---------|
| `rule::horizontal()` / `rule::vertical()` | Divider line |
| `rich_text()` | Styled text with spans, links, underline, strikethrough |
| `markdown()` | Render markdown content |
| `pane_grid()` | Resizable split pane layout |

---

## Sizing and Layout

Iced uses `Length` for sizing. The three strategies:

| Strategy | Meaning | Usage |
|----------|---------|-------|
| `Shrink` | Fit to content (default for most widgets) | `.width(Shrink)` or just omit |
| `Fill` | Expand to fill available space | `.width(Fill)` |
| `Fixed` | Exact pixel size | `.width(300)` or `.width(Pixels(300.0))` |

```rust
use iced::{Fill, Shrink, Length};
use iced::widget::{column, container, row, text};

fn view(state: &State) -> Element<'_, Message> {
    container(
        column![
            text("Header").size(24),
            row![
                // Left sidebar: fixed width
                container("Sidebar").width(200),
                // Main content: fills remaining space
                container("Content").width(Fill),
            ]
            .spacing(10)
            .height(Fill),
            text("Footer"),
        ]
        .spacing(10)
    )
    .padding(20)
    .width(Fill)
    .height(Fill)
    .into()
}
```

**Important layout rules:**
- `column![]` and `row![]` use `Shrink` by default but inherit `Fill` from children.
- Use `.spacing(N)` for gaps between children (NOT padding).
- Use `.padding(N)` for internal spacing within a container.
- Use `Space::with_width(Fill)` or `Space::with_height(Fill)` to push elements apart.
- Centering: `container(content).center_x(Fill).center_y(Fill)` or use the convenience `center()` function.
- Alignment within columns: `.align_x(Center)` — within rows: `.align_y(Center)`.

---

## Theming and Styling

### Built-in Themes

Iced ships with many theme variants: `Theme::Light`, `Theme::Dark`, `Theme::TokyoNight`, `Theme::Dracula`, `Theme::Nord`, `Theme::SolarizedLight`, `Theme::SolarizedDark`, `Theme::GruvboxLight`, `Theme::GruvboxDark`, `Theme::CatppuccinLatte`, `Theme::CatppuccinMocha`, `Theme::Kanagawa`, `Theme::Moonfly`, `Theme::Nightfly`, `Theme::Oxocarbon`, `Theme::Ferra`.

### Closure-based Widget Styling (0.13+)

Every widget's `.style()` method takes a closure. The closure receives the `Theme` and (for stateful widgets) a `Status`:

```rust
use iced::widget::{button, container, text};
use iced::{Border, Color, Element, Theme};

fn view(state: &State) -> Element<'_, Message> {
    // Container styling
    container(
        column![
            // Button with custom style per status
            button("Click me")
                .on_press(Message::Click)
                .style(|theme: &Theme, status| {
                    let palette = theme.extended_palette();
                    match status {
                        button::Status::Active => button::Style {
                            background: Some(palette.primary.strong.color.into()),
                            text_color: Color::WHITE,
                            border: Border::default().rounded(8),
                            ..button::Style::default()
                        },
                        button::Status::Hovered => button::Style {
                            background: Some(palette.primary.base.color.into()),
                            text_color: Color::WHITE,
                            border: Border::default().rounded(8),
                            ..button::Style::default()
                        },
                        _ => button::primary(theme, status),
                    }
                }),

            // Text with danger style (convenience function)
            text("Warning!").style(text::danger),
        ]
    )
    // Container convenience styles
    .style(container::rounded_box)
    .padding(20)
    .into()
}
```

**Built-in convenience style functions:**
- `button::primary`, `button::secondary`, `button::success`, `button::danger`, `button::text`
- `container::rounded_box`, `container::bordered_box`
- `text::default`, `text::primary`, `text::secondary`, `text::success`, `text::danger`

### Custom Themes

```rust
use iced::theme::{self, Palette};
use iced::{Color, Theme};

fn custom_theme() -> Theme {
    Theme::custom(
        "My Theme".to_string(),
        Palette {
            background: Color::from_rgb(0.1, 0.1, 0.15),
            text: Color::from_rgb(0.9, 0.9, 0.92),
            primary: Color::from_rgb(0.3, 0.6, 1.0),
            success: Color::from_rgb(0.3, 0.85, 0.4),
            danger: Color::from_rgb(1.0, 0.3, 0.35),
            warning: Color::from_rgb(1.0, 0.8, 0.2),
        },
    )
}
```

### Extracting Palette Colors

```rust
fn my_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border::default()
            .rounded(8)
            .width(1)
            .color(palette.background.strong.color),
        ..container::Style::default()
    }
}
```

The extended palette provides `background`, `primary`, `secondary`, `success`, `danger` — each with `.base`, `.weak`, `.strong` sub-palettes containing `.color` and `.text` fields.

---

## Async Operations with Task

`Task` replaces the old `Command` type. Use it for async operations, navigation, clipboard access, etc.

```rust
fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::FetchData => {
            state.loading = true;
            // Perform an async operation
            Task::perform(
                async {
                    // async work here
                    reqwest::get("https://api.example.com/data")
                        .await?
                        .json::<Vec<Item>>()
                        .await
                },
                |result| match result {
                    Ok(items) => Message::DataLoaded(items),
                    Err(e) => Message::Error(e.to_string()),
                },
            )
        }
        Message::DataLoaded(items) => {
            state.loading = false;
            state.items = items;
            Task::none()
        }
        Message::Error(err) => {
            state.loading = false;
            state.error = Some(err);
            Task::none()
        }
        _ => Task::none(),
    }
}
```

**Common Task patterns:**
- `Task::none()` — no side effects
- `Task::perform(future, map_fn)` — run a future, map result to Message
- `Task::batch([task1, task2])` — run multiple tasks concurrently
- `iced::exit()` — returns a Task that exits the application
- `text_input::focus(id)` — focus a text input
- `clipboard::write(text)` — write to clipboard

---

## Subscriptions

Subscriptions listen to external events over time (timers, keyboard, system events).

```rust
use iced::time::{self, Duration, Instant};
use iced::{event, keyboard, Subscription};

fn subscription(state: &State) -> Subscription<Message> {
    let mut subs = vec![];

    // Timer tick every second (only when running)
    if state.is_running {
        subs.push(
            time::every(Duration::from_secs(1)).map(Message::Tick)
        );
    }

    // Listen to all keyboard events
    subs.push(
        keyboard::on_key_press(|key, modifiers| {
            match (key, modifiers) {
                (keyboard::Key::Named(keyboard::key::Named::Space), _) => {
                    Some(Message::TogglePause)
                }
                _ => None,
            }
        })
    );

    Subscription::batch(subs)
}
```

**Common subscription sources:**
- `time::every(Duration)` — periodic timer
- `keyboard::on_key_press(handler)` — keyboard events
- `event::listen()` — all iced events
- `Subscription::run(stream_fn)` — custom async stream

---

## Component Patterns

### Splitting Large Applications

Decompose views into functions that return `Element`:

```rust
fn view(state: &State) -> Element<'_, Message> {
    column![
        header_view(state),
        content_view(state),
        footer_view(state),
    ]
    .spacing(10)
    .into()
}

fn header_view(state: &State) -> Element<'_, Message> {
    row![
        text(&state.title).size(24),
        Space::with_width(Fill),
        button("Settings").on_press(Message::OpenSettings),
    ]
    .padding(10)
    .align_y(Center)
    .into()
}

fn content_view(state: &State) -> Element<'_, Message> {
    // build content...
    column![].into()
}
```

### Page/Screen Navigation

```rust
#[derive(Default)]
struct State {
    screen: Screen,
    // screen-specific state...
    home: HomeState,
    settings: SettingsState,
}

#[derive(Default)]
enum Screen {
    #[default]
    Home,
    Settings,
    Detail(usize),
}

fn view(state: &State) -> Element<'_, Message> {
    match &state.screen {
        Screen::Home => home_view(&state.home),
        Screen::Settings => settings_view(&state.settings),
        Screen::Detail(id) => detail_view(state, *id),
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Navigate(screen) => {
            state.screen = screen;
            Task::none()
        }
        // delegate to screen-specific update...
        _ => Task::none(),
    }
}
```

### Dynamic Lists

```rust
fn list_view(items: &[Item]) -> Element<'_, Message> {
    let content: Vec<Element<'_, Message>> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            row![
                text(&item.name).width(Fill),
                button("Delete").on_press(Message::DeleteItem(i)),
            ]
            .spacing(10)
            .padding(5)
            .into()
        })
        .collect();

    scrollable(
        Column::with_children(content).spacing(5)
    )
    .height(Fill)
    .into()
}
```

For large lists where items may be reordered, use `keyed_column!` with stable keys to preserve widget state:

```rust
use iced::widget::keyed_column;

fn list_view(items: &[Item]) -> Element<'_, Message> {
    keyed_column(
        items.iter().map(|item| {
            (item.id, row![
                text(&item.name),
                button("X").on_press(Message::Remove(item.id)),
            ].into())
        })
    )
    .spacing(5)
    .into()
}
```

---

## Forms and Input Handling

```rust
#[derive(Default)]
struct FormState {
    name: String,
    email: String,
    role: Option<Role>,
    agree_tos: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Role {
    Admin,
    User,
    Guest,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::User => write!(f, "User"),
            Role::Guest => write!(f, "Guest"),
        }
    }
}

fn form_view(state: &FormState) -> Element<'_, Message> {
    let submit_enabled = !state.name.is_empty()
        && !state.email.is_empty()
        && state.role.is_some()
        && state.agree_tos;

    column![
        text("Registration").size(24),

        text_input("Full name", &state.name)
            .on_input(Message::NameChanged)
            .padding(10),

        text_input("Email", &state.email)
            .on_input(Message::EmailChanged)
            .on_submit(Message::Submit)
            .padding(10),

        pick_list(
            &[Role::Admin, Role::User, Role::Guest][..],
            state.role.as_ref(),
            Message::RoleSelected,
        )
        .placeholder("Select role..."),

        checkbox("I agree to the Terms of Service", state.agree_tos)
            .on_toggle(Message::ToggleTos),

        button("Submit")
            .on_press_maybe(submit_enabled.then_some(Message::Submit))
            .style(button::primary),
    ]
    .spacing(15)
    .padding(20)
    .max_width(400)
    .into()
}
```

**Important:** `on_press_maybe(Option<Message>)` disables the button when `None` — use this instead of conditional rendering for disabled states.

---

## Canvas (Custom 2D Drawing)

```rust
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{mouse, Color, Element, Fill, Point, Rectangle, Renderer, Theme};

struct MyCanvas {
    // drawing state
}

impl canvas::Program<Message> for MyCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // Draw a circle
        let circle = Path::circle(
            Point::new(bounds.width / 2.0, bounds.height / 2.0),
            50.0,
        );
        frame.fill(&circle, Color::from_rgb(0.3, 0.6, 1.0));

        // Draw a line
        let line = Path::line(Point::new(0.0, 0.0), Point::new(100.0, 100.0));
        frame.stroke(
            &line,
            Stroke::default()
                .with_color(Color::WHITE)
                .with_width(2.0),
        );

        vec![frame.into_geometry()]
    }
}

fn view(state: &State) -> Element<'_, Message> {
    Canvas::new(&state.canvas)
        .width(Fill)
        .height(Fill)
        .into()
}
```

Use `canvas::Cache` for expensive drawings that don't change every frame.

---

## Feature Flags

Common Cargo.toml feature flags:

```toml
[dependencies]
iced = { version = "0.13", features = ["image", "svg", "canvas", "tokio", "markdown"] }

# Feature reference:
# "image"    — raster image support (PNG, JPG, etc.)
# "svg"      — SVG rendering
# "canvas"   — 2D drawing canvas
# "tokio"    — use tokio as async executor (needed for reqwest, etc.)
# "markdown" — markdown rendering widget
# "advanced" — expose lower-level APIs for custom widgets
# "debug"    — enable F12 debug overlay
```

**If using `reqwest` or other tokio-dependent crates**, you MUST enable the `"tokio"` feature on iced, otherwise you'll get "no reactor running" panics.

---

## Critical Anti-Patterns — DO NOT DO THESE

### ❌ Using the old trait-based API

```rust
// WRONG — deprecated since 0.13
impl Application for MyApp {
    type Message = Message;
    fn new() -> (Self, Command<Message>) { ... }
    fn update(&mut self, message: Message) -> Command<Message> { ... }
    fn view(&self) -> Element<Message> { ... }
}
```

Use the function-based API: `iced::run(update, view)` or `iced::application(title, update, view)`.

### ❌ Forgetting `#[derive(Clone)]` on Message

```rust
// WRONG — will fail to compile
enum Message {
    Click,
}

// CORRECT
#[derive(Debug, Clone)]
enum Message {
    Click,
}
```

Messages MUST derive `Debug` and `Clone`. If a message contains data, that data must also be `Clone`.

### ❌ Forgetting `#[derive(Default)]` on State when using `iced::run`

```rust
// WRONG with iced::run — State needs Default
struct State { count: i64 }

// CORRECT
#[derive(Default)]
struct State { count: i64 }

// OR use iced::application with a boot function instead
```

### ❌ Forgetting `.into()` on the outermost widget

```rust
// WRONG — type mismatch
fn view(state: &State) -> Element<'_, Message> {
    column![text("hello")]
}

// CORRECT
fn view(state: &State) -> Element<'_, Message> {
    column![text("hello")].into()
}
```

### ❌ Storing widgets in state

```rust
// WRONG — widgets are ephemeral, recreated every frame
struct State {
    my_button: Button<Message>,  // DON'T DO THIS
}

// CORRECT — store data, not widgets
struct State {
    button_label: String,
    is_enabled: bool,
}
```

### ❌ Mutating state outside of `update`

The `view` function receives `&State` (immutable). ALL state changes MUST go through `update` via messages. Do not use `RefCell`, `Cell`, or interior mutability hacks to bypass this.

### ❌ Using `String` as Message variant data when `&str` would seem to work

Messages must be `'static` and owned. Always use `String`, not `&str`, in message enums:

```rust
// WRONG
enum Message {
    TextChanged(&'static str),
}

// CORRECT
#[derive(Debug, Clone)]
enum Message {
    TextChanged(String),
}
```

### ❌ Mixing up `on_press` and `on_input`

- `button().on_press(Message::Click)` — takes a `Message` value directly
- `text_input().on_input(Message::TextChanged)` — takes a `Fn(String) -> Message`

### ❌ Returning a concrete widget type when `Element` is needed

```rust
// WRONG — branches return different types
fn view(state: &State) -> Element<'_, Message> {
    if state.loading {
        text("Loading...") // text widget
    } else {
        column![...] // column widget
    }
    .into() // can't call .into() on mismatched types
}

// CORRECT — convert each branch to Element
fn view(state: &State) -> Element<'_, Message> {
    if state.loading {
        text("Loading...").into()
    } else {
        column![...].into()
    }
}
```

### ❌ Not enabling "tokio" feature when using async libraries

If your `Task::perform` future uses reqwest, sqlx, or any tokio-based library, you MUST add `features = ["tokio"]` to your iced dependency.

---

## Testing Patterns

Iced's TEA architecture makes unit testing straightforward because update logic is pure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_increases_count() {
        let mut state = State::default();
        update(&mut state, Message::Increment);
        update(&mut state, Message::Increment);
        update(&mut state, Message::Decrement);
        assert_eq!(state.count, 1);
    }

    #[test]
    fn form_validation() {
        let mut state = State::default();
        update(&mut state, Message::NameChanged("Alice".into()));
        assert!(!state.name.is_empty());
        assert!(state.is_valid());
    }
}
```

**Testing strategy:**
- Test `update` logic exhaustively — it's just state mutation, no GUI needed.
- Test complex view helper logic (filtering, sorting, formatting) as pure functions.
- Canvas `Program::draw` implementations can be tested by checking the geometry output.
- Integration/visual testing requires running the app — consider screenshot testing for critical UIs.

---

## Cargo.toml Template

```toml
[package]
name = "my-iced-app"
version = "0.1.0"
edition = "2021"

[dependencies]
iced = { version = "0.13", features = ["canvas", "image", "svg", "tokio"] }
# Add as needed:
# reqwest = { version = "0.12", features = ["json"] }
# serde = { version = "1", features = ["derive"] }
# serde_json = "1"
# tokio = { version = "1", features = ["full"] }  # only if using tokio directly

[profile.dev]
opt-level = 1  # iced renders noticeably faster even at opt-level 1

[profile.dev.package."*"]
opt-level = 2  # optimize dependencies in dev for better perf
```

---

## Quick Reference: Common Imports

```rust
// Core
use iced::{Element, Fill, Shrink, Center, Theme, Task, Subscription};

// Widgets
use iced::widget::{
    button, checkbox, column, combo_box, container, horizontal_rule,
    image, keyed_column, pick_list, progress_bar, radio, responsive,
    rich_text, row, rule, scrollable, slider, space, stack, svg,
    text, text_editor, text_input, toggler, tooltip, vertical_rule,
    Canvas, Column, Row, Space,
};

// Styling
use iced::{Border, Color, Padding, Pixels};
use iced::widget::{button, container, text};  // for style functions like button::primary

// Events and subscriptions
use iced::{event, keyboard, mouse, time};
use iced::keyboard::key;

// Canvas drawing
use iced::widget::canvas::{self, Cache, Frame, Geometry, Path, Stroke};

// Async
use iced::Task;

// Window management
use iced::window;
```

---

## Checklist Before Shipping

- [ ] State struct contains ALL mutable data — no global state, no static mut
- [ ] Message enum covers every user interaction — exhaustive match in update
- [ ] All `.into()` calls present on outermost widgets in view functions
- [ ] Message derives `Debug, Clone` (and `Copy` where possible)
- [ ] State derives `Default` if using `iced::run`, or boot function provided with `iced::application`
- [ ] `"tokio"` feature enabled if using async networking libraries
- [ ] `opt-level = 1` in dev profile for reasonable render performance
- [ ] Spacing/padding applied consistently (`.spacing()` between children, `.padding()` within containers)
- [ ] Disabled states use `on_press_maybe(None)` not hidden/removed buttons
- [ ] Large lists use `scrollable()` wrapper and consider `keyed_column` for reorderable items
- [ ] View functions return `Element<'_, Message>` (note the lifetime)