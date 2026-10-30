use crate::{Locale, Route, SiteRoute, metadata};
use glory::routing::{Bootstrap, Location, RouterHandle};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
    static RUNNING: RefCell<Option<glory::web::routing::RoutingApp>> = const { RefCell::new(None) };
}

fn window() -> Result<web_sys::Window, String> {
    web_sys::window().ok_or_else(|| "Missing browser window".to_owned())
}

fn present(route: SiteRoute) -> Result<(), String> {
    let window = window()?;
    let document = window.document().ok_or("Missing browser document")?;
    document.set_title(&route.title());
    if let Some(html) = document.document_element() {
        html.set_attribute("lang", route.locale.tag())
            .map_err(|_| "Could not set document language")?;
    }
    let set = |selector: &str, name: &str, value: &str| -> Result<(), String> {
        document
            .query_selector(selector)
            .map_err(|_| "Invalid metadata selector")?
            .ok_or_else(|| format!("Missing metadata {selector}"))?
            .set_attribute(name, value)
            .map_err(|_| "Could not update metadata".to_owned())
    };
    set("meta[name=description]", "content", &route.description())?;
    set("meta[property='og:title']", "content", &route.title())?;
    set(
        "meta[property='og:description']",
        "content",
        &route.description(),
    )?;
    set(
        "meta[property='og:url']",
        "content",
        &metadata::canonical(route),
    )?;
    set("link[rel=canonical]", "href", &metadata::canonical(route))?;
    // These belong to the outer document, outside Glory's hydrated root.
    let nodes = document
        .query_selector_all("link[rel=alternate][hreflang], meta[name=robots]")
        .map_err(|_| "Invalid metadata selector")?;
    for index in 0..nodes.length() {
        if let Some(node) = nodes.item(index)
            && let Some(parent) = node.parent_node()
        {
            let _ = parent.remove_child(&node);
        }
    }
    let head = document
        .query_selector("head")
        .map_err(|_| "Missing head")?
        .ok_or("Missing head")?;
    if route.page == Route::NotFound {
        let meta = document
            .create_element("meta")
            .map_err(|_| "Could not create metadata")?;
        meta.set_attribute("name", "robots")
            .map_err(|_| "Could not set metadata")?;
        meta.set_attribute("content", "noindex")
            .map_err(|_| "Could not set metadata")?;
        head.append_child(&meta)
            .map_err(|_| "Could not append metadata")?;
    } else {
        for (tag, locale) in [
            ("en", Locale::En),
            ("zh-CN", Locale::ZhCn),
            ("x-default", Locale::En),
        ] {
            let link = document
                .create_element("link")
                .map_err(|_| "Could not create metadata")?;
            for (key, value) in [
                ("rel", "alternate"),
                ("hreflang", tag),
                (
                    "href",
                    &metadata::canonical(SiteRoute::new(locale, route.page)),
                ),
            ] {
                link.set_attribute(key, value)
                    .map_err(|_| "Could not set metadata")?;
            }
            head.append_child(&link)
                .map_err(|_| "Could not append metadata")?;
        }
    }
    Ok(())
}

pub fn navigate(href: &str) -> Result<(), String> {
    let window = window()?;
    let current = window
        .location()
        .href()
        .map_err(|_| "Missing current URL")?;
    let url = web_sys::Url::new_with_base(href, &current).map_err(|_| "Invalid link")?;
    if url.origin()
        != window
            .location()
            .origin()
            .map_err(|_| "Missing URL origin")?
    {
        return Err("External links use native browser navigation".to_owned());
    }
    let target = Location::parse(&format!("{}{}{}", url.pathname(), url.search(), url.hash()))
        .map_err(|error| error.to_string())?;
    let router = RUNNING
        .with(|slot| slot.borrow().as_ref().map(|app| app.router()))
        .ok_or("Glory app is not mounted")?;
    router.push(target).map_err(|error| error.to_string())?;
    Ok(())
}

fn report(error: impl std::fmt::Display) {
    web_sys::console::error_1(&error.to_string().into());
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = window().map_err(JsValue::from)?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("Missing document"))?;
    let root = document
        .get_element_by_id("app")
        .ok_or_else(|| JsValue::from_str("Missing #app mount element"))?;
    let definitions = crate::routing::routes();
    let build = move |scope: &glory::Scope, router: RouterHandle| {
        let observed = router.clone();
        scope.effect(move || {
            if let Ok(snapshot) = observed.snapshot() {
                let site = crate::routing::site_route(snapshot.location(), &snapshot.plan);
                if let Err(error) = present(site) {
                    report(error);
                }
            }
        });
        crate::routing::view(router)
    };
    let app = if let Some(seed) = document.get_element_by_id("glory-route-seed") {
        let seed = Bootstrap::from_payload(&seed.text_content().unwrap_or_default())
            .map_err(|error| JsValue::from_str(&error.to_string()))?;
        glory::web::routing::hydrate_with(
            root,
            definitions,
            &seed,
            crate::routing::SCHEMA,
            build,
            report,
        )
    } else {
        glory::web::routing::launch_with(root, definitions, build, report)
    }
    .map_err(|error| JsValue::from_str(&error.to_string()))?;
    RUNNING.with(|slot| *slot.borrow_mut() = Some(app));
    Ok(())
}
