use crate::{Locale, Route, components::link, components::*};
use glory::prelude::*;
use sapid::Theme;

const INSTALL: &str = r#"[dependencies]
glory = { git = "https://github.com/glory-rs/glory", rev = "3c0c6576a38ddbd48211012f2a60ecaa7140816e", default-features = false, features = ["dsl"] }

# dsl includes web and the optional XML compiler.
# Handwritten display templates only need features = ["web"]."#;

const HANDWRITTEN: &str = include_str!("../examples/display_label.rs");

pub fn display(locale: Locale) -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", locale.text("architecture-the-glory-guide"))),
        Box::new(heading(1, "docs-title", locale.text("display-title"))),
        Box::new(copy("docs-lead", locale.text("display-lead"))),
        Box::new(heading(2, "docs-heading", locale.text("display-layers-title"))),
        Box::new(copy("docs-paragraph", locale.text("display-layers-body"))),
        Box::new(heading(2, "docs-heading", locale.text("display-install-title"))),
        Box::new(code_panel(locale, Theme::blue_dark(), "Cargo.toml", INSTALL, "docs-code")),
        Box::new(copy("docs-paragraph", locale.text("display-imports-body"))),
        Box::new(heading(2, "docs-heading", locale.text("display-counter-title"))),
        Box::new(code_panel(
            locale,
            Theme::blue_dark(),
            "display_counter.rs",
            include_str!("../examples/display_counter.rs"),
            "docs-code",
        )),
        Box::new(copy("docs-paragraph", locale.text("display-bindings-body"))),
        Box::new(copy("docs-paragraph", locale.text("display-start-body"))),
        Box::new(heading(2, "docs-heading", locale.text("display-handwritten-title"))),
        Box::new(code_panel(locale, Theme::blue_dark(), "label.rs", HANDWRITTEN, "docs-code")),
        Box::new(copy("docs-paragraph", locale.text("display-handwritten-body"))),
        Box::new(heading(2, "docs-heading", locale.text("display-structure-title"))),
        Box::new(code_panel(
            locale,
            Theme::blue_dark(),
            "display_rows.rs",
            include_str!("../examples/display_rows.rs"),
            "docs-code",
        )),
        Box::new(copy("docs-paragraph", locale.text("display-keyed-body"))),
        Box::new(copy("docs-paragraph", locale.text("display-content-body"))),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("display-boundary-title")),
            copy("", locale.text("display-boundary-body")),
        ))),
        Box::new(link(locale, locale.text("display-semantic-link"), Route::Widgets, "text-link")),
        Box::new(doc_nav(locale, Some(Route::Reactivity), Some(Route::Widgets))),
    ];
    docs_layout(locale, Route::Display, view().fill(content))
}
