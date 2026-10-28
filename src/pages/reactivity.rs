use crate::Locale;
use crate::{Route, components::*, pages::home::counter};
use glory::prelude::*;
use sapid::Theme;

const REACTIVE_CODE: &str = r#"let count = scope.cage(1_u64);
let parity = scope
    .bond(move || count.get() % 2)
    .with_partial_eq();

scope.binding(move || {
    println!("parity: {}", parity.get());
});

// Only affected consumers are notified.
count.revise(|value| *value += 1);"#;

pub fn reactivity(locale: Locale) -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", locale.text("reactivity-state-updates"))),
        Box::new(heading(1, "docs-title", locale.text("reactivity-fine-grained-reactivity"))),
        Box::new(copy(
            "docs-lead",
            locale.text("reactivity-cage-bond-and-vary-make-data-flow-explicit-values"),
        )),
        Box::new(view().class("reactive-concepts").fill((
            view().class("concept-card").fill((
                copy("concept-symbol", "C"),
                heading(2, "concept-title", "Cage"),
                copy(
                    "",
                    locale.text(
                        "reactivity-an-owned-copy-handle-to-mutable-application-state-revise",
                    ),
                ),
            )),
            view().class("concept-card").fill((
                copy("concept-symbol", "B"),
                heading(2, "concept-title", "Bond"),
                copy(
                    "",
                    locale.text("reactivity-a-tracked-derived-value-equality-gates-are-opt-in"),
                ),
            )),
            view().class("concept-card").fill(
                (
                    copy("concept-symbol", "V"),
                    heading(2, "concept-title", "Vary"),
                    copy(
                        "",
                        locale.text(
                            "reactivity-a-uniform-value-input-that-lets-widget-builders-accept",
                        ),
                    ),
                ),
            ),
        ))),
        Box::new(heading(
            2,
            "docs-heading",
            locale.text("reactivity-derived-values-without-the-mystery"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("reactivity-reads-inside-bond-binding-and-effect-computations-are-tracked"),
        )),
        Box::new(code_panel(
            locale,
            Theme::blue_dark(),
            "reactivity.rs",
            REACTIVE_CODE,
            "docs-code",
        )),
        Box::new(heading(2, "docs-heading", locale.text("reactivity-try-a-live-widget"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("reactivity-this-counter-is-running-through-glory-s-browser-host"),
        )),
        Box::new(view().class("docs-playground").fill(counter(locale))),
        Box::new(heading(
            2,
            "docs-heading",
            locale.text("reactivity-ownership-is-part-of-the-model"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("reactivity-dropping-a-scope-invalidates-its-handles-and-cancels-owned"),
        )),
        Box::new(doc_nav(locale, Some(Route::Architecture), Some(Route::Display))),
    ];
    docs_layout(locale, Route::Reactivity, view().fill(content))
}
