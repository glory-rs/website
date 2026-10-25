use crate::{AppState, Route};
use std::cell::RefCell;
use wasm_bindgen::{JsValue, prelude::*};

thread_local! {
    static STATE: RefCell<Option<AppState>> = const { RefCell::new(None) };
    static RUNNING: RefCell<Option<Running>> = const { RefCell::new(None) };
}

struct Running {
    _app: glory::web::WebApp,
    _popstate: Closure<dyn FnMut(web_sys::PopStateEvent)>,
}

fn window() -> Result<web_sys::Window, String> {
    web_sys::window().ok_or_else(|| "Missing browser window".to_owned())
}

fn route(window: &web_sys::Window) -> Result<Route, String> {
    let path = window.location().pathname().map_err(|_| "Missing URL path")?;
    Ok(Route::parse(&path))
}

fn present(route: Route) -> Result<(), String> {
    let window = window()?;
    let document = window.document().ok_or("Missing browser document")?;
    document.set_title(route.title());
    window.scroll_to_with_x_and_y(0.0, 0.0);
    Ok(())
}

pub(crate) fn register(state: AppState) {
    STATE.with(|slot| *slot.borrow_mut() = Some(state));
}

pub fn navigate(href: &str) -> Result<(), String> {
    let window = window()?;
    let current = window.location().href().map_err(|_| "Missing current URL")?;
    let url = web_sys::Url::new_with_base(href, &current).map_err(|_| "Invalid link")?;
    if url.origin() != window.location().origin().map_err(|_| "Missing URL origin")? {
        return Err("External links use native browser navigation".to_owned());
    }
    let next = Route::parse(&url.pathname());
    window
        .history()
        .map_err(|_| "Missing browser history")?
        .push_state_with_url(&JsValue::NULL, "", Some(&url.href()))
        .map_err(|_| "Could not update browser history")?;
    let state = STATE.with(|slot| slot.borrow().as_ref().copied());
    state.ok_or("Glory app is not mounted")?.route.revise(|route| *route = next);
    present(next)
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = window().map_err(JsValue::from)?;
    let document = window.document().ok_or_else(|| JsValue::from_str("Missing document"))?;
    let root = document
        .get_element_by_id("app")
        .ok_or_else(|| JsValue::from_str("Missing #app mount element"))?;
    let initial = route(&window).map_err(JsValue::from)?;
    let ssr = document
        .query_selector("meta[name=\"glory-render\"]")
        .map_err(|_| JsValue::from_str("Invalid render metadata"))?
        .and_then(|meta| meta.get_attribute("content"))
        .as_deref()
        == Some("ssr");
    let app = if ssr {
        glory::web::hydrate_with(root, move |_| crate::app(initial))
            .map_err(|error| JsValue::from_str(&error.to_string()))?
    } else {
        glory::web::launch_with(root, move |_| crate::app(initial))
            .map_err(|error| JsValue::from_str(&error.to_string()))?
    };
    let callback_window = window.clone();
    let popstate = Closure::<dyn FnMut(web_sys::PopStateEvent)>::new(move |_| {
        if let Ok(next) = route(&callback_window) {
            STATE.with(|slot| {
                if let Some(state) = slot.borrow().as_ref() {
                    state.route.revise(|route| *route = next);
                }
            });
            let _ = present(next);
        }
    });
    window
        .add_event_listener_with_callback("popstate", popstate.as_ref().unchecked_ref())
        .map_err(|_| JsValue::from_str("Could not listen for browser navigation"))?;
    RUNNING.with(|slot| *slot.borrow_mut() = Some(Running { _app: app, _popstate: popstate }));
    Ok(())
}
