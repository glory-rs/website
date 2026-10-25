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

fn learning_card(title: &str, content: &str, route: Route, label: &str) -> impl Widget {
    view().class("learning-card").fill((
        heading(3, "docs-card-title", title),
        copy("docs-paragraph", content),
        link(label, route, "text-link"),
    ))
}

pub fn docs() -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", "THE GLORY GUIDE")),
        Box::new(heading(1, "docs-title", "Learn Glory from first principles.")),
        Box::new(copy(
            "docs-lead",
            "Glory is a semantic UI framework built around handwritten Rust builders, owned fine-grained reactivity, and replaceable rendering hosts. This guide explains not just what to type, but how the pieces fit together.",
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "Before you start"),
            copy("", "The guide assumes basic Rust knowledge. Glory's current semantic architecture is under active development, so follow the migrated examples and implementation ledger rather than retired HTML-era tutorials."),
        ))),
        Box::new(heading(2, "docs-heading", "Choose your learning path")),
        Box::new(view().class("learning-grid").fill((
            learning_card("1. Understand the model", "Learn why Glory uses a retained semantic tree, explicit reactive ownership, and typed updates instead of a virtual DOM.", Route::Architecture, "Architecture & design  →"),
            learning_card("2. Build an interface", "Use built-in elements, derive reactive values, and implement your own Widget trait in plain Rust.", Route::CustomWidgets, "Write a custom widget  →"),
            learning_card("3. Choose a host", "Launch in a browser, generate SSR snapshots, or mount the same widget in a native Blitz window.", Route::Platforms, "Explore platforms  →"),
        ))),
        Box::new(heading(2, "docs-heading", "Install the framework")),
        Box::new(copy("docs-paragraph", "Glory requires Rust 1.94 or newer. Add the facade and select only the hosts your application uses. The web feature includes HTML mapping; hydrate adds browser adoption support.")),
        Box::new(code_panel(Theme::blue_dark(), "Cargo.toml", INSTALL, "docs-code")),
        Box::new(heading(2, "docs-heading", "Create your first widget")),
        Box::new(copy("docs-paragraph", "Widgets are ordinary Rust values. Builders describe semantic nodes, while mounting connects them to a host. There is no template language and no browser DOM in this shared function.")),
        Box::new(code_panel(Theme::blue_dark(), "main.rs", FIRST_APP, "docs-code")),
        Box::new(heading(2, "docs-heading", "What happens next?")),
        Box::new(copy("docs-paragraph", "The browser host creates real DOM from the semantic tree. A Cage revision runs only affected derived computations and bindings, which emit typed field patches. SSR and native hosts consume the same semantic operations through different adapters.")),
        Box::new(view().class("docs-host-grid").fill((
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "Declare"), copy("", "Compose views, text, controls, and your own Widget values with handwritten builders."))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "React"), copy("", "Store mutable sources in Cage handles and derive tracked values with Bond computations."))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "Render"), copy("", "Let Web, SSR, or Blitz interpret the same semantic creation and patch operations."))),
        ))),
        Box::new(doc_nav(None, Some(Route::Architecture))),
    ];
    docs_layout(Route::Docs, view().fill(content))
}
