use crate::Locale;
use crate::{Route, components::link, components::*};
use glory::prelude::*;
use sapid::Theme;

const BUILD_ONCE: &str = r#"use glory::prelude::*;

pub fn counter() -> impl Widget {
    widget(|scope| {
        let count = scope.cage(0_u64);

        col().fill((
            text().kind(TextKind::Heading(1)).content("Counter"),
            text().content(count.map(|count| format!("Count: {count}"))),
            button()
                .label("Add one")
                .on(activate, move |_| count.revise(|count| *count += 1)),
        ))
        .show_in(scope)
    })
}"#;

const GROUPS: &str = r#"use glory::prelude::*;

pub fn actions() -> impl Widget {
    // Logical structure only: no wrapper box is created for the Group.
    group().fill((
        text().content("Choose an action"),
        button().label("Save"),
        button().label("Cancel"),
    ))
}

pub fn toolbar() -> impl Widget {
    // A View has a box and accepts typed layout styles.
    row()
        .style(
            Style::new()
                .display(Display::Flex)
                .flex_direction(FlexDirection::Row)
                .gap(8.0),
        )
        .fill((button().label("Save"), button().label("Cancel")))
}"#;

const STRUCTURE: &str = r#"let expanded = scope.cage(false);

view().fill((
    button()
        .label("Toggle details")
        .on(activate, move |_| expanded.revise(|open| *open = !*open)),
    when(expanded, || {
        group().fill((
            text().kind(TextKind::Heading(2)).content("Details"),
            text().content("This branch has its own owned lifetime."),
        ))
    }),
))
.show_in(scope)"#;

const HOSTS: &str = r#"// Native SSR produces a versioned fragment for later browser adoption.
let snapshot = glory::ssr::snapshot(app())?;
serve_html(snapshot.html);

// The browser mounts the same widget definition into a DOM root.
let web = glory::web::launch(root, app())?;
keep_web_app_alive(web);

// A memory host is useful for semantic tests; it does not paint pixels.
let app = App::memory();
app.mount(app_widget())?;"#;

pub fn architecture(locale: Locale) -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", locale.text("architecture-the-glory-guide"))),
        Box::new(heading(1, "docs-title", locale.text("architecture-architecture-and-design"))),
        Box::new(copy(
            "docs-lead",
            locale.text("architecture-glory-retains-a-typed-semantic-tree-builds-each-mounted"),
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("display-layers-title")),
            copy("", locale.text("display-layers-body")),
            crate::components::link(locale, locale.text("display-home-link"), Route::Display, "text-link"),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("architecture-a-retained-tree-not-a-virtual-dom"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-mounting-a-widget-creates-semantic-nodes-with-stable-identities"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-widget-build-runs-once-for-each-mount-it-creates"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "counter.rs", BUILD_ONCE, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-in-this-example-count-map-creates-a-tracked-derived"),
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("architecture-a-bare-build-time-read-is-not-reactive")),
            copy("", locale.text("architecture-text-content-count-get-to-string-copies-one-value")),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("architecture-from-a-cage-revision-to-a-typed-patch"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-cage-is-the-owned-mutable-source-it-is-not"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-that-distinction-matters-for-both-correctness-and-performance-the"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-this-does-not-make-every-computation-free-a-derived"),
        )),
        Box::new(heading(2, "docs-heading", locale.text("architecture-change-structure-with-control-flow"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-property-bindings-cannot-add-or-remove-semantic-nodes-use"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "structure.rs", STRUCTURE, "docs-code")),
        Box::new(heading(2, "docs-heading", locale.text("architecture-group-is-logical-view-has-a-box"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-use-group-when-children-need-a-logical-parent-but"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-use-view-when-you-need-a-layout-box-typed"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "groups.rs", GROUPS, "docs-code")),
        Box::new(heading(2, "docs-heading", locale.text("architecture-owner-is-the-lifetime-boundary"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-every-ui-scope-wraps-a-core-owner-and-adds"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-a-cage-is-a-copy-generational-handle-not-an"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-ownership-also-explains-independent-component-state-two-mounts-of"),
        )),
        Box::new(link(locale, locale.text("architecture-review-cage-bond-and-vary-ownership"), Route::Reactivity, "text-link")),
        Box::new(heading(2, "docs-heading", locale.text("architecture-one-semantic-model-replaceable-hosts"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-the-shared-widget-describes-semantics-and-behavior-while-the"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-app-memory-provides-a-reference-interpreter-for-semantic-tests"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "hosts.rs", HOSTS, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            locale.text("architecture-portability-is-a-semantic-and-typed-style-contract-not"),
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("architecture-the-mental-model")),
            copy("", locale.text("architecture-build-structure-once-bind-changing-properties-explicitly-let-control")),
        ))),
        Box::new(doc_nav(locale, Some(Route::Docs), Some(Route::Reactivity))),
    ];
    docs_layout(locale, Route::Architecture, view().fill(content))
}
