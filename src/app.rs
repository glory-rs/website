use crate::{Route, components, pages};
use glory::prelude::*;

#[derive(Clone, Copy)]
pub struct AppState {
    pub route: Cage<Route>,
}

pub fn request_navigation(request: sapid::AnchorRequest) {
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = crate::web::navigate(&request.href) {
        web_sys::console::error_1(&error.into());
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = request;
}

fn register(state: AppState) {
    #[cfg(target_arch = "wasm32")]
    crate::web::register(state);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = state;
}

fn page(route: Route) -> Box<dyn Widget> {
    let content: Box<dyn Widget> = match route {
        Route::Home => Box::new(pages::home()),
        Route::Docs => Box::new(pages::docs()),
        Route::Reactivity => Box::new(pages::reactivity()),
        Route::Sapid => Box::new(pages::sapid_page()),
        Route::NotFound => Box::new(pages::not_found()),
    };
    let theme = sapid::Theme::blue_dark();
    Box::new(view().class(format!("glory-site {}", theme.class())).fill((
        components::header(route),
        content,
        components::footer(),
    )))
}

pub fn app(route: Route) -> impl Widget {
    widget(move |scope| {
        let state = AppState { route: scope.cage(route) };
        register(state);
        let routes = scope.bond(move || vec![state.route.get()]);
        each(routes, |route| route.path(), move |route, _| page(route.get())).show_in(scope)
    })
}
