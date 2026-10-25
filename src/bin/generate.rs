use glory_website::{Route, app};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

fn document(route: Route, html: &str) -> String {
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<meta name=\"glory-render\" content=\"ssr\">\n<meta name=\"description\" content=\"{}\">\n<meta property=\"og:title\" content=\"{}\">\n<meta property=\"og:description\" content=\"{}\">\n<meta property=\"og:type\" content=\"website\">\n<meta name=\"theme-color\" content=\"#111225\">\n<title>{}</title>\n<link rel=\"icon\" href=\"/assets/glory.ico\">\n<link rel=\"icon\" type=\"image/svg+xml\" href=\"/assets/glory.svg\">\n<style data-gly-base=\"v1\">{}</style>\n<link rel=\"stylesheet\" href=\"/sapid.css\">\n<link rel=\"stylesheet\" href=\"/site.css\">\n</head>\n<body>\n<div id=\"app\">{}</div>\n<script type=\"module\">import init from '/glory_website.js'; await init(); window.gloryReady = true;</script>\n</body>\n</html>\n",
        route.description(),
        route.title(),
        route.description(),
        route.title(),
        glory::html::BASE_CSS,
        html,
    )
}

fn output_path(directory: &Path, route: Route) -> PathBuf {
    match route {
        Route::Home => directory.join("index.html"),
        Route::NotFound => directory.join("404.html"),
        _ => directory.join(route.path().trim_matches('/')).join("index.html"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let directory = PathBuf::from(std::env::args().nth(1).ok_or("Expected an output directory")?);
    if !directory.is_absolute() {
        return Err("The output directory must be absolute".into());
    }
    fs::create_dir_all(&directory)?;
    for route in Route::ALL.into_iter().chain([Route::NotFound]) {
        let snapshot = glory::ssr::snapshot(app(route))?;
        let path = output_path(&directory, route);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, document(route, &snapshot.html))?;
        println!("Generated {}", path.display());
    }
    Ok(())
}
