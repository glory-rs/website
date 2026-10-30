use glory::{
    prelude::*,
    routing::{Bootstrap, Error, Location, MemoryHistory, Router},
};
use glory_website::{Locale, Route, SiteRoute, routing};
use std::{cell::RefCell, rc::Rc};

#[test]
fn shared_definitions_match_both_languages_aliases_and_real_not_found() {
    let routes = routing::routes();
    for locale in Locale::ALL {
        for page in Route::ALL {
            let site = SiteRoute::new(locale, page);
            for target in [
                site.path(),
                format!("{}index.html", site.path()),
                site.path().trim_end_matches('/').to_owned(),
            ] {
                let location =
                    Location::parse(if target.is_empty() { "/" } else { &target }).unwrap();
                let plan = routes.resolve(&location).unwrap();
                assert_eq!(routing::site_route(&location, &plan), site);
                let seed = Bootstrap::new(&routes, location.clone(), routing::SCHEMA).unwrap();
                assert_eq!(seed.validate(&routes, routing::SCHEMA).unwrap(), location);
            }
        }
    }
    assert!(matches!(
        routes.resolve(&Location::parse("/zh/missing/deep").unwrap()),
        Err(Error::NoMatch)
    ));
}

#[test]
fn router_retains_query_only_page_state_and_handles_localized_fallback_and_history() {
    let app = App::memory();
    let saved = Rc::new(RefCell::new(None));
    let capture = saved.clone();
    app.mount(widget(move |scope| {
        let routes = routing::routes();
        let router = Router::new(
            scope,
            routes.clone(),
            Rc::new(MemoryHistory::new(Location::parse("/").unwrap())),
        )
        .unwrap();
        *capture.borrow_mut() = Some(router.handle());
        routing::view(router.handle()).show_in(scope)
    }))
    .unwrap();
    let router = saved.borrow().as_ref().unwrap().clone();
    let roots = app.tree().roots().to_vec();
    router
        .push(Location::parse("/?source=test#example").unwrap())
        .unwrap();
    app.flush().unwrap();
    assert_eq!(app.tree().roots(), roots);
    router
        .push(Location::parse("/zh/docs/web/").unwrap())
        .unwrap();
    app.flush().unwrap();
    let snapshot = router.current().unwrap();
    assert_eq!(
        routing::site_route(snapshot.location(), &snapshot.plan),
        SiteRoute::new(Locale::ZhCn, Route::Web)
    );
    router
        .push(Location::parse("/zh/unknown?kept=1#example").unwrap())
        .unwrap();
    app.flush().unwrap();
    let snapshot = router.current().unwrap();
    assert_eq!(snapshot.error, Some(Error::NoMatch));
    assert_eq!(
        routing::site_route(snapshot.location(), &snapshot.plan),
        SiteRoute::new(Locale::ZhCn, Route::NotFound)
    );
    router.back().unwrap();
    app.flush().unwrap();
    assert_eq!(router.current().unwrap().location().path(), "/zh/docs/web/");
    app.clear().unwrap();
    assert!(!router.is_alive());
}
