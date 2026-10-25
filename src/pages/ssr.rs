use crate::{Route, components::*};
use glory::prelude::*;
use sapid::Theme;

const PLAIN_HTML: &str = r#"use glory::prelude::*;
use glory::ssr::Document;

fn article() -> impl Widget {
    col().fill((
        text().kind(TextKind::Heading(1)).content("Glory SSR"),
        text().content("Rendered by the semantic HTML mapper."),
    ))
}

// A plain HTML fragment: no versioned hydration envelope.
let fragment = glory::ssr::render(article())?;

// A complete document. BASE_CSS is included automatically.
let document = Document {
    title: "Glory SSR".into(),
    lang: Some("en".into()),
    stylesheets: vec!["/assets/app.css".into()],
    ..Default::default()
};
let html = glory::ssr::render_document(article(), &document)?;"#;

const SHARED_COUNTER: &str = r#"use glory::prelude::*;

// Share this factory and its initial inputs across server and browser.
fn counter(scope: &Scope, start: u64) -> impl Widget + use<> {
    let count = scope.cage(start);
    col().fill((
        text().content(count.map(|value| format!("Count: {value}"))),
        button()
            .label("Increment")
            .on(activate, move |_| count.revise(|value| *value += 1)),
    ))
}"#;

const SNAPSHOT_SERVER: &str = r#"// Native request handler: create a fresh snapshot for this request.
let snapshot = glory::ssr::snapshot_with(|scope| counter(scope, 0))?;
let html = format!(
    "<!doctype html><html lang=\"en\"><head>\
     <meta charset=\"utf-8\"><title>Counter</title>\
     <style data-gly-base=\"v1\">{}</style></head><body>\
     <main id=\"app\">{}</main>\
     <script type=\"module\" src=\"/assets/bootstrap.js\"></script>\
     </body></html>",
    glory::html::BASE_CSS,
    snapshot.html,
);
// Insert snapshot.html verbatim. Do not pretty-print or sanitize the markers."#;

const HYDRATE_BROWSER: &str = r#"use glory::web::WebApp;

fn mount(root: web_sys::Element) -> Result<WebApp, glory::web::HydrationError> {
    // Same factory and same initial value as the server snapshot.
    glory::web::hydrate_with(root, |scope| counter(scope, 0))
}

let app = mount(root)?;
// Store `app` for the entire application lifetime. Dropping WebApp releases
// the DOM binding, listeners, observers, frame host, and owned tasks."#;

const RESOURCE_FACTORY: &str = r#"use glory::prelude::*;

fn profile(scope: &Scope, resources: &Resources) -> impl Widget + use<> {
    let profile = resources
        .bind(
            scope.owner(),
            "profile",       // Stable wire name.
            "profile/1",     // Explicit schema version.
            || 7_u64,        // Business key, not a global cache key.
            |id| async move { Ok::<_, String>(load_profile(id).await) },
        )
        .expect("valid resource registration");

    text().content(profile.map(|state| match state {
        ResourceState::Ready { value, .. } => value.to_string(),
        ResourceState::Failed { error, .. } => format!("Failed: {error}"),
        _ => "Loading profile…".into(),
    }))
}"#;

const RESOURCE_SERVER: &str = r#"// Synchronous SSR needs an explicitly prefetched terminal value.
let mut seeds = glory::resources::Snapshot::default();
seeds.seed(
    "profile",
    "profile/1",
    &7_u64,
    Ok::<_, &String>(&"Ada Lovelace".to_owned()),
)?;
let resources = Resources::restore(seeds)?;
let output = glory::ssr::snapshot_resources_with(&resources, |scope| {
    profile(scope, &resources)
})?;
let data = glory::ssr::resource_script("profile-data", &output.data, None)?;

let html = format!(
    "<!doctype html><html><head><style data-gly-base=\"v1\">{}</style></head>\
     <body><main id=\"app\">{}</main>{}</body></html>",
    glory::html::BASE_CSS,
    output.view.html,
    data, // The inert JSON script is outside #app.
);"#;

const RESOURCE_BROWSER: &str = r#"use glory::resources::{Limits, Resources};
use glory::web::{HydrationError, WebApp};

fn mount(
    root: web_sys::Element,
    data: web_sys::Element,
) -> Result<(WebApp, Resources), HydrationError> {
    glory::web::hydrate_resources_with(
        root,
        &data,
        Limits::default(),
        |scope, resources| profile(scope, resources),
    )
}

let (app, resources) = mount(root, data_element)?;
// Keep the WebApp alive. A matching seed suppresses only the first load;
// refresh and new business keys still invoke the loader normally."#;

const STREAM_SERVER: &str = r##"use glory::prelude::*;
use glory::ssr::stream::{Bootstrap, Config};

// One local request context; the stream owns its App and async work.
let resources = Resources::default();
let mut body = glory::ssr::stream_with(
    resources,
    Config::default(),
    |scope, resources| profile(scope, resources),
)?;
let bootstrap = Bootstrap {
    root_id: "app".into(),
    data_id: "stream-data".into(),
    script_url: "/glory-stream.js".into(), // Serve web::STREAM_SCRIPT here.
    nonce: Some(request_nonce),
};

// HTTP-host pseudocode: your adapter supplies send_head and send_chunk.
send_head(glory::html::BASE_CSS, &bootstrap.head_script()?).await?;
while let Some(frame) = body.next_frame().await {
    let frame = frame?; // Shell, then zero or more Patch frames, then End.
    send_chunk(frame.to_html(&bootstrap)?).await?;
    // Awaiting the write before requesting another frame applies backpressure.
}
// Dropping `body` early cancels its request work."##;

const STREAM_BROWSER: &str = r#"use glory::resources::{Limits, Resources};
use glory::web::{HydrationError, WebApp};

fn adopt_stream(
    root: web_sys::Element,
    data: web_sys::Element,
) -> Result<(WebApp, Resources), HydrationError> {
    glory::web::hydrate_resources_with(
        root,
        &data,
        Limits::default(),
        |scope, resources| profile(scope, resources),
    )
}

// Call adopt_stream only from the root's `glory:stream-ready` listener.
// That event follows End, final key relabeling, and resource handoff.
let (app, resources) = adopt_stream(root, stream_data)?;
// Retain `app` just as you would after ordinary hydration."#;

pub fn ssr() -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", "SERVER RENDERING & ADOPTION")),
        Box::new(heading(1, "docs-title", "SSR and hydration.")),
        Box::new(copy(
            "docs-lead",
            "Render useful HTML on the server, then let the browser adopt that exact semantic tree. Glory keeps static output, snapshots, resource transfer, and progressive streaming as explicit choices.",
        )),
        Box::new(heading(2, "docs-heading", "Start with ordinary HTML")),
        Box::new(copy(
            "docs-paragraph",
            "Use ssr::render for a fragment and ssr::render_document for a complete document with metadata, stylesheets, and BASE_CSS. These APIs produce ordinary server-rendered HTML, not a versioned hydration Snapshot. There is no hydration envelope for web::hydrate to adopt.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "static_html.rs", PLAIN_HTML, "docs-code")),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "Plain HTML is not hydratable"),
            copy("", "Choose render or render_document for static pages. Do not place their output in a mount and call hydrate: adoption requires snapshot or snapshot_with (or a completed stream)."),
        ))),
        Box::new(heading(2, "docs-heading", "Create an adoptable snapshot")),
        Box::new(copy(
            "docs-paragraph",
            "snapshot and snapshot_with wrap the mapper output in Glory's versioned fragment boundaries. Insert Snapshot::html verbatim into an otherwise empty, exclusively owned mount element. Include glory::html::BASE_CSS in the document; unlike render_document, manual snapshot assembly does not add it for you.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "shared.rs", SHARED_COUNTER, "docs-code")),
        Box::new(code_panel(Theme::blue_dark(), "server.rs", SNAPSHOT_SERVER, "docs-code")),
        Box::new(heading(2, "docs-heading", "Hydrate the same application")),
        Box::new(copy(
            "docs-paragraph",
            "In the wasm32 browser host, call web::hydrate with a widget or web::hydrate_with with a factory. The factory, structure, and initial data must match the server. Hydration validates the entire subtree before attaching runtime state; a version or structure mismatch returns HydrationError and preserves the existing DOM.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "browser.rs", HYDRATE_BROWSER, "docs-code")),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "Synchronous SSR does not await"),
            copy("", "render, snapshot, and their _with forms construct and serialize the current tree immediately. They do not block on async Resource loaders or wait for pending values. Seed known terminal data explicitly, render a pending state, or use stream_with when the server should progressively await work."),
        ))),
        Box::new(heading(2, "docs-heading", "Transfer resources explicitly")),
        Box::new(copy(
            "docs-paragraph",
            "The optional resources feature transfers only live, named terminal values. Give every binding a stable name and schema string, and make its business key explicit. For synchronous SSR, prefetch into a resource Snapshot with seed, restore a fresh Resources context, render with snapshot_resources_with, and export before App cleanup removes registrations.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "profile.rs", RESOURCE_FACTORY, "docs-code")),
        Box::new(code_panel(Theme::blue_dark(), "resource_server.rs", RESOURCE_SERVER, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "resource_script creates an escaped, inert application/json block, not executable bootstrap code. Put it outside the hydrated mount. In the browser, hydrate_resources_with reads it with explicit Limits, restores Resources, and runs the same named bindings. A matching success or error seed skips only the initial request.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "resource_browser.rs", RESOURCE_BROWSER, "docs-code")),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "One context per request"),
            copy("", "Never share a Resources context between requests or applications; clones share that context. Create a new one every time. On a failed low-level adoption attempt, restore a fresh context from the original payload for retry. hydrate_resources_with preserves the data block on failure and removes it only after success."),
        ))),
        Box::new(heading(2, "docs-heading", "Stream Shell, Patch, and End")),
        Box::new(copy(
            "docs-paragraph",
            "With the native stream feature, ssr::stream_with returns a local, pull-driven request body. Shell is available before loader polling, Patch frames carry semantic HTML edits as work completes, and End carries terminal resources plus canonical key relabeling. Serve web::STREAM_SCRIPT under your CSP and emit Bootstrap::head_script before the Shell so early input is captured.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "stream_server.rs", STREAM_SERVER, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "Backpressure is the polling contract: request the next frame only after the previous chunk has been written. Pausing demand pauses rendering and loader polling; dropping the body cancels its App and futures. Do not replace this with an eager producer that buffers the whole response.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "stream_browser.rs", STREAM_BROWSER, "docs-code")),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "Wait for the final state"),
            copy("", "A streaming Shell uses temporary stream markers and is deliberately not adoptable. Never hydrate the Shell or a mid-stream Patch. After End has been applied, the root dispatches glory:stream-ready; only then hydrate the finalized tree with the same factory and transferred resources."),
        ))),
        Box::new(heading(2, "docs-heading", "Choose the rendering path")),
        Box::new(view().class("docs-choice-table").fill((
            view().class("docs-choice-row docs-choice-header").fill((
                copy("", "Need"),
                copy("", "Server API"),
                copy("", "Browser outcome"),
            )),
            view().class("docs-choice-row").fill((
                copy("docs-choice-label", "Static content"),
                copy("", "render / render_document"),
                copy("", "Plain HTML; no adoption"),
            )),
            view().class("docs-choice-row").fill((
                copy("docs-choice-label", "Interactive initial tree"),
                copy("", "snapshot / snapshot_with"),
                copy("", "hydrate / hydrate_with"),
            )),
            view().class("docs-choice-row").fill((
                copy("docs-choice-label", "Prefetched named data"),
                copy("", "snapshot_resources_with + resource_script"),
                copy("", "hydrate_resources_with"),
            )),
            view().class("docs-choice-row").fill((
                copy("docs-choice-label", "Progressive async work"),
                copy("", "stream_with + Bootstrap"),
                copy("", "Hydrate only after End / glory:stream-ready"),
            )),
        ))),
        Box::new(heading(2, "docs-heading", "Common pitfalls")),
        Box::new(view().class("docs-pitfalls").fill((
            view().class("docs-pitfall").fill((
                heading(3, "docs-card-title", "Rebuilding instead of adopting"),
                copy("docs-paragraph", "launch requires an empty mount and creates a new tree. Use hydrate for a Snapshot; do not clear or rewrite server markup first."),
            )),
            view().class("docs-pitfall").fill((
                heading(3, "docs-card-title", "Changing initial inputs"),
                copy("docs-paragraph", "A different factory, initial Cage value, resource name, schema, or final resource state can produce a mismatch or an extra load. Share the component and seed contract."),
            )),
            view().class("docs-pitfall").fill((
                heading(3, "docs-card-title", "Dropping WebApp"),
                copy("docs-paragraph", "The returned WebApp is a lifetime guard, not a throwaway result. Store it for as long as the interface should remain mounted and interactive."),
            )),
            view().class("docs-pitfall").fill((
                heading(3, "docs-card-title", "Expecting implicit async SSR"),
                copy("docs-paragraph", "A synchronous snapshot never waits for a future. Use explicit seeds for already known values, or the pull-driven stream for progressive completion."),
            )),
            view().class("docs-pitfall").fill((
                heading(3, "docs-card-title", "Nesting the data block"),
                copy("docs-paragraph", "Keep resource_script output outside the mount. Hydration owns the mount's complete subtree; the separate JSON block has its own identity and lifetime."),
            )),
        ))),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "Bring your own production HTTP server"),
            copy("", "Glory supplies rendering, frames, and browser bootstrap assets—not a production HTTP server. Routing, TLS, concurrency, socket writes, CSP, disconnect handling, and deployment belong to your host. The semantic-web ssr_fixture and stream_fixture binaries are acceptance fixtures, not Axum or Salvo integrations and not production server implementations."),
        ))),
        Box::new(doc_nav(Some(Route::Web), Some(Route::Platforms))),
    ];
    docs_layout(Route::Ssr, view().fill(content))
}
