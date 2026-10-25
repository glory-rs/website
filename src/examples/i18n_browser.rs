use super::i18n_shared::{language_demo, locale_for_path};
use std::cell::RefCell;
use wasm_bindgen::JsValue;

thread_local! {
    static APP: RefCell<Option<glory::web::WebApp>> = const { RefCell::new(None) };
}

// Call this from your app's #[wasm_bindgen(start)] entry.
pub fn start_demo() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Missing window"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("Missing document"))?;
    let root =
        document.get_element_by_id("app").ok_or_else(|| JsValue::from_str("Missing mount"))?;
    // Use the same URL policy before hydration; do not override it with navigator.language.
    let initial = locale_for_path(&window.location().pathname()?);
    let app = glory::web::hydrate_with(root, move |_| language_demo(initial))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    APP.with(|slot| *slot.borrow_mut() = Some(app));
    Ok(())
}
