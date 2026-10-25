use crate::Locale;
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

pub fn styling(locale: Locale) -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", locale.text("styling-style-presentation"))),
        Box::new(heading(
            1,
            "docs-title",
            locale.text("styling-style-without-losing-the-boundaries"),
        )),
        Box::new(copy(
            "docs-lead",
            locale.text("styling-choose-typed-style-values-for-portable-declarations-class-names"),
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("styling-replacement-not-accumulation")),
            copy(
                "",
                locale.text("styling-style-replaces-the-complete-typed-style-and-class-replaces"),
            ),
        ))),
        Box::new(heading(
            2,
            "docs-heading",
            locale.text("styling-use-typed-style-for-semantic-declarations"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("styling-style-has-explicit-rust-types-for-dimensions-flex-layout"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "typed.rs", TYPED_STYLE, "docs-code")),
        Box::new(code_panel(
            locale,
            Theme::blue_dark(),
            "replacement.rs",
            REPLACEMENT,
            "docs-code",
        )),
        Box::new(heading(2, "docs-heading", locale.text("styling-use-classes-with-external-css"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("styling-classes-connect-semantic-nodes-to-the-surrounding-document-s"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "app.css", EXTERNAL_CSS, "docs-code")),
        Box::new(code_panel(
            locale,
            Theme::blue_dark(),
            "profile.rs",
            EXTERNAL_WIDGET,
            "docs-code",
        )),
        Box::new(heading(2, "docs-heading", locale.text("styling-parse-a-bounded-sheet"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("styling-enable-glory-s-css-feature-to-parse-an-immutable"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "sheet.rs", SHEET, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            locale.text("styling-the-example-uses-ssr-css-render-document-checks-and"),
        )),
        Box::new(heading(
            2,
            "docs-heading",
            locale.text("styling-distinguish-absent-sizes-from-unlimited-sizes"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("styling-maximum-sizes-use-limit-not-length-option-none-means"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "limits.rs", LIMITS, "docs-code")),
        Box::new(heading(
            2,
            "docs-heading",
            locale.text("styling-check-the-selected-host-profile"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("styling-each-renderer-exposes-an-immutable-style-profile-for-the"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "profiles.rs", PROFILE, "docs-code")),
        Box::new(view().class("docs-callout").fill((
            copy(
                "docs-callout-title",
                locale.text("styling-portable-values-are-not-identical-pixels"),
            ),
            copy("", locale.text("styling-a-declaration-accepted-by-two-hosts-can-still-resolve")),
        ))),
        Box::new(doc_nav(locale, Some(Route::CustomWidgets), Some(Route::Web))),
    ];
    docs_layout(locale, Route::Styling, view().fill(content))
}
