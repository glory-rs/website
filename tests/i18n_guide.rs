use glory::{
    i18n::{Args, Error, Locale},
    render::{Event, NodeId, Props},
    ui::App,
};
use glory_website::{
    Route, SiteRoute, app,
    examples::{i18n_catalog, i18n_server, i18n_shared},
};

fn plain(value: &str) -> String {
    value.replace(['\u{2068}', '\u{2069}'], "")
}
fn nodes(app: &App) -> Vec<NodeId> {
    let mut pending = app.tree().roots().to_vec();
    let mut nodes = Vec::new();
    while let Some(id) = pending.pop() {
        pending.extend(app.tree().node(id).unwrap().children.iter().copied());
        nodes.push(id);
    }
    nodes
}
fn click(app: &App, label: &str) {
    let id = nodes(app).into_iter().find(|id| {
        matches!(&app.tree().node(*id).unwrap().descriptor.view().props, Props::Button(button) if button.label == label)
    }).unwrap_or_else(|| panic!("missing button {label}"));
    app.dispatch(id, Event::Activate).unwrap();
    app.flush().unwrap();
}
fn message(app: &App) -> String {
    nodes(app)
        .into_iter()
        .find_map(|id| {
            let tree = app.tree();
            let node = tree.node(id).unwrap();
            let view = node.descriptor.view();
            if view.classes.iter().any(|name| name == "i18n-demo-message") {
                match &view.props {
                    Props::Text(text) => Some(plain(&text.content)),
                    _ => None,
                }
            } else {
                None
            }
        })
        .expect("demo message")
}

#[test]
fn displayed_catalog_example_resolves_parameters_and_plural_categories() {
    let catalog = i18n_catalog::catalog();
    assert!(catalog.validate_complete().is_empty());
    for tag in ["en", "zh-CN"] {
        let locale: Locale = tag.parse().unwrap();
        let greeting = plain(&i18n_catalog::greeting(&catalog, &locale, "Ada").unwrap());
        assert_eq!(greeting, if tag == "en" { "Hello, Ada!" } else { "你好，Ada！" });
        for count in [0, 1, 2] {
            let mut args = Args::new();
            args.set("count", count);
            let text = plain(&catalog.format_exact(&locale, "inbox", Some(&args)).unwrap());
            let expected = if tag == "zh-CN" {
                format!("你有 {count} 条消息。")
            } else if count == 1 {
                "You have one message.".into()
            } else {
                format!("You have {count} messages.")
            };
            assert_eq!(text, expected);
        }
        assert!(matches!(
            catalog.format_exact(&locale, "greeting", None),
            Err(Error::Format { .. })
        ));
    }
}

#[test]
fn displayed_widget_switches_language_without_resetting_its_count() {
    let app = App::memory();
    app.mount(i18n_shared::language_demo("en".parse().unwrap())).unwrap();
    assert_eq!(message(&app), "You have one message.");
    click(&app, "Add a message");
    assert_eq!(message(&app), "You have 2 messages.");
    click(&app, "简体中文");
    assert_eq!(message(&app), "你有 2 条消息。");
    click(&app, "增加一条消息");
    assert_eq!(message(&app), "你有 3 条消息。");
    click(&app, "English");
    assert_eq!(message(&app), "You have 3 messages.");
}

#[test]
fn displayed_server_example_uses_the_shared_url_policy_and_snapshot() {
    for (path, tag, text) in
        [("/demo", "en", "You have one message."), ("/zh/demo", "zh-CN", "你有")]
    {
        let html = i18n_server::render_page(path).unwrap();
        assert!(html.contains(&format!("<html lang=\"{tag}\">")));
        assert!(html.contains(text));
        assert!(html.contains("data-gly-base=\"v1\""));
        assert!(html.contains("/app.js"));
    }
    assert_eq!(i18n_shared::locale_for_path("/zh").to_string(), "zh-CN");
    assert_eq!(i18n_shared::locale_for_path("/zh/demo?lang=en#preview").to_string(), "zh-CN");
    assert_eq!(i18n_shared::locale_for_path("/zh-other").to_string(), "en");
}

#[test]
fn both_guide_routes_include_examples_and_localized_navigation() {
    for locale in glory_website::Locale::ALL {
        let route = SiteRoute::new(locale, Route::I18n);
        assert_eq!(SiteRoute::parse(&route.path()), route);
        let html = glory::ssr::snapshot(app(route)).unwrap().html;
        for marker in [
            "i18n-demo-message",
            "Catalog::new",
            "format_exact",
            "hydrate_with",
            "BASE_CSS",
            "demo.zh-CN.ftl",
        ] {
            assert!(html.contains(marker), "{marker}");
        }
        let prefix = if locale == glory_website::Locale::En { "" } else { "/zh" };
        for path in ["i18n", "styling", "web"] {
            assert!(html.contains(&format!("href=\"{prefix}/docs/{path}/\"")));
        }
    }
}
