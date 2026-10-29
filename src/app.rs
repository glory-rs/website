use crate::{Route, SiteRoute, components, pages};
use glory::prelude::*;

#[derive(Clone, Copy)]
pub struct AppState {
    pub route: Cage<SiteRoute>,
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

fn page(route: SiteRoute) -> Box<dyn Widget> {
    let locale = route.locale;
    let content: Box<dyn Widget> = match route.page {
        Route::Home => Box::new(pages::home(locale)),
        Route::Docs => Box::new(pages::docs(locale)),
        Route::Architecture => Box::new(pages::architecture(locale)),
        Route::Reactivity => Box::new(pages::reactivity(locale)),
        Route::Display => Box::new(pages::display(locale)),
        Route::Widgets => Box::new(pages::widgets(locale)),
        Route::CustomWidgets => Box::new(pages::custom_widgets(locale)),
        Route::Styling => Box::new(pages::styling(locale)),
        Route::I18n => Box::new(pages::i18n(locale)),
        Route::Web => Box::new(pages::web(locale)),
        Route::Ssr => Box::new(pages::ssr(locale)),
        Route::Platforms => Box::new(pages::platforms(locale)),
        Route::NotFound => Box::new(pages::not_found(locale)),
    };
    let theme = sapid::Theme::blue_dark();
    let section =
        if crate::navigation::is_documentation(route.page) { " documentation-site" } else { "" };
    Box::new(view().class(format!("glory-site {}{section}", theme.class())).fill((
        components::header(locale, route.page),
        content,
        components::footer(locale),
    )))
}

pub fn app(route: impl Into<SiteRoute>) -> impl Widget {
    let route = route.into();
    widget(move |scope| {
        let state = AppState { route: scope.cage(route) };
        register(state);
        let routes = scope.bond(move || vec![state.route.get()]);
        each(routes)
            .key(|route| *route)
            .children(move |entry| {
                let route = entry.value;
                page(route.get())
            })
            .show_in(scope)
    })
}
