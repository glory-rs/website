use crate::Locale;
use crate::{Route, components::link, components::*};
use glory::prelude::*;
use sapid::Theme;

const CARGO: &str = r#"[package]
name = "my_glory_app"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
glory = { version = "=0.6.0", default-features = false, features = ["web"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "=0.2.129"
web-sys = { version = "=0.3.106", features = ["Window", "Document", "Element"] }"#;

const START: &str = r#"use glory::prelude::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
    static APP: RefCell<Option<glory::web::WebApp>> = const { RefCell::new(None) };
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let root = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id("app"))
        .ok_or_else(|| JsValue::from_str("missing #app mount element"))?;

    let app = glory::web::launch(root, crate::Counter)
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    APP.with(|slot| *slot.borrow_mut() = Some(app));
    Ok(())
}"#;

const LAUNCH_WITH: &str = r#"let app = glory::web::launch_with(root, |scope| {
    let count = scope.cage(0_u64);
    col().fill((
        text().content(count.map(|count| format!("Count: {count}"))),
        button()
            .label("Add one")
            .on(activate, move |_| count.revise(|count| *count += 1)),
    ))
})?;
APP.with(|slot| *slot.borrow_mut() = Some(app));"#;

const HTML: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Glory app</title>
  </head>
  <body>
    <main id="app"></main>
    <script type="module">
      import init from "./my_glory_app.js";
      await init();
    </script>
  </body>
</html>"#;

const BUILD: &str = r#"rustup target add wasm32-unknown-unknown
cargo build --lib --target wasm32-unknown-unknown

# Use a wasm-bindgen-cli version matching the locked wasm-bindgen crate.
wasm-bindgen \
  --target web \
  --out-dir dist \
  target/wasm32-unknown-unknown/debug/my_glory_app.wasm

# Copy index.html into dist, then serve the directory over HTTP.
python -m http.server 8080 --bind 127.0.0.1 --directory dist"#;

const SHARED_APP: &str = r#"// Shared by the native SSR binary and the wasm32 browser crate.
use glory::prelude::*;

pub fn app() -> impl Widget {
    crate::Counter
}"#;

const SSR: &str = r#"fn render_page() -> Result<String> {
    // snapshot(), rather than render(), includes hydration version markers.
    let snapshot = glory::ssr::snapshot(app())?;
    Ok(format!(
        "<!doctype html><html lang=\"en\"><head>\
         <meta charset=\"utf-8\"><title>Counter</title>\
         <style data-gly-base=\"v1\">{}</style></head><body>\
         <main id=\"app\">{}</main>\
         <script type=\"module\">import init from '/my_glory_app.js'; await init();</script>\
         </body></html>",
        glory::html::BASE_CSS,
        snapshot.html,
    ))
}"#;

const HYDRATE: &str = r#"// Enable Glory's hydrate feature for this browser build.
let app = glory::web::hydrate(root, app())
    .map_err(|error| JsValue::from_str(&error.to_string()))?;

// Hydration also returns the WebApp guard that must be retained.
APP.with(|slot| *slot.borrow_mut() = Some(app));"#;

pub fn web(locale: Locale) -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", locale.text("architecture-the-glory-guide"))),
        Box::new(heading(1, "docs-title", locale.text("web-web-applications"))),
        Box::new(copy(
            "docs-lead",
            locale.text("web-a-glory-browser-application-is-a-wasm32-rust-library"),
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
        Box::new(heading(2, "docs-heading", locale.text("web-select-the-wasm32-web-feature"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("web-enable-glory-s-web-feature-for-a-dom-application"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "Cargo.toml", CARGO, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            locale.text("web-the-cdylib-output-is-the-wasm-library-loaded-by"),
        )),
        Box::new(heading(2, "docs-heading", locale.text("web-find-the-root-and-call-launch"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("web-launch-root-widget-mounts-an-already-constructed-widget-into"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "src/web.rs", START, "docs-code")),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("web-webapp-is-a-lifetime-guard")),
            copy("", locale.text("web-do-not-let-the-returned-webapp-fall-out-of")),
        ))),
        Box::new(heading(
            2,
            "docs-heading",
            locale.text("web-use-launch-with-for-root-scope-construction"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("web-launch-with-root-factory-accepts-fnonce-scope-w-it"),
        )),
        Box::new(code_panel(
            locale,
            Theme::blue_dark(),
            "launch_with.rs",
            LAUNCH_WITH,
            "docs-code",
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("web-both-functions-return-result-webapp-the-factory-form-is"),
        )),
        Box::new(heading(2, "docs-heading", locale.text("web-the-csr-mount-must-be-empty"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("web-launch-and-launch-with-require-an-empty-unclaimed-root"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "index.html", HTML, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            locale.text("web-the-module-imports-the-javascript-generated-by-wasm-bindgen"),
        )),
        Box::new(heading(2, "docs-heading", locale.text("web-build-bindings-and-serve-over-http"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("web-compile-the-library-to-wasm-run-wasm-bindgen-with"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "terminal", BUILD, "docs-code")),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("web-do-not-open-index-html-with-file")),
            copy("", locale.text("web-es-module-and-wasm-loading-require-a-proper-http")),
        ))),
        Box::new(heading(
            2,
            "docs-heading",
            locale.text("web-link-ssr-to-the-browser-with-hydrate"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("web-client-only-launch-creates-a-fresh-dom-hydration-is"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("web-put-the-shared-widget-factory-in-code-compiled-for"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "src/app.rs", SHARED_APP, "docs-code")),
        Box::new(heading(
            2,
            "docs-heading",
            locale.text("web-serialize-a-snapshot-not-plain-markup"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("web-glory-ssr-snapshot-app-returns-a-snapshot-whose-html"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "src/server.rs", SSR, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            locale.text("web-glory-ssr-render-is-appropriate-when-you-only-need"),
        )),
        Box::new(heading(
            2,
            "docs-heading",
            locale.text("web-adopt-the-dom-and-retain-the-same-guard"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("web-use-hydrate-root-widget-or-hydrate-with-root-factory"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "src/web.rs", HYDRATE, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            locale.text("web-the-hydrate-with-factory-has-the-same-fnonce-scope"),
        )),
        Box::new(link(
            locale,
            locale.text("web-review-getting-started-and-host-features"),
            Route::Docs,
            "text-link",
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("web-the-complete-startup-contract")),
            copy("", locale.text("web-build-for-wasm32-with-web-look-up-a-web")),
        ))),
        Box::new(doc_nav(locale, Some(Route::I18n), Some(Route::Ssr))),
    ];
    docs_layout(locale, Route::Web, view().fill(content))
}
