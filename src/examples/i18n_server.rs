use super::i18n_shared::{language_demo, locale_for_path};

pub fn render_page(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let initial = locale_for_path(path);
    let snapshot = glory::ssr::snapshot(language_demo(initial.clone()))?;
    Ok(format!(
        r#"<!doctype html>
<html lang="{initial}"><head><meta charset="utf-8">
<title>Glory i18n demo</title>
<style data-gly-base="v1">{}</style>
</head><body><main id="app">{}</main>
<script type="module">import init from '/app.js'; await init();</script>
</body></html>"#,
        glory::html::BASE_CSS,
        snapshot.html
    ))
}
