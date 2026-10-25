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
    })
}"#;

const SSR_CODE: &str = r#"// One widget tree, multiple rendering hosts.
let snapshot = glory::ssr::snapshot_with(|scope| {
    app(scope)
})?;

// The browser adopts the existing semantic HTML.
let app = glory::web::hydrate_with(root, |scope| {
    app(scope)
})?;"#;

const SAPID_CODE: &str = r#"let theme = sapid::Theme::blue_dark();

sapid::card()
    .theme(theme.clone())
    .fill((
        sapid::title(2).content("Hello, Glory"),
        sapid::button()
            .theme(theme)
            .label("Build something"),
    ))"#;

pub(crate) fn counter() -> impl Widget {
    widget(|scope| {
        let count = scope.cage(0_i32);
        let label = count.map(|count| format!("Clicks: {count}"));
        view()
            .class("live-counter")
            .fill((
                copy("counter-label", label),
                view().class("counter-actions").fill((
                    sapid::button()
                        .theme(Theme::blue_dark())
                        .label("Add one")
                        .name("Add one to the counter")
                        .on_press(move || count.revise(|count| *count += 1)),
                    sapid::button()
                        .theme(Theme::blue_dark())
                        .variant(sapid::ButtonVariant::Secondary)
                        .label("Reset")
                        .on_press(move || count.revise(|count| *count = 0)),
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
                        "A fine-grained, host-agnostic UI framework with the safety and expressiveness of Rust.",
                    ),
                    view().class("hero-actions").fill((
                        link("Get started  →", Route::Docs, "button button-primary"),
                        external(
                            "Explore the source",
                            "https://github.com/glory-rs/glory",
                            "button button-secondary",
                        ),
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
                    image()
                        .source("/assets/glory.svg")
                        .alt("Glory")
                        .class("hero-logo"),
                    copy("hero-art-caption", "Build beyond the browser."),
                )),
            )),
        ),
        view().class("intro section-container").fill((
            view().class("intro-copy").fill((
                copy("eyebrow", "A SOLID FOUNDATION"),
                heading(2, "section-title", "Build interactive applications in Rust."),
                copy(
                    "section-copy",
                    "Glory combines explicit semantic widgets, owned fine-grained reactivity, and replaceable rendering hosts in one coherent framework.",
                ),
            )),
            view().class("feature-list").fill((
                feature("Write interfaces with handwritten Rust builders—no template language required."),
                feature("Keep state local and explicit with Cage, Bond, and Lotus."),
                feature("Render to WebAssembly, server-side HTML, or native desktop hosts."),
                feature("Adopt server-rendered DOM with structured, validated hydration."),
                feature("Use typed styles and accessible semantics across every host."),
            )),
        )),
        view().class("showcase-section section-container").fill((
            view().class("centered-heading").fill((
                copy("eyebrow", "FAMILIAR PATTERNS, PRECISE UPDATES"),
                heading(2, "section-title", "Your favorite UI patterns."),
                copy(
                    "section-copy",
                    "Compose self-contained widgets with reactive state that is easy to read, test, and own.",
                ),
            )),
            view().class("example-grid").fill((
                code_panel(theme.clone(), "counter.rs", COUNTER_CODE, ""),
                browser_preview(view().fill((
                    copy("preview-kicker", "LIVE GLORY WIDGET"),
                    heading(3, "preview-title", "A small, real counter"),
                    copy("preview-copy", "Hydrated from the same Rust tree you see on the left."),
                    counter(),
                ))),
            )),
            copy(
                "example-footnote",
                "Fine-grained reactive values trigger targeted semantic updates without diffing a virtual DOM.",
            ),
        )),
        view().class("language-section").fill(
            view().class("section-container").fill((
                view().class("centered-heading").fill((
                    copy("eyebrow", "YOUR FAVORITE LANGUAGE"),
                    heading(2, "section-title", "From the server to the screen."),
                    copy(
                        "section-copy",
                        "Share one semantic tree between synchronous SSR and the browser. Hydration validates the complete subtree before adopting it.",
                    ),
                )),
                view().class("example-grid").fill((
                    code_panel(theme.clone(), "hydrate.rs", SSR_CODE, ""),
                    view().class("architecture-card").fill((
                        copy("preview-kicker", "ONE TREE, THREE HOSTS"),
                        view().class("host-row").fill((copy("host-number", "01"), view().fill((heading(3, "host-title", "Web"), copy("host-copy", "WASM DOM rendering, input, geometry, and scrolling."))))),
                        view().class("host-row").fill((copy("host-number", "02"), view().fill((heading(3, "host-title", "SSR"), copy("host-copy", "Static documents, snapshots, and optional streaming."))))),
                        view().class("host-row").fill((copy("host-number", "03"), view().fill((heading(3, "host-title", "Native"), copy("host-copy", "Blitz layout, desktop windows, and accessibility bridges."))))),
                    )),
                )),
            )),
        ),
        view().class("tools-section section-container").fill((
            view().class("centered-heading").fill((
                copy("eyebrow", "A COMPOSABLE ECOSYSTEM"),
                heading(2, "section-title", "Your favorite tools."),
                copy(
                    "section-copy",
                    "Sapid adds thoughtfully designed widgets, themes, code highlighting, forms, navigation, and more—all built on Glory semantics.",
                ),
            )),
            view().class("example-grid").fill((
                code_panel(theme, "sapid.rs", SAPID_CODE, ""),
                view().class("sapid-preview").fill((
                    copy("sapid-glyph", "✦"),
                    heading(3, "preview-title", "Widgets with good manners."),
                    copy("preview-copy", "Composable builders. Expressive themes. Accessible behavior included."),
                    view().class("sapid-tags").fill((
                        sapid::badge().theme(Theme::blue_dark()).content("Forms"),
                        sapid::badge().theme(Theme::blue_dark()).content("Navigation"),
                        sapid::badge().theme(Theme::blue_dark()).content("Data display"),
                    )),
                    link("Meet Sapid  →", Route::Sapid, "text-link"),
                )),
            )),
            view().class("cta").fill((
                copy("eyebrow", "START WITH A SINGLE WIDGET"),
                heading(2, "cta-title", "Make something glorious."),
                link("Read the guide  →", Route::Docs, "button button-primary"),
            )),
        )),
    ))
}
