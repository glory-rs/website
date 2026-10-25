use glory::render::{Event, NodeId, Props};
use glory::ui::App;
use glory_website::{Route, app};

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
        assert!(route.title().contains("Glory"));
        assert!(!route.description().is_empty());
    }
    assert_eq!(Route::parse("/docs"), Route::Docs);
    assert_eq!(Route::parse("/docs/reactivity/index.html"), Route::Reactivity);
    assert_eq!(Route::parse("/missing"), Route::NotFound);
}

#[test]
fn every_route_has_complete_ssr_content_and_navigation() {
    glory::ssr::snapshot(glory_website::pages::home()).unwrap();
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
    assert_eq!(content(), "Clicks: 0");
    app.dispatch(button, Event::Activate).unwrap();
    app.flush().unwrap();
    assert_eq!(content(), "Clicks: 1");
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
