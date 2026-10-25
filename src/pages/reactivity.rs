use crate::{Route, components::link, components::*, pages::home::counter};
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

pub fn reactivity() -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", "STATE & UPDATES")),
        Box::new(heading(1, "docs-title", "Fine-grained reactivity.")),
        Box::new(copy(
            "docs-lead",
            "Cage, Bond, and Lotus make data flow explicit. Values and subscriptions belong to an Owner, so their lifetime stays connected to the widget tree.",
        )),
        Box::new(view().class("reactive-concepts").fill((
            view().class("concept-card").fill((copy("concept-symbol", "C"), heading(2, "concept-title", "Cage"), copy("", "An owned, Copy handle to mutable application state. Revise it to notify dependent computations."))),
            view().class("concept-card").fill((copy("concept-symbol", "B"), heading(2, "concept-title", "Bond"), copy("", "A tracked derived value. Equality gates are opt-in and can avoid redundant downstream work."))),
            view().class("concept-card").fill((copy("concept-symbol", "L"), heading(2, "concept-title", "Lotus"), copy("", "A uniform value input that lets widget builders accept static or reactive content."))),
        ))),
        Box::new(heading(2, "docs-heading", "Derived values without the mystery")),
        Box::new(copy("docs-paragraph", "Reads inside Bond, binding, and effect computations are tracked. A bare read while constructing a widget is not a subscription.")),
        Box::new(code_panel(Theme::blue_dark(), "reactivity.rs", REACTIVE_CODE, "docs-code")),
        Box::new(heading(2, "docs-heading", "Try a live widget")),
        Box::new(copy("docs-paragraph", "This counter is running through Glory's browser host. Its label derives directly from the same Cage used by the event handlers.")),
        Box::new(view().class("docs-playground").fill(counter())),
        Box::new(heading(2, "docs-heading", "Ownership is part of the model")),
        Box::new(copy("docs-paragraph", "Dropping a scope invalidates its handles and cancels owned work. Collection projections and indexed lists preserve precise dependencies without scanning an entire data source.")),
        Box::new(link("Back to getting started  →", Route::Docs, "text-link")),
    ];
    docs_layout(Route::Reactivity, view().fill(content))
}
