use crate::{Route, components::link, components::*};
use glory::prelude::*;
use glory::render::Role;
use sapid::Theme;

const COUNTER_CODE: &str = r#"use glory::prelude::*;

pub fn counter() -> impl Widget {
    widget(|scope| {
        let count = scope.cage(0_i32);
        let label = count.map(|n| format!("Clicks: {n}"));

        row().fill((
            text().content(label),
            button()
                .label("Add one")
                .on(activate, move |_| {
                    count.revise(|n| *n += 1)
                }),
        ))
        .show_in(scope)
    })
}"#;

const SSR_CODE: &str = r#"// Serialize a versioned semantic snapshot.
let snapshot = glory::ssr::snapshot(counter())?;
let html = format!("<main id=\"app\">{}</main>", snapshot.html);

// Adopt that exact HTML in the browser.
let app = glory::web::hydrate(root, counter())?;
// Keep the WebApp alive."#;

pub(crate) fn counter() -> impl Widget {
    widget(|scope| {
        let count = scope.cage(0_i32);
        let label = count.map(|count| format!("Clicks: {count}"));
        view()
            .class("live-counter")
            .fill((
                copy("counter-label", label),
                view().class("counter-actions").fill((
                    button()
                        .class("counter-button counter-primary")
                        .label("Add one")
                        .name("Add one to the counter")
                        .on(activate, move |_| count.revise(|count| *count += 1)),
                    button()
                        .class("counter-button")
                        .label("Reset")
                        .on(activate, move |_| count.revise(|count| *count = 0)),
                )),
            ))
            .show_in(scope)
    })
}

fn browser_preview(content: impl Widget) -> impl Widget {
    view().class("browser-preview").fill((
        view().class("browser-chrome").fill((
            view().class("browser-dots").fill((copy("", "●"), copy("", "●"), copy("", "●"))),
            copy("browser-address", "localhost:8796"),
        )),
        view().class("browser-content").fill(content),
    ))
}

fn topic_card(title: &str, content: &str, route: Route, label: &str) -> impl Widget {
    view().class("learning-card").fill((
        heading(3, "docs-card-title", title),
        copy("docs-paragraph", content),
        link(label, route, "text-link"),
    ))
}

pub fn home() -> impl Widget {
    let theme = Theme::blue_dark();
    view().role(Role::Main).class("home-page").fill((
        view().class("hero").fill(
            view().class("hero-inner").fill((
                view().class("hero-copy").fill((
                    copy("eyebrow hero-eyebrow", "SEMANTIC UI FOR RUST"),
                    heading(1, "hero-title", "One language.\nEvery interface."),
                    copy(
                        "hero-lead",
                        "Learn the model behind Glory: explicit semantics, fine-grained state, and rendering hosts you can replace.",
                    ),
                    view().class("hero-actions").fill((
                        link("Start learning  →", Route::Docs, "button button-primary"),
                        link("Design principles", Route::Architecture, "button button-secondary"),
                    )),
                    view().class("hero-facts").fill((
                        copy("", "No UI DSL"),
                        copy("fact-dot", "·"),
                        copy("", "No virtual DOM"),
                        copy("fact-dot", "·"),
                        copy("", "Pure Rust"),
                    )),
                )),
                view().class("hero-art").fill((
                    view().class("logo-orbit orbit-one"),
                    view().class("logo-orbit orbit-two"),
                    image().source("/assets/glory.svg").alt("Glory").class("hero-logo"),
                    copy("hero-art-caption", "Build beyond the browser."),
                )),
            )),
        ),
        view().class("intro section-container").fill((
            view().class("intro-copy").fill((
                copy("eyebrow", "DESIGNED TO BE UNDERSTOOD"),
                heading(2, "section-title", "A UI framework without the hidden machinery."),
                copy(
                    "section-copy",
                    "Glory keeps the application model explicit: semantic values describe the interface, Owners control lifetime, and hosts decide how those semantics become pixels.",
                ),
            )),
            view().class("feature-list").fill((
                feature("Describe interfaces with handwritten Rust builders—not a template compiler."),
                feature("Build once, then send typed patches instead of diffing virtual trees."),
                feature("Keep state and subscriptions attached to an explicit Owner lifetime."),
                feature("Share widgets across Web, SSR, and native rendering hosts."),
                feature("Inspect the same semantic model in headless tests before opening a window."),
            )),
        )),
        view().class("showcase-section section-container").fill((
            view().class("centered-heading").fill((
                copy("eyebrow", "STATE TO SEMANTICS"),
                heading(2, "section-title", "Small pieces. Precise updates."),
                copy(
                    "section-copy",
                    "A Cage owns mutable state. A derived label tracks it. The button emits a semantic Activate event. No component-wide rerender is required.",
                ),
            )),
            view().class("example-grid").fill((
                code_panel(theme.clone(), "counter.rs", COUNTER_CODE, ""),
                browser_preview(view().fill((
                    copy("preview-kicker", "LIVE GLORY WIDGET"),
                    heading(3, "preview-title", "A small, real counter"),
                    copy("preview-copy", "Built with Glory's own button, text, view, and reactive Cage."),
                    counter(),
                ))),
            )),
            copy(
                "example-footnote",
                "The same Widget implementation works in a headless test, a browser host, an SSR snapshot, or a native window.",
            ),
        )),
        view().class("language-section").fill(
            view().class("section-container").fill((
                view().class("centered-heading").fill((
                    copy("eyebrow", "RENDERING IS A HOST DECISION"),
                    heading(2, "section-title", "One widget tree. Multiple hosts."),
                    copy(
                        "section-copy",
                        "Glory separates what your interface means from how a platform presents it. Choose static HTML, hydrated WebAssembly, or native Blitz rendering without rewriting the widget.",
                    ),
                )),
                view().class("example-grid").fill((
                    code_panel(theme.clone(), "hydrate.rs", SSR_CODE, ""),
                    view().class("architecture-card").fill((
                        copy("preview-kicker", "THE HOST MATRIX"),
                        view().class("host-row").fill((copy("host-number", "01"), view().fill((heading(3, "host-title", "Web"), copy("host-copy", "WASM DOM rendering, input, geometry, and scrolling."))))),
                        view().class("host-row").fill((copy("host-number", "02"), view().fill((heading(3, "host-title", "SSR"), copy("host-copy", "Static documents, snapshots, resources, and optional streaming."))))),
                        view().class("host-row").fill((copy("host-number", "03"), view().fill((heading(3, "host-title", "Native"), copy("host-copy", "Blitz layout, desktop windows, and accessibility bridges."))))),
                        link("Understand SSR & hydration  →", Route::Ssr, "text-link"),
                    )),
                )),
            )),
        ),
        view().class("tools-section section-container").fill((
            view().class("centered-heading").fill((
                copy("eyebrow", "LEARN THE FRAMEWORK"),
                heading(2, "section-title", "Understand it. Then build with it."),
                copy(
                    "section-copy",
                    "Follow the guides from first principles to a working cross-platform application. Each page includes concrete APIs, code, and current boundaries.",
                ),
            )),
            view().class("learning-grid learning-grid-home").fill((
                topic_card("Architecture", "Trace a Cage revision through derived values, semantic patches, and a rendering host.", Route::Architecture, "Read the design  →"),
                topic_card("Custom widgets", "Implement the Widget trait, compose children, handle events, and own state correctly.", Route::CustomWidgets, "Write a widget  →"),
                topic_card("SSR & hydration", "Choose between static HTML, snapshots, resource transfer, and streaming.", Route::Ssr, "Render on the server  →"),
                topic_card("Cross-platform", "Share one widget across Web, SSR, and Blitz while respecting host capabilities.", Route::Platforms, "Compare hosts  →"),
            )),
            view().class("cta").fill((
                copy("eyebrow", "START WITH THE MODEL"),
                heading(2, "cta-title", "Know what your UI is doing."),
                link("Explore the architecture  →", Route::Architecture, "button button-primary"),
            )),
        )),
    ))
}
