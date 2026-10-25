use crate::{Locale, Route, SiteRoute};

pub const ORIGIN: &str = "https://glory.taidge.org";

pub fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn canonical(route: SiteRoute) -> String {
    format!("{ORIGIN}{}", route.path())
}

pub fn head(route: SiteRoute) -> String {
    let title = escape(&route.title());
    let description = escape(&route.description());
    let canonical = canonical(route);
    let mut result = format!(
        "<title>{title}</title>\n<meta name=\"description\" content=\"{description}\">\n<meta property=\"og:title\" content=\"{title}\">\n<meta property=\"og:description\" content=\"{description}\">\n<meta property=\"og:type\" content=\"website\">\n<meta property=\"og:url\" content=\"{canonical}\">\n<link rel=\"canonical\" href=\"{canonical}\">\n"
    );
    if route.page == Route::NotFound {
        result.push_str("<meta name=\"robots\" content=\"noindex\">\n");
    } else {
        for locale in Locale::ALL {
            result.push_str(&format!(
                "<link rel=\"alternate\" hreflang=\"{}\" href=\"{}\">\n",
                locale.tag(),
                self::canonical(SiteRoute::new(locale, route.page))
            ));
        }
        result.push_str(&format!(
            "<link rel=\"alternate\" hreflang=\"x-default\" href=\"{}\">\n",
            self::canonical(SiteRoute::new(Locale::En, route.page))
        ));
    }
    result
}

pub fn document(route: SiteRoute, html: &str) -> String {
    document_with_assets(route, html, "")
}

pub fn document_with_assets(route: SiteRoute, html: &str, assets: &str) -> String {
    let icon = crate::assets::media("glory.ico");
    let logo = crate::assets::media("glory.svg");
    format!(
        r##"<!doctype html>
<html lang="{}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="glory-render" content="ssr">
{}
<meta name="theme-color" content="#111225">
<link rel="icon" href="{icon}">
<link rel="icon" type="image/svg+xml" href="{logo}">
<style data-gly-base="v1">{}</style>
<link rel="stylesheet" href="{assets}/sapid.css">
<link rel="stylesheet" href="{assets}/site.css">
</head>
<body>
<div id="app">{html}</div>
<script type="module">import init from '{assets}/glory_website.js'; await init(); window.gloryReady = true;</script>
</body>
</html>
"##,
        route.locale.tag(),
        head(route),
        glory::html::BASE_CSS
    )
}

pub fn sitemap() -> String {
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" xmlns:xhtml=\"http://www.w3.org/1999/xhtml\">\n",
    );
    for page in Route::ALL {
        for locale in Locale::ALL {
            xml.push_str(&format!("<url><loc>{}</loc>", canonical(SiteRoute::new(locale, page))));
            for alternate in Locale::ALL {
                xml.push_str(&format!(
                    "<xhtml:link rel=\"alternate\" hreflang=\"{}\" href=\"{}\"/>",
                    alternate.tag(),
                    canonical(SiteRoute::new(alternate, page))
                ));
            }
            xml.push_str(&format!(
                "<xhtml:link rel=\"alternate\" hreflang=\"x-default\" href=\"{}\"/></url>\n",
                canonical(SiteRoute::new(Locale::En, page))
            ));
        }
    }
    xml.push_str("</urlset>\n");
    xml
}
