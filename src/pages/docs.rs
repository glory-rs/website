use crate::Locale;
use crate::{Route, components::link, components::*};
use glory::prelude::*;
use sapid::Theme;

const INSTALL: &str = r#"[dependencies]
glory = { version = "0.3.1", features = ["web"] }

# Add hosts only when your application needs them:
# features = ["web", "ssr", "hydrate"]"#;

const FIRST_APP: &str = r#"use glory::prelude::*;

fn hello() -> impl Widget {
    col().fill((
        text()
            .kind(TextKind::Heading(1))
            .content("Hello, Glory"),
        text().content("Semantic UI, handwritten in Rust."),
    ))
}

// In a wasm32 browser host:
let app = glory::web::launch(root, hello())?;
// Keep the returned WebApp alive."#;

fn learning_card(
    locale: Locale,
    title: impl Into<String>,
    content: impl Into<String>,
    route: Route,
    label: impl Into<String>,
) -> impl Widget {
    view().class("learning-card").fill((
        heading(3, "docs-card-title", title),
        copy("docs-paragraph", content.into()),
        link(locale, label, route, "text-link"),
    ))
}

pub fn docs(locale: Locale) -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", locale.text("architecture-the-glory-guide"))),
        Box::new(heading(1, "docs-title", locale.text("docs-learn-glory-from-first-principles"))),
        Box::new(copy(
            "docs-lead",
            locale.text("docs-glory-is-a-semantic-ui-framework-built-around-handwritten"),
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("docs-before-you-start")),
            copy("", locale.text("docs-the-guide-assumes-basic-rust-knowledge-glory-s-current")),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("docs-choose-your-learning-path"))),
        Box::new(view().class("learning-grid").fill((
            learning_card(
                locale,
                locale.text("docs-1-understand-the-model"),
                locale.text("docs-learn-why-glory-uses-a-retained-semantic-tree-explicit"),
                Route::Architecture,
                locale.text("docs-architecture-design"),
            ),
            learning_card(
                locale,
                locale.text("docs-2-build-an-interface"),
                locale.text("docs-use-built-in-elements-derive-reactive-values-and-implement"),
                Route::CustomWidgets,
                locale.text("docs-write-a-custom-widget"),
            ),
            learning_card(
                locale,
                locale.text("docs-3-choose-a-host"),
                locale.text("docs-launch-in-a-browser-generate-ssr-snapshots-or-mount"),
                Route::Platforms,
                locale.text("docs-explore-platforms"),
            ),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("docs-install-the-framework"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("docs-glory-requires-rust-1-94-or-newer-add-the"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "Cargo.toml", INSTALL, "docs-code")),
        Box::new(heading(2, "docs-heading", locale.text("docs-create-your-first-widget"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("docs-widgets-are-ordinary-rust-values-builders-describe-semantic-nodes"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "main.rs", FIRST_APP, "docs-code")),
        Box::new(heading(2, "docs-heading", locale.text("docs-what-happens-next"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("docs-the-browser-host-creates-real-dom-from-the-semantic"),
        )),
        Box::new(view().class("docs-host-grid").fill((
            view().class("docs-host-card").fill((
                heading(3, "docs-card-title", locale.text("docs-declare")),
                copy(
                    "",
                    locale.text("docs-compose-views-text-controls-and-your-own-widget-values"),
                ),
            )),
            view().class("docs-host-card").fill((
                heading(3, "docs-card-title", locale.text("docs-react")),
                copy(
                    "",
                    locale.text("docs-store-mutable-sources-in-cage-handles-and-derive-tracked"),
                ),
            )),
            view().class("docs-host-card").fill((
                heading(3, "docs-card-title", locale.text("docs-render")),
                copy("", locale.text("docs-let-web-ssr-or-blitz-interpret-the-same-semantic")),
            )),
        ))),
        Box::new(doc_nav(locale, None, Some(Route::Architecture))),
    ];
    docs_layout(locale, Route::Docs, view().fill(content))
}
