use glory::render::{Event, NodeId, Props};
use glory::ui::App;
use glory_website::{Locale, Route, SiteRoute, app, i18n, metadata};

fn nodes(app: &App) -> Vec<NodeId> {
    let mut pending = app.tree().roots().to_vec();
    let mut result = Vec::new();
    while let Some(id) = pending.pop() {
        pending.extend(app.tree().node(id).unwrap().children.iter().copied());
        result.push(id);
    }
    result
}

#[test]
fn routes_round_trip_and_unknown_paths_have_a_page() {
    for route in Route::ALL {
        assert_eq!(Route::parse(route.path()), route);
        assert!(route.title(Locale::En).contains("Glory"));
        assert!(!route.description(Locale::En).is_empty());
    }
    assert_eq!(Route::parse("/docs"), Route::Docs);
    assert_eq!(Route::parse("/docs/reactivity/index.html"), Route::Reactivity);
    assert_eq!(Route::parse("/missing"), Route::NotFound);
}

#[test]
fn every_route_has_complete_ssr_content_and_navigation() {
    glory::ssr::snapshot(glory_website::pages::home(Locale::En)).unwrap();
    for route in Route::ALL.into_iter().chain([Route::NotFound]) {
        let html = glory::ssr::snapshot(app(route)).unwrap().html;
        assert!(html.contains("glory-site"), "{route:?}");
        assert!(html.contains("Primary navigation"), "{route:?}");
        assert!(html.contains("href=\"/docs/\""), "{route:?}");
        assert!(html.contains("href=\"/docs/widgets/\""), "{route:?}");
        if !matches!(route, Route::Home | Route::NotFound) {
            assert!(html.contains("START HERE"), "{route:?}");
            assert!(html.contains("CORE CONCEPTS"), "{route:?}");
            assert!(html.contains("RENDERING &amp; PLATFORMS"), "{route:?}");
            assert!(html.contains("Tutorial pagination"), "{route:?}");
        }
        assert!(!html.contains("<iframe"), "{route:?}");
        match route {
            Route::Home => assert!(html.contains("One language.")),
            Route::Docs => assert!(html.contains("Learn Glory from first principles.")),
            Route::Architecture => assert!(html.contains("Architecture and design.")),
            Route::Reactivity => assert!(html.contains("Fine-grained reactivity.")),
            Route::Widgets => assert!(html.contains("Compose with built-in widgets.")),
            Route::CustomWidgets => assert!(html.contains("Writing custom widgets.")),
            Route::Styling => assert!(html.contains("Style without losing the boundaries.")),
            Route::Web => assert!(html.contains("Web applications.")),
            Route::Ssr => assert!(html.contains("SSR and hydration.")),
            Route::Platforms => assert!(html.contains("Share widgets across platforms.")),
            Route::NotFound => assert!(html.contains("This page drifted away.")),
        }
    }
}

#[test]
fn counter_updates_the_shared_semantic_tree() {
    let app = App::memory();
    app.mount(glory_website::app(Route::Home)).unwrap();
    let all = nodes(&app);
    let button = all
        .iter()
        .copied()
        .find(|id| {
            app.tree().node(*id).unwrap().descriptor.view().name.as_deref()
                == Some("Add one to the counter")
        })
        .expect("counter button");
    let label = all
        .iter()
        .copied()
        .find(|id| {
            app.tree()
                .node(*id)
                .unwrap()
                .descriptor
                .view()
                .classes
                .iter()
                .any(|class| class == "counter-label")
        })
        .expect("counter label");
    let content = || match &app.tree().node(label).unwrap().descriptor.view().props {
        Props::Text(text) => text.content.clone(),
        other => panic!("expected text, got {other:?}"),
    };
    assert_eq!(content().replace(['\u{2068}', '\u{2069}'], ""), "Clicks: 0");
    app.dispatch(button, Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(content().replace(['\u{2068}', '\u{2069}'], ""), "Clicks: 1");
}

#[test]
fn localized_routes_preserve_page_and_navigation_context() {
    for locale in Locale::ALL {
        for page in Route::ALL.into_iter().chain([Route::NotFound]) {
            let route = SiteRoute::new(locale, page);
            assert_eq!(SiteRoute::parse(&route.path()), route);
            assert_eq!(SiteRoute::parse(&format!("{}?q=rust#example", route.path())), route);
            let switched = route.switch_path("?q=rust", "#example");
            assert_eq!(SiteRoute::parse(&switched), SiteRoute::new(locale.other(), page));
            assert!(switched.ends_with("?q=rust#example"));
        }
    }
    assert_eq!(SiteRoute::parse("/zh"), SiteRoute::new(Locale::ZhCn, Route::Home));
    assert_eq!(
        SiteRoute::parse("/zh/docs/web/index.html"),
        SiteRoute::new(Locale::ZhCn, Route::Web)
    );
    assert_eq!(SiteRoute::parse("/zh/missing/deep"), SiteRoute::new(Locale::ZhCn, Route::NotFound));
    assert_eq!(SiteRoute::parse("/zhwhatever"), SiteRoute::new(Locale::En, Route::NotFound));
}

#[test]
fn every_translation_is_complete_and_resolves() {
    assert!(i18n::catalog().validate_complete().is_empty());
    let mut args = glory::i18n::Args::new();
    args.set("count", 2);
    let source = include_str!("../locales/en.ftl");
    let mut count = 0;
    for line in source.lines().filter(|line| !line.starts_with([' ', '#'])) {
        if let Some((key, _)) = line.split_once(" = ") {
            for locale in Locale::ALL {
                assert!(!locale.format(key, Some(&args)).is_empty(), "{key}");
            }
            count += 1;
        }
    }
    assert!(count > 350);
}

#[test]
fn chinese_pages_render_translated_content_links_and_metadata() {
    for page in Route::ALL.into_iter().chain([Route::NotFound]) {
        let route = SiteRoute::new(Locale::ZhCn, page);
        let html = glory::ssr::snapshot(app(route)).unwrap().html;
        assert!(html.contains("主导航"), "{page:?}");
        assert!(html.contains("href=\"/zh/docs/\""), "{page:?}");
        assert!(html.contains("href=\"/zh/docs/widgets/\""), "{page:?}");
        assert!(html.contains("切换到英文"));
        if !matches!(page, Route::Home | Route::NotFound) {
            assert!(html.contains("核心概念"));
            assert!(html.contains("教程翻页导航"));
        }
        let document = metadata::document(route, &html);
        assert!(document.contains("<html lang=\"zh-CN\">"));
        assert!(document.contains(&format!("<title>{}</title>", metadata::escape(&route.title()))));
        assert!(document.contains(&format!(
            "rel=\"canonical\" href=\"https://glory.taidge.com{}\"",
            route.path()
        )));
        if page == Route::NotFound {
            assert!(document.contains("name=\"robots\" content=\"noindex\""));
            assert!(!document.contains("hreflang="));
        } else {
            assert!(document.contains("hreflang=\"en\""));
            assert!(document.contains("hreflang=\"zh-CN\""));
            assert!(document.contains("hreflang=\"x-default\""));
        }
    }
    let home = glory::ssr::snapshot(app(SiteRoute::new(Locale::ZhCn, Route::Home))).unwrap().html;
    assert!(home.contains("一种语言。"));
    assert!(home.contains("每一种界面。"));
    assert!(home.contains("点击次数："));
    let sitemap = metadata::sitemap();
    assert_eq!(sitemap.matches("<url>").count(), 20);
    assert!(!sitemap.contains("404.html"));
}

#[test]
fn mobile_navigation_is_controlled_by_rust_state() {
    let app = App::memory();
    app.mount(glory_website::app(Route::Docs)).unwrap();
    let all = nodes(&app);
    let toggle = all
        .iter()
        .copied()
        .find(|id| {
            app.tree().node(*id).unwrap().descriptor.view().name.as_deref()
                == Some("Toggle navigation")
        })
        .expect("menu toggle");
    let mobile_links = || {
        nodes(&app)
            .into_iter()
            .filter(|id| {
                app.tree()
                    .node(*id)
                    .unwrap()
                    .descriptor
                    .view()
                    .classes
                    .iter()
                    .any(|class| class == "mobile-nav-link")
            })
            .count()
    };
    assert_eq!(mobile_links(), 0);
    app.dispatch(toggle, Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(mobile_links(), 6);
}
