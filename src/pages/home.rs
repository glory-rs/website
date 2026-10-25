use crate::Locale;
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

pub(crate) fn counter(locale: Locale) -> impl Widget {
    widget(move |scope| {
        let count = scope.cage(0_i32);
        let message = locale
            .context()
            .message(scope.owner(), "counter-clicks", move || {
                let mut args = glory::i18n::Args::new();
                args.set("count", count.get());
                args
            })
            .expect("live counter owner");
        let label = scope.bond(move || message.get().expect("valid counter translation"));
        view()
            .class("live-counter")
            .fill((
                copy("counter-label", label),
                view().class("counter-actions").fill((
                    button()
                        .class("counter-button counter-primary")
                        .label(locale.text("home-add-one"))
                        .name(locale.text("home-add-one-to-the-counter"))
                        .on(activate, move |_| count.revise(|count| *count += 1)),
                    button()
                        .class("counter-button")
                        .label(locale.text("home-reset"))
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

fn topic_card(
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

pub fn home(locale: Locale) -> impl Widget {
    let theme = Theme::blue_dark();
    view().role(Role::Main).class("home-page").fill((
        view().class("hero").fill(
            view().class("hero-inner").fill((
                view().class("hero-copy").fill((
                    copy("eyebrow hero-eyebrow", locale.text("home-semantic-ui-for-rust")),
                    heading(1, "hero-title", locale.text("home-one-language-every-interface")),
                    copy(
                        "hero-lead",
                        locale.text("home-learn-the-model-behind-glory-explicit-semantics-fine-grained"),
                    ),
                    view().class("hero-actions").fill((
                        link(locale, locale.text("home-start-learning"), Route::Docs, "button button-primary"),
                        link(locale, locale.text("home-design-principles"), Route::Architecture, "button button-secondary"),
                    )),
                    view().class("hero-facts").fill((
                        copy("", locale.text("home-no-ui-dsl")),
                        copy("fact-dot", "·"),
                        copy("", locale.text("home-no-virtual-dom")),
                        copy("fact-dot", "·"),
                        copy("", locale.text("home-pure-rust")),
                    )),
                )),
                view().class("hero-art").fill((
                    view().class("logo-orbit orbit-one"),
                    view().class("logo-orbit orbit-two"),
                    image().source("/assets/glory.svg").alt("Glory").class("hero-logo"),
                    copy("hero-art-caption", locale.text("home-build-beyond-the-browser")),
                )),
            )),
        ),
        view().class("intro section-container").fill((
            view().class("intro-copy").fill((
                copy("eyebrow", locale.text("home-designed-to-be-understood")),
                heading(2, "section-title", locale.text("home-a-ui-framework-without-the-hidden-machinery")),
                copy(
                    "section-copy",
                    locale.text("home-glory-keeps-the-application-model-explicit-semantic-values-describe"),
                ),
            )),
            view().class("feature-list").fill((
                feature(locale.text("home-describe-interfaces-with-handwritten-rust-builders-not-a-template")),
                feature(locale.text("home-build-once-then-send-typed-patches-instead-of-diffing")),
                feature(locale.text("home-keep-state-and-subscriptions-attached-to-an-explicit-owner")),
                feature(locale.text("home-share-widgets-across-web-ssr-and-native-rendering-hosts")),
                feature(locale.text("home-inspect-the-same-semantic-model-in-headless-tests-before")),
            )),
        )),
        view().class("showcase-section section-container").fill((
            view().class("centered-heading").fill((
                copy("eyebrow", locale.text("home-state-to-semantics")),
                heading(2, "section-title", locale.text("home-small-pieces-precise-updates")),
                copy(
                    "section-copy",
                    locale.text("home-a-cage-owns-mutable-state-a-derived-label-tracks"),
                ),
            )),
            view().class("example-grid").fill((
                code_panel(locale, theme.clone(), "counter.rs", COUNTER_CODE, ""),
                browser_preview(view().fill((
                    copy("preview-kicker", locale.text("home-live-glory-widget")),
                    heading(3, "preview-title", locale.text("home-a-small-real-counter")),
                    copy("preview-copy", locale.text("home-built-with-glory-s-own-button-text-view-and")),
                    counter(locale, ),
                ))),
            )),
            copy(
                "example-footnote",
                locale.text("home-the-same-widget-implementation-works-in-a-headless-test"),
            ),
        )),
        view().class("language-section").fill(
            view().class("section-container").fill((
                view().class("centered-heading").fill((
                    copy("eyebrow", locale.text("home-rendering-is-a-host-decision")),
                    heading(2, "section-title", locale.text("home-one-widget-tree-multiple-hosts")),
                    copy(
                        "section-copy",
                        locale.text("home-glory-separates-what-your-interface-means-from-how-a"),
                    ),
                )),
                view().class("example-grid").fill((
                    code_panel(locale, theme.clone(), "hydrate.rs", SSR_CODE, ""),
                    view().class("architecture-card").fill((
                        copy("preview-kicker", locale.text("home-the-host-matrix")),
                        view().class("host-row").fill((copy("host-number", "01"), view().fill((heading(3, "host-title", locale.text("common-web")), copy("host-copy", locale.text("home-wasm-dom-rendering-input-geometry-and-scrolling")))))),
                        view().class("host-row").fill((copy("host-number", "02"), view().fill((heading(3, "host-title", locale.text("common-ssr")), copy("host-copy", locale.text("home-static-documents-snapshots-resources-and-optional-streaming")))))),
                        view().class("host-row").fill((copy("host-number", "03"), view().fill((heading(3, "host-title", locale.text("home-native")), copy("host-copy", locale.text("home-blitz-layout-desktop-windows-and-accessibility-bridges")))))),
                        link(locale, locale.text("home-understand-ssr-hydration"), Route::Ssr, "text-link"),
                    )),
                )),
            )),
        ),
        view().class("tools-section section-container").fill((
            view().class("centered-heading").fill((
                copy("eyebrow", locale.text("home-learn-the-framework")),
                heading(2, "section-title", locale.text("home-understand-it-then-build-with-it")),
                copy(
                    "section-copy",
                    locale.text("home-follow-the-guides-from-first-principles-to-a-working"),
                ),
            )),
            view().class("learning-grid learning-grid-home").fill((
                topic_card(locale, locale.text("common-architecture"), locale.text("home-trace-a-cage-revision-through-derived-values-semantic-patches"), Route::Architecture, locale.text("home-read-the-design")),
                topic_card(locale, locale.text("common-custom-widgets"), locale.text("home-implement-the-widget-trait-compose-children-handle-events-and"), Route::CustomWidgets, locale.text("home-write-a-widget")),
                topic_card(locale, locale.text("common-ssr-hydration"), locale.text("home-choose-between-static-html-snapshots-resource-transfer-and-streaming"), Route::Ssr, locale.text("home-render-on-the-server")),
                topic_card(locale, locale.text("home-cross-platform"), locale.text("home-share-one-widget-across-web-ssr-and-blitz-while"), Route::Platforms, locale.text("home-compare-hosts")),
            )),
            view().class("cta").fill((
                copy("eyebrow", locale.text("home-start-with-the-model")),
                heading(2, "cta-title", locale.text("home-know-what-your-ui-is-doing")),
                link(locale, locale.text("home-explore-the-architecture"), Route::Architecture, "button button-primary"),
            )),
        )),
    ))
}
