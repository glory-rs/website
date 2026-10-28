use crate::Locale;
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

pub fn ssr(locale: Locale) -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", locale.text("ssr-server-rendering-adoption"))),
        Box::new(heading(1, "docs-title", locale.text("ssr-ssr-and-hydration"))),
        Box::new(copy(
            "docs-lead",
            locale.text("ssr-render-useful-html-on-the-server-then-let-the"),
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("display-widget-guide-title")),
            copy("", locale.text("display-widget-guide-body")),
            crate::components::link(
                locale,
                locale.text("display-home-link"),
                Route::Display,
                "text-link",
            ),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("ssr-start-with-ordinary-html"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("ssr-use-ssr-render-for-a-fragment-and-ssr-render"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "static_html.rs", PLAIN_HTML, "docs-code")),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("ssr-plain-html-is-not-hydratable")),
            copy("", locale.text("ssr-choose-render-or-render-document-for-static-pages-do")),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("ssr-create-an-adoptable-snapshot"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("ssr-snapshot-and-snapshot-with-wrap-the-mapper-output-in"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "shared.rs", SHARED_COUNTER, "docs-code")),
        Box::new(code_panel(locale, Theme::blue_dark(), "server.rs", SNAPSHOT_SERVER, "docs-code")),
        Box::new(heading(2, "docs-heading", locale.text("ssr-hydrate-the-same-application"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("ssr-in-the-wasm32-browser-host-call-web-hydrate-with"),
        )),
        Box::new(code_panel(
            locale,
            Theme::blue_dark(),
            "browser.rs",
            HYDRATE_BROWSER,
            "docs-code",
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("ssr-synchronous-ssr-does-not-await")),
            copy(
                "",
                locale.text("ssr-render-snapshot-and-their-with-forms-construct-and-serialize"),
            ),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("ssr-transfer-resources-explicitly"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("ssr-the-optional-resources-feature-transfers-only-live-named-terminal"),
        )),
        Box::new(code_panel(
            locale,
            Theme::blue_dark(),
            "profile.rs",
            RESOURCE_FACTORY,
            "docs-code",
        )),
        Box::new(code_panel(
            locale,
            Theme::blue_dark(),
            "resource_server.rs",
            RESOURCE_SERVER,
            "docs-code",
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("ssr-resource-script-creates-an-escaped-inert-application-json-block"),
        )),
        Box::new(code_panel(
            locale,
            Theme::blue_dark(),
            "resource_browser.rs",
            RESOURCE_BROWSER,
            "docs-code",
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("ssr-one-context-per-request")),
            copy(
                "",
                locale.text("ssr-never-share-a-resources-context-between-requests-or-applications"),
            ),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("ssr-stream-shell-patch-and-end"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("ssr-with-the-native-stream-feature-ssr-stream-with-returns"),
        )),
        Box::new(code_panel(
            locale,
            Theme::blue_dark(),
            "stream_server.rs",
            STREAM_SERVER,
            "docs-code",
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("ssr-backpressure-is-the-polling-contract-request-the-next-frame"),
        )),
        Box::new(code_panel(
            locale,
            Theme::blue_dark(),
            "stream_browser.rs",
            STREAM_BROWSER,
            "docs-code",
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("ssr-wait-for-the-final-state")),
            copy("", locale.text("ssr-a-streaming-shell-uses-temporary-stream-markers-and-is")),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("ssr-choose-the-rendering-path"))),
        Box::new(view().class("docs-choice-table").fill((
            view().class("docs-choice-row docs-choice-header").fill((
                copy("", locale.text("ssr-need")),
                copy("", locale.text("ssr-server-api")),
                copy("", locale.text("ssr-browser-outcome")),
            )),
            view().class("docs-choice-row").fill((
                copy("docs-choice-label", locale.text("ssr-static-content")),
                copy("", locale.text("ssr-render-render-document")),
                copy("", locale.text("ssr-plain-html-no-adoption")),
            )),
            view().class("docs-choice-row").fill((
                copy("docs-choice-label", locale.text("ssr-interactive-initial-tree")),
                copy("", locale.text("ssr-snapshot-snapshot-with")),
                copy("", locale.text("ssr-hydrate-hydrate-with")),
            )),
            view().class("docs-choice-row").fill((
                copy("docs-choice-label", locale.text("ssr-prefetched-named-data")),
                copy("", locale.text("ssr-snapshot-resources-with-resource-script")),
                copy("", "hydrate_resources_with"),
            )),
            view().class("docs-choice-row").fill((
                copy("docs-choice-label", locale.text("ssr-progressive-async-work")),
                copy("", locale.text("ssr-stream-with-bootstrap")),
                copy("", locale.text("ssr-hydrate-only-after-end-glory-stream-ready")),
            )),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("ssr-common-pitfalls"))),
        Box::new(view().class("docs-pitfalls").fill((
            view().class("docs-pitfall").fill((
                heading(3, "docs-card-title", locale.text("ssr-rebuilding-instead-of-adopting")),
                copy(
                    "docs-paragraph",
                    locale.text("ssr-launch-requires-an-empty-mount-and-creates-a-new"),
                ),
            )),
            view().class("docs-pitfall").fill((
                heading(3, "docs-card-title", locale.text("ssr-changing-initial-inputs")),
                copy(
                    "docs-paragraph",
                    locale.text("ssr-a-different-factory-initial-cage-value-resource-name-schema"),
                ),
            )),
            view().class("docs-pitfall").fill((
                heading(3, "docs-card-title", locale.text("ssr-dropping-webapp")),
                copy(
                    "docs-paragraph",
                    locale.text("ssr-the-returned-webapp-is-a-lifetime-guard-not-a"),
                ),
            )),
            view().class("docs-pitfall").fill((
                heading(3, "docs-card-title", locale.text("ssr-expecting-implicit-async-ssr")),
                copy(
                    "docs-paragraph",
                    locale.text("ssr-a-synchronous-snapshot-never-waits-for-a-future-use"),
                ),
            )),
            view().class("docs-pitfall").fill((
                heading(3, "docs-card-title", locale.text("ssr-nesting-the-data-block")),
                copy(
                    "docs-paragraph",
                    locale.text("ssr-keep-resource-script-output-outside-the-mount-hydration-owns"),
                ),
            )),
        ))),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("ssr-bring-your-own-production-http-server")),
            copy(
                "",
                locale.text("ssr-glory-supplies-rendering-frames-and-browser-bootstrap-assets-not"),
            ),
        ))),
        Box::new(doc_nav(locale, Some(Route::Web), Some(Route::Platforms))),
    ];
    docs_layout(locale, Route::Ssr, view().fill(content))
}
