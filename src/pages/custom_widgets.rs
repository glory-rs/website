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

pub fn custom_widgets() -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", "THE GLORY GUIDE")),
        Box::new(heading(1, "docs-title", "Writing custom widgets.")),
        Box::new(copy(
            "docs-lead",
            "Implementing Widget gives a reusable Rust value its own construction logic, state ownership, children, and semantic event handlers. The trait is small; understanding build, Scope, and mount lifetime is what makes the component correct.",
        )),
        Box::new(heading(2, "docs-heading", "A complete Counter")),
        Box::new(copy(
            "docs-paragraph",
            "The following component is host-independent. It contains no DOM types, browser listeners, HTML strings, or native window calls. Mount Counter through Web, SSR, Blitz, or the memory host and the same semantic tree and event behavior are used.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "counter.rs", COUNTER, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "The required method is fn build(&mut self, scope: &Scope) -> Result. Result here is Glory's UI result alias, so node creation, child mounting, and other fallible operations can use ?. The final col().show_in(scope) mounts the root builder into the Counter's scope and returns that Result. Returning the bare col() would be a type error inside a Widget implementation.",
        )),
        Box::new(heading(2, "docs-heading", "State belongs to the mounted Scope")),
        Box::new(copy(
            "docs-paragraph",
            "scope.cage(0_i32) creates mutable state owned by this mount. The returned Cage is a small Copy handle, so it can be captured by the derived label and the button handler. There is no count field on Counter: the state is runtime-owned and lives for exactly as long as the mounted Scope. Every separate Counter mount gets an independent zero.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "count.map derives a reactive String for text().content. That builder method accepts values convertible into Lotus, allowing either a static String or a reactive source. When the event handler calls revise, the derived value and text binding are scheduled; build is not called again. Avoid creating maps or other reactive nodes inside pure derived computations.",
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "Build is construction, not a render function"),
            copy("", "Do not expect self fields or a bare Cage read to refresh the interface. Store configuration in self, create owned reactive state in build, and install bindings for every property that must change."),
        ))),
        Box::new(heading(2, "docs-heading", "Activate is a semantic Event")),
        Box::new(copy(
            "docs-paragraph",
            ".on(activate, handler) subscribes to EventKind::Activate. The callback receives a typed glory::render::Event, re-exported by the prelude. Event::Activate represents a control activation request without making the shared widget depend on a browser MouseEvent or keyboard event. The host handles native input and queues the semantic event; Glory routes it to the registered handler.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "The matches! check in Counter is explicit about the payload, although a handler registered only for activate will normally receive Event::Activate. Other controls use other event kinds and payloads: value_changed carries text and a revision, while checked_changed carries a boolean. Do not simulate input by directly invoking DOM events from shared component logic; drive application state or use semantic events in tests.",
        )),
        Box::new(heading(2, "docs-heading", "fill accepts IntoChildren")),
        Box::new(copy(
            "docs-paragraph",
            "View, Group, Button, and Link builders accept children through fill. The IntoChildren implementations cover one Widget, tuples of heterogeneous widgets, Vec<W>, and () for no children. A tuple is the usual choice for a fixed layout. Use Vec<Box<dyn Widget>> when a single heterogeneous vector is more convenient, and use each or list when children represent a reactive collection.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "A reusable wrapper can accept impl IntoChildren in its constructor and store the resulting glory::ui::Children. Children is a consumed mount input, not a cloneable template. Because build receives &mut self and runs once, replace it with an empty child list and pass the original value to fill. fill then mounts each child beneath the new semantic parent.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "panel.rs", CHILDREN, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "Panel::new((heading, body, Counter)) works because the three-element tuple implements IntoChildren. Calling fill more than once extends the builder's stored child list; it does not replace earlier children. This differs from class and style, which configure one replaceable property slot.",
        )),
        Box::new(heading(2, "docs-heading", "class and style replace their previous values")),
        Box::new(copy(
            "docs-paragraph",
            "Calling .class(...) twice replaces the complete whitespace-separated class list on that builder; it does not append. Similarly, .style(...) replaces the complete typed Style value and removes any earlier binding for that property slot. This rule applies whether the new value is static or reactive. CSS cascade rules can still combine styles from external stylesheets, but two builder calls are not a merge API.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "Construct one Style with chained field methods when you want to combine fields. Be especially careful with row() and col(): their initial flex display and direction are just a Style value. Supplying a replacement style without those fields removes the convenience defaults. Start from view(), or include display and flex_direction explicitly in the replacement.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "overrides.rs", OVERRIDES, "docs-code")),
        Box::new(heading(2, "docs-heading", "Children and Owner are linked, not identical")),
        Box::new(copy(
            "docs-paragraph",
            "Semantic parentage and reactive ownership are related but distinct. The Panel's View is the semantic parent of its children. Each child passed through fill is mounted in a fresh child Scope, and the constructing scope retains that mount. The child Owner therefore follows the Panel's lifetime even though the child can have its own state, subscriptions, tasks, and descendants.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "Removing a mounted branch disposes its Owner and descendants, invalidates their generational handles, removes listeners and semantic nodes, and cancels owned queued work. Capturing a child's Cage in a parent callback does not invert that relationship or keep the child alive. Pass application state down deliberately from a longer-lived Scope when it must outlive a branch.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "You normally do not need to call scope.child() yourself: fill, show_in, mount_with, and control-flow controllers establish and retain the required child scopes. If a custom controller creates a child UI Scope directly, it must retain that Scope or its Mounted guard for the entire mounted lifetime. Dropping the guard disposes the subtree.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "cleanup.rs", CLEANUP, "docs-code")),
        Box::new(heading(2, "docs-heading", "Choose a trait, closure, or plain function")),
        Box::new(copy(
            "docs-paragraph",
            "Implement Widget when the component has a reusable name, configuration, accepted children, or trait-based composition. Counter and Panel are good examples. Use widget(|scope| ...) when a local component needs owned state but does not justify a named struct. The closure must implement FnMut(&Scope) -> Result + 'static and is still built only once per mount.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "save_button.rs", CLOSURE, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "A plain fn card(...) -> impl Widget is enough for stateless composition or for returning another widget configured with arguments. If that function needs a Scope to allocate local Cages, return widget(...) rather than trying to read state during ordinary function execution. None of these choices changes the update model: dynamic properties require bindings, and structural changes require control flow.",
        )),
        Box::new(link("Review the retained tree and update model  →", Route::Architecture, "text-link")),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "A mount checklist"),
            copy("", "Allocate state from the build Scope; finish the root with show_in(scope); consume stored children exactly once; use typed Event handlers; and let fill, show_in, or an explicit Mounted guard retain every child lifetime."),
        ))),
        Box::new(doc_nav(Some(Route::Widgets), Some(Route::Styling))),
    ];
    docs_layout(Route::CustomWidgets, view().fill(content))
}
