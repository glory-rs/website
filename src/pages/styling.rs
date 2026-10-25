use crate::{Route, components::*};
use glory::prelude::*;
use sapid::Theme;

const TYPED_STYLE: &str = r#"use glory::prelude::*;

fn status_panel() -> impl Widget {
    let style = Style::new()
        .display(Display::Flex)
        .flex_direction(FlexDirection::Column)
        .width(Length::Percent(100.0))
        .max_width(Limit::Px(640.0))
        .gap(8.0)
        .padding(Edges::all(16.0))
        .radius(8.0);

    view()
        .class("status-panel")
        .style(style)
        .fill(text().content("Ready"))
}"#;

const REPLACEMENT: &str = r#"use glory::prelude::*;

fn panel(active: bool) -> impl Widget {
    // Compute each final value once instead of trying to append declarations.
    let classes = if active { "panel is-active" } else { "panel" };
    let style = Style::new()
        .padding(Edges::all(16.0))
        .gap(if active { 8.0 } else { 12.0 });

    view().class(classes).style(style)
}

fn replacement_pitfall() -> impl Widget {
    view()
        .class("panel")
        .class("panel is-active") // Final classes: panel and is-active.
        .style(Style::new().padding(Edges::all(16.0)))
        .style(Style::new().gap(8.0)) // The padding declaration is gone.
}"#;

const EXTERNAL_CSS: &str = r#".profile {
    display: flex;
    gap: 12px;
    padding: 16px;
    border: 1px solid #ccd4e0;
}

.profile > .profile-title {
    font: 600 1.25rem/1.4 system-ui, sans-serif;
}"#;

const EXTERNAL_WIDGET: &str = r#"use glory::prelude::*;

fn profile() -> impl Widget {
    view().class("profile").fill(
        text()
            .class("profile-title")
            .content("Ada Lovelace"),
    )
}

// With the ssr feature, attach the external document stylesheet.
let document = glory::ssr::Document {
    title: "Profile".into(),
    stylesheets: vec!["/assets/app.css".into()],
    ..Default::default()
};
let html = glory::ssr::render_document(profile(), &document)?;"#;

const SHEET: &str = r#"use glory::prelude::*;
use glory::style::{Profile, Sheet};

fn render_panel() -> std::result::Result<String, Box<dyn std::error::Error>> {
    let sheet = Sheet::parse(
        ".panel { display:flex; gap:12px; padding:16px } \
         .panel > .title:hover { color:#123456; max-width:none }",
    )?;
    sheet.check(Profile::HTML)?;

    let document = glory::ssr::Document {
        title: "Bounded sheet".into(),
        sheets: vec![sheet],
        ..Default::default()
    };
    let widget = view()
        .class("panel")
        .fill(text().class("title").content("Portable rules"));
    Ok(glory::ssr::render_document(widget, &document)?)
}"#;

const LIMITS: &str = r#"use glory::prelude::*;

let absent = Style::new();
let uncapped = Style::new().max_width(Limit::None);
let capped = Style::new().max_width(Limit::Percent(80.0));

assert_eq!(absent.max_width, None); // No inline declaration; CSS can set a cap.
assert_eq!(uncapped.max_width, Some(Limit::None)); // Explicit max-width: none.
assert_eq!(capped.max_width, Some(Limit::Percent(80.0)));

// Preferred and minimum sizes use Length, including Length::Auto.
let automatic = Style::new().width(Length::Auto);
assert_eq!(automatic.width, Some(Length::Auto));"#;

const PROFILE: &str = r#"use glory::prelude::*;

fn accepted_by_host(app: &glory::App, style: &Style) -> bool {
    style.validate().is_ok()
        && app.style_profile().inline.check(style).is_ok()
}

// App::memory() records typed declarations with Profile::RECORD, but does no layout.
// HTML hosts use Profile::HTML. Blitz exposes its current native profile as
// glory::blitz::STYLE_PROFILE when the blitz feature is enabled."#;

pub fn styling() -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", "STYLE & PRESENTATION")),
        Box::new(heading(1, "docs-title", "Style without losing the boundaries.")),
        Box::new(copy(
            "docs-lead",
            "Choose typed Style values for portable declarations, class names for document CSS, or a validated Sheet for a bounded portable subset. The rendering host still owns CSS cascade, fonts, and layout.",
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "Replacement, not accumulation"),
            copy("", "style(...) replaces the complete typed Style and class(...) replaces the complete whitespace-separated class list. Repeated calls do not append or merge. Compute the final value first, including any layout declarations supplied by row() or col()."),
        ))),
        Box::new(heading(2, "docs-heading", "Use typed Style for semantic declarations")),
        Box::new(copy(
            "docs-paragraph",
            "Style has explicit Rust types for dimensions, flex layout, overflow, color, typography, borders, and other supported properties. Absent fields emit no inline declaration; explicit default values remain meaningful.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "typed.rs", TYPED_STYLE, "docs-code")),
        Box::new(code_panel(Theme::blue_dark(), "replacement.rs", REPLACEMENT, "docs-code")),
        Box::new(heading(2, "docs-heading", "Use classes with external CSS")),
        Box::new(copy(
            "docs-paragraph",
            "Classes connect semantic nodes to the surrounding document's CSS. External CSS is not parsed as a typed Style and can use the host engine's wider CSS syntax. Serve it from the browser document, list it in an SSR Document, or install it through the native host; Glory does not infer a shared stylesheet from a class name.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "app.css", EXTERNAL_CSS, "docs-code")),
        Box::new(code_panel(Theme::blue_dark(), "profile.rs", EXTERNAL_WIDGET, "docs-code")),
        Box::new(heading(2, "docs-heading", "Parse a bounded Sheet")),
        Box::new(copy(
            "docs-paragraph",
            "Enable Glory's css feature to parse an immutable Sheet. It accepts the current typed properties, class-based child/descendant selectors, and hover/focus/enabled/disabled/checked states. At-rules, id/type selectors, unknown declarations, unsupported values, and inputs beyond its budgets are rejected. It is not a complete CSS parser.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "sheet.rs", SHEET, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "The example uses ssr + css: render_document checks and lowers every rule for Profile::HTML. For manual integration, glory::html::sheet(&sheet, profile) produces checked HTML-family CSS; a Blitz renderer with CSS support uses set_sheet(&sheet). The target profile is checked again before installation.",
        )),
        Box::new(heading(2, "docs-heading", "Distinguish absent sizes from unlimited sizes")),
        Box::new(copy(
            "docs-paragraph",
            "Maximum sizes use Limit, not Length. Option::None means the field is absent and leaves the stylesheet in charge; Some(Limit::None) explicitly emits an unbounded maximum and removes a stylesheet cap. Length::Auto belongs to preferred and minimum sizes, not max_width or max_height.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "limits.rs", LIMITS, "docs-code")),
        Box::new(heading(2, "docs-heading", "Check the selected host profile")),
        Box::new(copy(
            "docs-paragraph",
            "Each renderer exposes an immutable style profile for the App lifetime. Mounting and typed style updates validate values and reject unsupported properties or units before committing them. Profile::RECORD accepts declarations without performing layout; HTML and Blitz have different real engine boundaries.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "profiles.rs", PROFILE, "docs-code")),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "Portable values are not identical pixels"),
            copy("", "A declaration accepted by two hosts can still resolve through different fonts, text shaping, viewport rules, and CSS engines. Test important layouts on every target instead of expecting pixel-identical output."),
        ))),
        Box::new(doc_nav(Some(Route::CustomWidgets), Some(Route::Web))),
    ];
    docs_layout(Route::Styling, view().fill(content))
}
