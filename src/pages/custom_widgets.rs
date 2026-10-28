use crate::Locale;
use crate::{Route, components::link, components::*};
use glory::prelude::*;
use sapid::Theme;

const COUNTER: &str = r#"use glory::prelude::*;

pub struct Counter;

impl Widget for Counter {
    fn build(&mut self, scope: &Scope) -> Result {
        let count = scope.cage(0_i32);
        let label = count.map(|count| format!("Count: {count}"));

        col()
            .class("counter")
            .fill((
                text().kind(TextKind::Heading(1)).content("Counter"),
                text().content(label),
                button()
                    .label("Increment")
                    .on(activate, move |event| {
                        if matches!(event, Event::Activate) {
                            count.revise(|count| *count += 1);
                        }
                    }),
            ))
            .show_in(scope)
    }
}"#;

const CHILDREN: &str = r#"use glory::prelude::*;
use glory::ui::{Children, IntoChildren};

pub struct Panel {
    children: Children,
}

impl Panel {
    pub fn new(children: impl IntoChildren) -> Self {
        Self { children: children.into_children() }
    }
}

impl Widget for Panel {
    fn build(&mut self, scope: &Scope) -> Result {
        // build takes &mut self, so consume the configured children once.
        let children = std::mem::replace(&mut self.children, ().into_children());
        view().class("panel").fill(children).show_in(scope)
    }
}

pub fn page() -> impl Widget {
    Panel::new((
        text().kind(TextKind::Heading(1)).content("Account"),
        text().content("Children can be any heterogeneous widget tuple."),
        Counter,
    ))
}"#;

const OVERRIDES: &str = r#"// The second call replaces the complete class list, not just one token.
let button = button()
    .class("primary")
    .class("primary is-active"); // final classes: primary, is-active

// These fields are merged by the Style builder into one Style value.
let spacing = Style::new()
    .padding(Edges::all(12.0))
    .gap(8.0);
let panel = view().style(spacing);

// col() is a View with flex defaults. Replacing its Style must retain them.
let column = col().style(
    Style::new()
        .display(Display::Flex)
        .flex_direction(FlexDirection::Column)
        .gap(12.0),
);"#;

const CLOSURE: &str = r#"use glory::prelude::*;

pub fn save_button() -> impl Widget {
    widget(|scope| {
        let saving = scope.cage(false);
        let label = saving.map(|saving| {
            if *saving { "Saving…" } else { "Save" }.to_owned()
        });

        button()
            .label(label)
            .on(activate, move |_| saving.revise(|saving| *saving = true))
            .show_in(scope)
    })
}"#;

const CLEANUP: &str = r#"widget(|scope| {
    let value = scope.cage(String::new());
    scope.on_cleanup(|| {
        // Release application resources that belong to this mount.
    })?;

    text().content(value).show_in(scope)
})"#;

pub fn custom_widgets(locale: Locale) -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", locale.text("architecture-the-glory-guide"))),
        Box::new(heading(1, "docs-title", locale.text("custom-widgets-writing-custom-widgets"))),
        Box::new(copy(
            "docs-lead",
            locale.text("custom-widgets-implementing-widget-gives-a-reusable-rust-value-its-own"),
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("display-widget-guide-title")),
            copy("", locale.text("display-widget-guide-body")),
            crate::components::link(locale, locale.text("display-home-link"), Route::Display, "text-link"),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("custom-widgets-a-complete-counter"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-the-following-component-is-host-independent-it-contains-no"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "counter.rs", COUNTER, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-the-required-method-is-fn-build-mut-self-scope"),
        )),
        Box::new(heading(2, "docs-heading", locale.text("custom-widgets-state-belongs-to-the-mounted-scope"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-scope-cage-0-i32-creates-mutable-state-owned-by"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-count-map-derives-a-reactive-string-for-text-content"),
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("custom-widgets-build-is-construction-not-a-render-function")),
            copy("", locale.text("custom-widgets-do-not-expect-self-fields-or-a-bare-cage")),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("custom-widgets-activate-is-a-semantic-event"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-on-activate-handler-subscribes-to-eventkind-activate-the-callback"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-the-matches-check-in-counter-is-explicit-about-the"),
        )),
        Box::new(heading(2, "docs-heading", locale.text("custom-widgets-fill-accepts-intochildren"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-view-group-button-and-link-builders-accept-children-through"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-a-reusable-wrapper-can-accept-impl-intochildren-in-its"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "panel.rs", CHILDREN, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-panel-new-heading-body-counter-works-because-the-three"),
        )),
        Box::new(heading(2, "docs-heading", "class and style replace their previous values")),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-calling-class-twice-replaces-the-complete-whitespace-separated-class"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-construct-one-style-with-chained-field-methods-when-you"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "overrides.rs", OVERRIDES, "docs-code")),
        Box::new(heading(2, "docs-heading", locale.text("custom-widgets-children-and-owner-are-linked-not-identical"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-semantic-parentage-and-reactive-ownership-are-related-but-distinct"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-removing-a-mounted-branch-disposes-its-owner-and-descendants"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-you-normally-do-not-need-to-call-scope-child"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "cleanup.rs", CLEANUP, "docs-code")),
        Box::new(heading(2, "docs-heading", locale.text("custom-widgets-choose-a-trait-closure-or-plain-function"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-implement-widget-when-the-component-has-a-reusable-name"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "save_button.rs", CLOSURE, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            locale.text("custom-widgets-a-plain-fn-card-impl-widget-is-enough-for"),
        )),
        Box::new(link(locale, locale.text("custom-widgets-review-the-retained-tree-and-update-model"), Route::Architecture, "text-link")),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("custom-widgets-a-mount-checklist")),
            copy("", locale.text("custom-widgets-allocate-state-from-the-build-scope-finish-the-root")),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("custom-widgets-sapid-heading"))),
        Box::new(copy("docs-paragraph", locale.text("custom-widgets-sapid-description"))),
        Box::new(external(locale.text("navigation-explore-sapid"), crate::navigation::SAPID_URL, "text-link")),
        Box::new(doc_nav(locale, Some(Route::Widgets), Some(Route::Styling))),
    ];
    docs_layout(locale, Route::CustomWidgets, view().fill(content))
}
