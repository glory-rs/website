use crate::{Route, components::link, components::*};
use glory::prelude::*;
use sapid::Theme;

const INSTALL: &str = r#"[dependencies]
glory = { version = "0.3.1", features = ["web"] }

# For server-side snapshots and hydration:
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

pub fn docs() -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", "THE GLORY GUIDE")),
        Box::new(heading(1, "docs-title", "Build your first Glory app.")),
        Box::new(copy(
            "docs-lead",
            "Glory is a semantic UI framework built around handwritten Rust builders, owned fine-grained reactivity, and replaceable rendering hosts.",
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "Active development"),
            copy("", "The current semantic architecture is a breaking rewrite. Follow the implementation ledger and migrated examples in the Glory repository for the supported APIs."),
        ))),
        Box::new(heading(2, "docs-heading", "Install the framework")),
        Box::new(copy("docs-paragraph", "Glory requires Rust 1.94 or newer. Add the facade to your application and select the hosts you need.")),
        Box::new(code_panel(Theme::blue_dark(), "Cargo.toml", INSTALL, "docs-code")),
        Box::new(heading(2, "docs-heading", "Create a widget")),
        Box::new(copy("docs-paragraph", "Widgets are ordinary Rust values. Compose views, text, controls, and other widgets without a UI template language.")),
        Box::new(code_panel(Theme::blue_dark(), "main.rs", FIRST_APP, "docs-code")),
        Box::new(heading(2, "docs-heading", "Choose a rendering host")),
        Box::new(view().class("docs-host-grid").fill((
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "Web"), copy("", "Enable web for a WASM DOM application. Keep the returned WebApp alive for the lifetime of the interface."))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "SSR"), copy("", "Enable ssr for HTML documents and snapshots. Add hydrate when the browser should adopt the generated DOM."))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "Native"), copy("", "The optional Blitz host provides native layout and desktop integration with the same semantic widget model."))),
        ))),
        Box::new(heading(2, "docs-heading", "Keep exploring")),
        Box::new(copy("docs-paragraph", "Next, learn how owned reactive values connect application state to precise interface updates.")),
        Box::new(link("Fine-grained reactivity  →", Route::Reactivity, "text-link")),
    ];
    docs_layout(Route::Docs, view().fill(content))
}
