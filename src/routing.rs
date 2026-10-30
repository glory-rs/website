//! Shared declarations for static generation, hydration and client navigation.
use crate::{Locale, Route, SiteRoute};
use glory::{
    prelude::*,
    routing::{
        Location, MatchPlan, MemoryHistory, RouteDef, RouteKey, Router, RouterHandle, Routes,
        SlashPolicy,
    },
};
use std::rc::Rc;

pub const SCHEMA: &str = "glory-website-routing-v1";

macro_rules! declarations {
    ($($page:ident),* $(,)?) => { &[$((Route::$page, [
        stringify!($page), concat!("zh.", stringify!($page)),
        concat!("index.", stringify!($page)), concat!("zh.index.", stringify!($page)),
    ])),*] };
}
const PAGES: &[(Route, [&str; 4])] = declarations![
    Home,
    Docs,
    Architecture,
    Reactivity,
    Display,
    Widgets,
    CustomWidgets,
    Styling,
    I18n,
    Web,
    Ssr,
    Platforms,
];

fn entries() -> impl Iterator<Item = (RouteKey<()>, SiteRoute, String)> {
    PAGES.iter().flat_map(|(page, ids)| {
        Locale::ALL
            .into_iter()
            .enumerate()
            .flat_map(move |(language, locale)| {
                let site = SiteRoute::new(locale, *page);
                let path = site.path();
                [
                    (RouteKey::new(ids[language]), site, path.clone()),
                    (
                        RouteKey::new(ids[language + 2]),
                        site,
                        format!("{path}index.html"),
                    ),
                ]
            })
    })
}

pub fn routes() -> Routes {
    let mut builder = Routes::builder().slash_policy(SlashPolicy::Ignore);
    for (key, _, path) in entries() {
        builder = builder.root(RouteDef::new(key, path));
    }
    builder.finish().expect("static website routes")
}

pub fn site_route(location: &Location, plan: &MatchPlan) -> SiteRoute {
    if let Some(matched) = plan.chain.last()
        && let Some((_, site, _)) = entries().find(|(key, _, _)| key.id() == matched.id)
    {
        return site;
    }
    let path = location.path();
    let locale = if path == "/zh" || path.starts_with("/zh/") {
        Locale::ZhCn
    } else {
        Locale::En
    };
    SiteRoute::new(locale, Route::NotFound)
}

pub fn parse(target: &str) -> SiteRoute {
    let definitions = routes();
    let Ok(location) = Location::parse(if target.is_empty() { "/" } else { target }) else {
        return SiteRoute::new(Locale::En, Route::NotFound);
    };
    let plan = definitions.resolve(&location).unwrap_or_default();
    site_route(&location, &plan)
}

// URL aliases and query/fragment changes share one language/chapter identity.
// Derive it directly from Router's snapshot rather than mirror a route Cage.
pub fn view(router: RouterHandle) -> impl Widget {
    widget(move |scope| {
        let observed = router.clone();
        let source = scope.bond(move || {
            let snapshot = observed.snapshot().expect("mounted website router");
            vec![site_route(snapshot.location(), &snapshot.plan)]
        });
        each(source)
            .key(|site| *site)
            .seed(crate::app::page)
            .show_in(scope)
    })
}

pub fn static_app(site: SiteRoute) -> impl Widget {
    widget(move |scope| {
        let definitions = routes();
        let router = Router::new(
            scope,
            definitions.clone(),
            Rc::new(MemoryHistory::new(
                Location::parse(&site.path()).expect("static site URL"),
            )),
        )
        .map_err(|error| glory::render::Error::Backend(error.to_string()))?;
        view(router.handle()).show_in(scope)
    })
}
