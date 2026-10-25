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
glory = { version = "0.3.1", default-features = false, features = ["web"] }

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

pub fn web() -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", "THE GLORY GUIDE")),
        Box::new(heading(1, "docs-title", "Web applications.")),
        Box::new(copy(
            "docs-lead",
            "A Glory browser application is a wasm32 Rust library started by wasm-bindgen. The host entry point finds an empty web_sys::Element, mounts the shared widget tree, retains the WebApp guard, and serves the generated JavaScript and WASM over HTTP.",
        )),
        Box::new(heading(2, "docs-heading", "Select the wasm32 Web feature")),
        Box::new(copy(
            "docs-paragraph",
            "Enable Glory's web feature for a DOM application. It selects the HTML mapper and browser host; the shared Counter itself still needs only glory::prelude::*. Add wasm-bindgen for the start export and web-sys with Window, Document, and Element for mount lookup. Keep the wasm-bindgen CLI used after compilation at the same version as the locked Rust crate.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "Cargo.toml", CARGO, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "The cdylib output is the WASM library loaded by the browser; rlib keeps the code available to native tests and server-side rendering. Build the browser entry with --target wasm32-unknown-unknown. The old web-csr and web-ssr feature names are not part of this API: choose web, ssr, and optionally hydrate.",
        )),
        Box::new(heading(2, "docs-heading", "Find the root and call launch")),
        Box::new(copy(
            "docs-paragraph",
            "launch(root, widget) mounts an already constructed Widget into a web_sys::Element. The complete entry point below uses the Counter from the custom-widget guide. Convert Glory's error into JsValue so module startup reports a useful failure instead of silently leaving an empty page.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "src/web.rs", START, "docs-code")),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "WebApp is a lifetime guard"),
            copy("", "Do not let the returned WebApp fall out of scope at the end of start. Dropping it releases the DOM, observers, native listeners, animation frame, and owned async tasks. Store it in application state or a thread_local guard for as long as the interface should run."),
        ))),
        Box::new(heading(2, "docs-heading", "Use launch_with for root-scope construction")),
        Box::new(copy(
            "docs-paragraph",
            "launch_with(root, factory) accepts FnOnce(&Scope) -> W. It is useful when constructing the root widget also needs to allocate Cages or other state in the root scope. The factory returns the widget; it does not call show_in itself. launch(root, widget) is the convenient equivalent when the widget value is already available.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "launch_with.rs", LAUNCH_WITH, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "Both functions return Result<WebApp>. The factory form is not a recurring render callback: it runs once during mount, exactly like Widget::build. Reactive values installed there update the retained tree through bindings after startup.",
        )),
        Box::new(heading(2, "docs-heading", "The CSR mount must be empty")),
        Box::new(copy(
            "docs-paragraph",
            "launch and launch_with require an empty, unclaimed root. They reject an element with any child nodes or an existing data-gly-app attribute. Child text nodes count too, so keep whitespace, comments, fallback content, and server markup out of a CSR mount. This guard prevents client rendering from silently deleting content that should instead be hydrated.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "index.html", HTML, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "The module imports the JavaScript generated by wasm-bindgen and awaits initialization. The #[wasm_bindgen(start)] function then runs, finds #app, and mounts Glory. You do not need to create DOM elements in Rust startup beyond looking up this root; the semantic Web renderer creates and updates the application's DOM.",
        )),
        Box::new(heading(2, "docs-heading", "Build bindings and serve over HTTP")),
        Box::new(copy(
            "docs-paragraph",
            "Compile the library to WASM, run wasm-bindgen with --target web, and place index.html beside the generated .js and _bg.wasm files. Then start an HTTP server for that output directory and open the shown localhost URL in a browser.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "terminal", BUILD, "docs-code")),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "Do not open index.html with file://"),
            copy("", "ES module and WASM loading require a proper HTTP origin and MIME handling; browser file:// security rules commonly block those requests. Serve the generated directory over HTTP even for local development. A successful native cargo build alone does not exercise this browser startup path."),
        ))),
        Box::new(heading(2, "docs-heading", "Link SSR to the browser with hydrate")),
        Box::new(copy(
            "docs-paragraph",
            "Client-only launch creates a fresh DOM. Hydration is a separate, explicit operation that adopts a versioned SSR Snapshot without replaying initial property setters or replacing the existing subtree. Enable ssr in the native server build and hydrate in the wasm32 browser build; hydrate implies web. If one dependency declaration serves both targets, features = [\"web\", \"ssr\", \"hydrate\"] selects the complete setup.",
        )),
        Box::new(copy(
            "docs-paragraph",
            "Put the shared widget factory in code compiled for both targets. The server and browser construct equivalent initial semantic trees from the same application logic. Arbitrary Rust heap state is not magically transferred by hydration; initial values must agree, and asynchronous request data needs an explicit resource or application data strategy.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "src/app.rs", SHARED_APP, "docs-code")),
        Box::new(heading(2, "docs-heading", "Serialize a Snapshot, not plain markup")),
        Box::new(copy(
            "docs-paragraph",
            "glory::ssr::snapshot(app()) returns a Snapshot whose html includes the version markers required by hydrate. Insert that fragment verbatim inside the exclusively owned #app element. Also include glory::html::BASE_CSS with the data-gly-base=\"v1\" attribute; the browser host can then recognize the base style rather than duplicate it.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "src/server.rs", SSR, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "glory::ssr::render is appropriate when you only need HTML output, but its unversioned string is not the hydration contract. The Snapshot structure, widget factory, and browser feature set must agree. Hydration validates the complete initial subtree before attaching runtime metadata; on a version or structure mismatch it reports an error instead of repairing arbitrary DOM in place.",
        )),
        Box::new(heading(2, "docs-heading", "Adopt the DOM and retain the same guard")),
        Box::new(copy(
            "docs-paragraph",
            "Use hydrate(root, widget) or hydrate_with(root, factory) only for an SSR root. Do not call launch on that root first: launch requires emptiness, while hydrate expects the versioned server fragment. The browser still constructs the widget to establish state and bindings, but adopts matching DOM instead of recreating it. Existing eligible user input is preserved through the adoption rules.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "src/web.rs", HYDRATE, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "The hydrate_with factory has the same FnOnce(&Scope) -> W shape as launch_with and is useful when the shared factory returns both a widget and handles needed by browser application state. Whichever entry point you use, retain the returned WebApp. Hydration changes how the initial DOM is acquired, not who owns the running app afterward.",
        )),
        Box::new(link("Review getting started and host features  →", Route::Docs, "text-link")),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "The complete startup contract"),
            copy("", "Build for wasm32 with web; look up a web_sys root; use launch or launch_with only on an empty CSR mount; serve modules and WASM over HTTP; retain WebApp; and choose hydrate or hydrate_with with the matching SSR Snapshot when adopting server DOM."),
        ))),
        Box::new(doc_nav(Some(Route::Styling), Some(Route::Ssr))),
    ];
    docs_layout(Route::Web, view().fill(content))
}
