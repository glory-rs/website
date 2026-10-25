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

pub fn architecture() -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", "THE GLORY GUIDE")),
        Box::new(heading(1, "docs-title", "Architecture and design.")),
        Box::new(copy(
            "docs-lead",
            "Glory retains a typed semantic tree, builds each mounted widget once, and sends reactive changes directly to the affected nodes. The same application model can be interpreted by browser, server, test, and native hosts.",
        )),
        Box::new(heading(2, "docs-heading", "A retained tree, not a virtual DOM")),
        Box::new(copy(
            "docs-paragraph",
            "Mounting a widget creates semantic nodes with stable identities and typed properties. A View, Text, Button, Input, Check, Image, Link, or Group records what the interface is; it does not ask the shared UI layer to create a DOM element or a native control. The active renderer retains those nodes and translates operations for its host.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "Widget::build runs once for each mount. It creates the initial structure, allocates state in the supplied Scope, and installs event handlers and reactive bindings. A later state change does not call build again and does not manufacture a second tree to compare with the first. Builders use &mut self because mounting can consume their stored children and declarations.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "counter.rs", BUILD_ONCE, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "In this example, count.map creates a tracked derived value and text().content installs a binding for it. The button handler revises the Cage. The runtime then wakes the affected binding, which updates the existing Text node. Constructing a new Counter value would describe a new mount; it is not part of the update path.",
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "A bare build-time read is not reactive"),
            copy("", "text().content(count.get().to_string()) copies one value into a static property. Use count.map(...), another Bond, or content_fn(...) when the text must change. Reactive reads are tracked inside Bond, Binding, and Effect computations, not by an arbitrary read during build."),
        ))),
        Box::new(heading(2, "docs-heading", "From a Cage revision to a typed Patch")),
        Box::new(copy(
            "docs-paragraph",
            "Cage is the owned mutable source; it is not a diff engine. revise changes the source and schedules direct dependents. A text binding that observes the new value prepares glory::render::Patch::Text(Change::Set(new_text)) for its known node. Other properties use their own variants, such as Patch::Label, Patch::Classes, Patch::Style, Patch::Value, and Patch::Checked.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "That distinction matters for both correctness and performance. The update already knows its target and its field, so no VDOM walk is needed to discover what changed. Typed patches also let a renderer validate revisions, controlled-input state, supported properties, and host capabilities before applying an operation. Writes are scheduled work: the runtime's batching and the host's event or frame boundary determine when the binding and renderer commit run.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "This does not make every computation free. A derived computation still executes the Rust code you wrote, and a broad dependency can still cause unnecessary work. Keep derived values focused, use equality gates where they help, and use keyed each or the windowed list controller for collections rather than rebuilding an entire screen from a Cage.",
        )),
        Box::new(heading(2, "docs-heading", "Change structure with control flow")),
        Box::new(copy(
            "docs-paragraph",
            "Property bindings cannot add or remove semantic nodes. Use when for conditional branches, each for ordinary keyed collections, and list for large windowed collections. These controllers own the mount lifetime of their branches and rows. Source reads performed by structural factories are handled separately from the pure property bindings.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "structure.rs", STRUCTURE, "docs-code")),
        Box::new(heading(2, "docs-heading", "Group is logical; View has a box")),
        Box::new(copy(
            "docs-paragraph",
            "Use group() when children need a logical parent but no physical wrapper. Group has no layout box, no implicit accessibility role, and no physical geometry or scrolling target. It can still participate in logical ancestry and appropriate logical event routing. This makes it useful for returning several children without distorting a flex or grid layout.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "Use view() when you need a layout box, typed styles, classes, or a structural document role. row() and col() are View constructors with flex direction defaults, not special renderer primitives. Do not put a Group into a layout calculation and expect it to have dimensions; query a boxed descendant or use a View instead.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "groups.rs", GROUPS, "docs-code")),
        Box::new(heading(2, "docs-heading", "Owner is the lifetime boundary")),
        Box::new(copy(
            "docs-paragraph",
            "Every UI Scope wraps a core Owner and adds semantic construction context. Cages, Bonds, bindings, effects, listeners, tasks, and mounted children created through that scope belong to its ownership tree. show_in and fill retain the resulting mounts in their parent. When control flow removes a branch or an application drops a mount, disposing its Owner retires that subtree and cancels its owned work.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "A Cage is a Copy, generational handle, not an Rc-like lifetime extension. Capturing it in a callback does not keep its Owner alive; operations on a retired handle fail rather than addressing a reused value. Owner::on_cleanup registers explicit cleanup, with children cleaned before parents. Prefer scope.downgrade() when a long-lived callback needs an optional Scope reference instead of strongly retaining the UI scope.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "Ownership also explains independent component state. Two mounts of Counter execute build twice in two child scopes and allocate two distinct count Cages. They may share application state only when you deliberately capture a Cage from a longer-lived owner. Normal fill and show_in calls manage child scopes for you; manually created child UI scopes must be retained for as long as their mounted UI should live.",
        )),
        Box::new(link("Review Cage, Bond, and Lotus ownership  →", Route::Reactivity, "text-link")),
        Box::new(heading(2, "docs-heading", "One semantic model, replaceable hosts")),
        Box::new(copy(
            "docs-paragraph",
            "The shared widget describes semantics and behavior, while the host supplies presentation and execution services. The web feature interprets semantic operations through the browser DOM and browser input. The ssr feature synchronously serializes HTML or versioned Snapshot fragments; event handlers do not become executable JavaScript. The optional Blitz host interprets the same semantic operations with native CSS layout, text, painting, and desktop window integration.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "App::memory provides a reference interpreter for semantic tests. It can exercise mounts, patches, and typed events without claiming to produce browser or native pixels. At the other boundary, WebApp and the Blitz desktop Shell keep their respective hosts alive and drive input, flushing, frames, and cleanup.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "hosts.rs", HOSTS, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "Portability is a semantic and typed-style contract, not a promise of identical pixels on every engine. Check each host's supported style profile and interaction capabilities. Keep DOM lookup, HTTP serving, window creation, and other platform concerns in the host entry points so the shared widget remains reusable.",
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "The mental model"),
            copy("", "Build structure once, bind changing properties explicitly, let control flow own structural changes, and let the selected host interpret typed semantic operations. There is no hidden render-function rerun or VDOM reconciliation pass between a Cage revision and a Patch."),
        ))),
        Box::new(doc_nav(Some(Route::Docs), Some(Route::Reactivity))),
    ];
    docs_layout(Route::Architecture, view().fill(content))
}
