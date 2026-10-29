use crate::Locale;
use crate::{Route, components::*};
use glory::prelude::*;
use sapid::Theme;

const SHARED_COUNTER: &str = r#"use glory::prelude::*;

pub fn counter() -> impl Widget {
    widget(|scope| {
        let count = scope.cage(0_u64);
        col().class("counter").fill((
            text().content(count.map(|count| format!("Count: {count}"))),
            button()
                .label("Increment")
                .on(activate, move |_| count.revise(|value| *value += 1)),
        ))
        .show_in(scope)
    })
}"#;

const FEATURES: &str = r#"[features]
default = []
dsl = ["glory/dsl"]
web = ["glory/web"]
ssr = ["glory/ssr"]
hydrate = ["glory/hydrate"]
stream = ["glory/stream"]
stream-web = ["glory/stream", "glory/hydrate"]
apace = ["glory/apace"]
desktop = ["glory/apace-shell"]
desktop-accessibility = ["glory/apace-accesskit"]

[dependencies]
glory = { version = "=0.6.0", default-features = false }"#;

const WEB_AND_SSR: &str = r#"// Native server: produce a versioned fragment for an otherwise empty mount.
let snapshot = glory::ssr::snapshot(counter())?;
let fragment = snapshot.html;

// wasm32 browser CSR: mount into an empty element and retain the WebApp.
let app = glory::web::launch(csr_root, counter())?;

// wasm32 browser hydration: adopt the matching SSR fragment instead.
// Use the same initial widget tree, and retain this WebApp as well.
let app = glory::web::hydrate(hydration_root, counter())?;"#;

const DESKTOP: &str = r#"#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use glory::apace::shell::{Shell, WindowApp, WindowAttributes};

    let app = WindowApp::new();
    app.mount(counter())?;

    let mut shell = Shell::new()?;
    shell.add_window(
        app,
        WindowAttributes::default().with_title("Glory counter"),
    );
    shell.run()?;
    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn main() {}"#;

pub fn platforms(locale: Locale) -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", locale.text("platforms-one-tree-multiple-hosts"))),
        Box::new(heading(1, "docs-title", locale.text("platforms-share-widgets-across-platforms"))),
        Box::new(copy(
            "docs-lead",
            locale.text("platforms-keep-application-components-in-terms-of-widget-scope-semantic"),
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("display-widget-guide-title")),
            copy("", locale.text("display-widget-guide-body")),
            crate::components::link(locale, locale.text("display-home-link"), Route::Display, "text-link"),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("platforms-write-the-shared-component-once"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("platforms-this-component-contains-no-web-sys-html-document-or"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "shared.rs", SHARED_COUNTER, "docs-code")),
        Box::new(heading(2, "docs-heading", locale.text("platforms-forward-only-the-features-you-use"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("platforms-glory-has-no-default-host-application-features-forward-to"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "Cargo.toml", FEATURES, "docs-code")),
        Box::new(heading(2, "docs-heading", locale.text("platforms-choose-the-host-entry-point"))),
        Box::new(code_panel(locale, Theme::blue_dark(), "web_and_server.rs", WEB_AND_SSR, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            locale.text("platforms-launch-requires-an-empty-unclaimed-browser-mount-hydrate-expects"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "desktop.rs", DESKTOP, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            locale.text("platforms-windowapp-owns-the-shared-app-and-native-document-shell"),
        )),
        Box::new(heading(2, "docs-heading", locale.text("platforms-capability-matrix"))),
        Box::new(view().class("docs-host-grid").fill((
            view().class("docs-host-card").fill((heading(3, "docs-card-title", locale.text("platforms-headless-no-feature")), copy("", locale.text("platforms-app-memory-records-the-semantic-tree-and-typed-patches")))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", locale.text("platforms-web-csr-web")), copy("", locale.text("platforms-wasm32-dom-rendering-with-browser-input-focus-measurement-scrolling")))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", locale.text("platforms-ssr-ssr")), copy("", locale.text("platforms-native-synchronous-html-strings-documents-and-versioned-snapshots-through")))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", locale.text("platforms-hydration-hydrate")), copy("", locale.text("platforms-browser-adoption-of-a-compatible-ssr-snapshot-through-hydrate")))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", locale.text("platforms-streaming-stream")), copy("", locale.text("platforms-native-pull-driven-shell-patch-end-output-with-request")))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", locale.text("platforms-apace-document-apace")), copy("", locale.text("platforms-native-headless-css-box-text-layout-measurement-and-programmatic")))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", locale.text("platforms-apace-desktop-apace-shell")), copy("", locale.text("platforms-windowapp-and-shell-add-cpu-desktop-windows-activation-focus")))),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("platforms-mobile-title"))),
        Box::new(copy("docs-paragraph", locale.text("platforms-mobile-body"))),
        Box::new(external(locale.text("platforms-mobile-link"), "https://github.com/glory-rs/glory/blob/v0.6.0/examples/mobile/README.md", "text-link")),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("platforms-sharing-semantics-does-not-erase-platform-differences")),
            copy("", locale.text("platforms-cross-platform-widgets-do-not-guarantee-pixel-identical-output")),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("platforms-keep-current-platform-limits-explicit"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("platforms-the-desktop-shell-is-gated-to-windows-linux-and"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("platforms-ime-protocol-handling-is-not-proof-that-every-operating"),
        )),
        Box::new(doc_nav(locale, Some(Route::Ssr), None)),
    ];
    docs_layout(locale, Route::Platforms, view().fill(content))
}
