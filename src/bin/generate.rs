use glory_website::{Locale, Route, SiteRoute, app, metadata};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

fn output_path(directory: &Path, route: SiteRoute) -> PathBuf {
    let path = route.path();
    if route.page == Route::NotFound {
        directory.join(path.trim_start_matches('/'))
    } else {
        directory.join(path.trim_matches('/')).join("index.html")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let directory = PathBuf::from(std::env::args().nth(1).ok_or("Expected an output directory")?);
    let assets = std::env::args().nth(2).ok_or("Expected an immutable asset directory")?;
    if !assets.starts_with("assets-")
        || assets.len() != 27
        || !assets[7..].bytes().all(|b| b.is_ascii_hexdigit())
    {
        return Err("Invalid immutable asset directory".into());
    }
    let assets = format!("/{assets}");
    if !directory.is_absolute() {
        return Err("The output directory must be absolute".into());
    }
    fs::create_dir_all(&directory)?;
    for locale in Locale::ALL {
        for page in Route::ALL.into_iter().chain([Route::NotFound]) {
            let route = SiteRoute::new(locale, page);
            let snapshot = glory::ssr::snapshot(app(route))?;
            let path = output_path(&directory, route);
            fs::create_dir_all(path.parent().ok_or("Expected parent directory")?)?;
            fs::write(&path, metadata::document_with_assets(route, &snapshot.html, &assets))?;
            println!("Generated {}", path.display());
        }
    }
    fs::write(directory.join("sitemap.xml"), metadata::sitemap())?;
    fs::write(
        directory.join("robots.txt"),
        format!("User-agent: *\nAllow: /\nSitemap: {}/sitemap.xml\n", metadata::ORIGIN),
    )?;
    Ok(())
}
