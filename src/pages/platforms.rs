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
web = ["glory/web"]
ssr = ["glory/ssr"]
hydrate = ["glory/hydrate"]
stream = ["glory/stream"]
stream-web = ["glory/stream", "glory/hydrate"]
blitz = ["glory/blitz"]
desktop = ["glory/blitz-shell"]
desktop-accessibility = ["glory/blitz-accesskit"]

[dependencies]
glory = { version = "0.3.1", default-features = false }"#;

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
    use glory::blitz::shell::{Shell, WindowApp, WindowAttributes};

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

pub fn platforms() -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", "ONE TREE, MULTIPLE HOSTS")),
        Box::new(heading(1, "docs-title", "Share widgets across platforms.")),
        Box::new(copy(
            "docs-lead",
            "Keep application components in terms of Widget, Scope, semantic events, and typed styles. Put DOM lookup, HTML generation, window creation, and event loops in small host-specific entry points.",
        )),
        Box::new(heading(2, "docs-heading", "Write the shared component once")),
        Box::new(copy(
            "docs-paragraph",
            "This component contains no web-sys, HTML document, or window types. Its Cage and event handler have the same ownership and update behavior wherever the widget is mounted.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "shared.rs", SHARED_COUNTER, "docs-code")),
        Box::new(heading(2, "docs-heading", "Forward only the features you use")),
        Box::new(copy(
            "docs-paragraph",
            "Glory has no default host. Application features forward to facade features, keeping server, browser, and native dependency graphs explicit. web and ssr each select html; hydrate selects web; stream selects ssr and resources; blitz is native-only. For a progressive browser client, stream-web selects stream and hydrate (and therefore web); hydrate the application after the final End frame.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "Cargo.toml", FEATURES, "docs-code")),
        Box::new(heading(2, "docs-heading", "Choose the host entry point")),
        Box::new(code_panel(Theme::blue_dark(), "web_and_server.rs", WEB_AND_SSR, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "launch requires an empty, unclaimed browser mount. hydrate expects Glory's versioned SSR markers and validates the entire initial subtree before adopting it. The server has no browser input loop; interactivity begins with the browser host.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "desktop.rs", DESKTOP, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "WindowApp owns the shared App and native document. Shell owns the desktop event loop and windows. Enable blitz-shell for this entry point; blitz-accesskit additionally enables the optional desktop accessibility bridge.",
        )),
        Box::new(heading(2, "docs-heading", "Capability matrix")),
        Box::new(view().class("docs-host-grid").fill((
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "Headless · no feature"), copy("", "App::memory() records the semantic tree and typed patches for tests. Profile::RECORD performs no layout. There is no physical input, focus, geometry, or platform action driver."))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "Web CSR · web"), copy("", "wasm32 DOM rendering with browser input, focus, measurement, scrolling, resources, and RAF scheduling. launch creates a new tree; keep WebApp alive."))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "SSR · ssr"), copy("", "Native synchronous HTML strings, documents, and versioned snapshots through render, render_document, and snapshot. Event interests are recorded, but SSR has no physical input or geometry."))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "Hydration · hydrate"), copy("", "Browser adoption of a compatible SSR snapshot through hydrate or hydrate_with. It includes web, validates before changing the DOM, and preserves the mount on mismatch."))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "Streaming · stream"), copy("", "Native pull-driven Shell/Patch/End output with request resources and cancellation. Add web for the classic pre-edit script and hydrate for final application ownership; streaming alone is not a browser input host."))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "Blitz document · blitz"), copy("", "Native headless CSS/box/text layout, measurement, and programmatic scrolling. Optional paint/raster adds frames or CPU images. A standalone Renderer has no OS input driver."))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "Blitz desktop · blitz-shell"), copy("", "WindowApp and Shell add CPU desktop windows, activation, focus, navigation, controlled editing, and IME protocol integration on the desktop targets. AccessKit is a separate opt-in."))),
        ))),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "Sharing semantics does not erase platform differences"),
            copy("", "Cross-platform widgets do not guarantee pixel-identical output. Fonts, shaping, viewport/DPI behavior, CSS coverage, and native presentation differ. A green headless test proves neither layout nor displayed pixels, and neither headless memory nor a standalone Blitz Renderer supplies user input."),
        ))),
        Box::new(heading(2, "docs-heading", "Keep current platform limits explicit")),
        Box::new(copy(
            "docs-paragraph",
            "The desktop shell is gated to Windows, Linux, and macOS; it is not an Android host. Android Activity/Surface integration, mobile resources, input connections, and device acceptance remain separate work. Do not infer mobile support from an Android compile check or from desktop winit dependencies.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "IME protocol handling is not proof that every operating-system IME works. Likewise, the optional AccessKit bridge and Windows UI Automation evidence are not screen-reader acceptance. Unix AT-SPI client checks, spoken announcements, complete text-selection interfaces, OS IME behavior, and mobile accessibility still require platform-specific validation.",
        )),
        Box::new(doc_nav(Some(Route::Ssr), None)),
    ];
    docs_layout(Route::Platforms, view().fill(content))
}
