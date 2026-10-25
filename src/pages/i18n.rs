use crate::{Locale, Route, components::link, components::*, examples::i18n_shared::language_demo};
use glory::prelude::*;
use sapid::{CodeLanguage, Theme};

const INSTALL: &str = r#"[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
glory = { git = "https://github.com/glory-rs/glory", rev = "0a46e2bd2f301807571b67f4a91776402eda8aeb", default-features = false, features = ["i18n", "ssr", "hydrate"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "=0.2.129"
web-sys = { version = "=0.3.106", features = ["Window", "Document", "Element", "Location"] }"#;

pub fn i18n(locale: Locale) -> impl Widget {
    let prose = |key| copy("docs-paragraph", locale.text(key));
    let title = |key| heading(2, "docs-heading", locale.text(key));
    let code = |name, source, language| {
        source_panel(locale, Theme::blue_dark(), name, source, "docs-code", language)
    };
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", locale.text("i18n-kicker"))),
        Box::new(heading(1, "docs-title", locale.text("i18n-title"))),
        Box::new(copy("docs-lead", locale.text("i18n-lead"))),
        Box::new(title("i18n-install-title")),
        Box::new(prose("i18n-install-body")),
        Box::new(code("Cargo.toml", INSTALL, CodeLanguage::Plain)),
        Box::new(code("src/lib.rs", "pub mod examples;", CodeLanguage::Rust)),
        Box::new(code(
            "src/examples/mod.rs",
            include_str!("../examples/mod.rs"),
            CodeLanguage::Rust,
        )),
        Box::new(title("i18n-resources-title")),
        Box::new(prose("i18n-resources-body")),
        Box::new(code(
            "locales/demo.en.ftl",
            include_str!("../../locales/demo.en.ftl"),
            CodeLanguage::Plain,
        )),
        Box::new(code(
            "locales/demo.zh-CN.ftl",
            include_str!("../../locales/demo.zh-CN.ftl"),
            CodeLanguage::Plain,
        )),
        Box::new(prose("i18n-plural-body")),
        Box::new(title("i18n-catalog-title")),
        Box::new(prose("i18n-catalog-body")),
        Box::new(code(
            "src/examples/i18n_catalog.rs",
            include_str!("../examples/i18n_catalog.rs"),
            CodeLanguage::Rust,
        )),
        Box::new(prose("i18n-parameters-body")),
        Box::new(title("i18n-switch-title")),
        Box::new(prose("i18n-switch-body")),
        Box::new(code(
            "src/examples/i18n_shared.rs",
            include_str!("../examples/i18n_shared.rs"),
            CodeLanguage::Rust,
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("i18n-owner-title")),
            copy("", locale.text("i18n-owner-body")),
        ))),
        Box::new(title("i18n-demo-title")),
        Box::new(prose("i18n-demo-body")),
        Box::new(
            view()
                .class("docs-playground")
                .fill(language_demo(locale.tag().parse().expect("static language"))),
        ),
        Box::new(title("i18n-ssr-title")),
        Box::new(prose("i18n-ssr-body")),
        Box::new(code(
            "src/examples/i18n_server.rs",
            include_str!("../examples/i18n_server.rs"),
            CodeLanguage::Rust,
        )),
        Box::new(title("i18n-hydrate-title")),
        Box::new(prose("i18n-hydrate-body")),
        Box::new(code(
            "src/examples/i18n_browser.rs",
            include_str!("../examples/i18n_browser.rs"),
            CodeLanguage::Rust,
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("i18n-hydrate-warning-title")),
            copy("", locale.text("i18n-hydrate-warning-body")),
        ))),
        Box::new(title("i18n-policy-title")),
        Box::new(prose("i18n-errors-body")),
        Box::new(prose("i18n-validation-body")),
        Box::new(prose("i18n-text-body")),
        Box::new(prose("i18n-url-body")),
        Box::new(link(locale, locale.text("i18n-ssr-link"), Route::Ssr, "text-link")),
        Box::new(doc_nav(locale, Some(Route::Styling), Some(Route::Web))),
    ];
    docs_layout(locale, Route::I18n, view().fill(content))
}
