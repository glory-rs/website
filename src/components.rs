use crate::{Route, app::request_navigation};
use glory::prelude::*;
use glory::render::{Current, Role};
use sapid::Theme;

pub fn copy(class: &str, content: impl Into<Lotus<String>>) -> impl Widget {
    sapid::text().unstyled().class(class.to_owned()).content(content)
}

pub fn heading(level: u8, class: &str, content: &str) -> impl Widget {
    sapid::title(level).unstyled().class(class.to_owned()).content(content.to_owned())
}

pub fn link(label: &str, route: Route, class: &str) -> sapid::Anchor {
    sapid::anchor()
        .unstyled()
        .href(route.path())
        .on_navigate(request_navigation)
        .label(label.to_owned())
        .class(class.to_owned())
}

pub fn external(label: &str, href: &str, class: &str) -> sapid::Anchor {
    sapid::anchor().unstyled().href(href.to_owned()).label(label.to_owned()).class(class.to_owned())
}

fn nav_link(label: &str, route: Route, current: Route, mobile: bool) -> sapid::Anchor {
    let active = current == route || (route == Route::Docs && current == Route::Reactivity);
    let class = format!(
        "{}{}",
        if mobile { "mobile-nav-link" } else { "nav-link" },
        if active { " is-active" } else { "" }
    );
    link(label, route, &class).current(if active { Some(Current::Page) } else { None })
}

pub fn header(current: Route) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let menu_label = open.map(|open| if *open { "Close" } else { "Menu" }.to_owned());
        view()
            .role(Role::Banner)
            .class("site-header")
            .fill((
                view().class("header-inner").fill((
                    view().class("brand-wrap").fill((
                        image().source("/assets/glory.svg").decorative().class("brand-mark"),
                        link("Glory", Route::Home, "brand"),
                    )),
                    view()
                        .role(Role::Navigation)
                        .name("Primary navigation")
                        .class("desktop-nav")
                        .fill((
                            nav_link("Docs", Route::Docs, current, false),
                            nav_link("Architecture", Route::Architecture, current, false),
                            nav_link("Widgets", Route::Widgets, current, false),
                            nav_link("SSR", Route::Ssr, current, false),
                            nav_link("Platforms", Route::Platforms, current, false),
                        )),
                    view().class("header-actions").fill((
                        copy("version", "v0.3.1"),
                        external("GitHub", "https://github.com/glory-rs/glory", "github-link"),
                        sapid::button()
                            .unstyled()
                            .class("menu-button")
                            .name("Toggle navigation")
                            .label(menu_label)
                            .on_press(move || open.revise(|open| *open = !*open)),
                    )),
                )),
                when(open, move || {
                    view()
                        .role(Role::Navigation)
                        .name("Mobile navigation")
                        .class("mobile-nav")
                        .fill((
                            nav_link("Get started", Route::Docs, current, true),
                            nav_link("Architecture", Route::Architecture, current, true),
                            nav_link("Built-in widgets", Route::Widgets, current, true),
                            nav_link("SSR & hydration", Route::Ssr, current, true),
                            nav_link("Platforms", Route::Platforms, current, true),
                            external(
                                "GitHub repository",
                                "https://github.com/glory-rs/glory",
                                "mobile-nav-link",
                            ),
                        ))
                }),
            ))
            .show_in(scope)
    })
}

pub fn footer() -> impl Widget {
    view().role(Role::ContentInfo).class("site-footer").fill((
        view().class("footer-inner").fill((
            view().class("footer-brand").fill((
                image().source("/assets/glory.svg").decorative().class("footer-mark"),
                view().fill((
                    copy("footer-title", "Glory"),
                    copy("footer-tagline", "Semantic UI, handwritten in Rust."),
                )),
            )),
            view().class("footer-links").fill((
                view().fill((
                    copy("footer-heading", "Learn"),
                    link("Architecture", Route::Architecture, "footer-link"),
                    link("Write a widget", Route::CustomWidgets, "footer-link"),
                )),
                view().fill((
                    copy("footer-heading", "Rendering"),
                    link("SSR & hydration", Route::Ssr, "footer-link"),
                    link("Platforms", Route::Platforms, "footer-link"),
                )),
            )),
        )),
        view().class("footer-bottom").fill(copy(
            "",
            "Built with Glory and Sapid. The framework is under active development.",
        )),
    ))
}

pub fn code_panel(theme: Theme, name: &str, source: &str, class: &str) -> impl Widget {
    view().class(format!("code-panel {class}")).fill((
        view().class("code-header").fill((
            copy("code-file", name.to_owned()),
            sapid::copy_button()
                .theme(theme.clone())
                .size(sapid::Size::ExtraSmall)
                .label("Copy code")
                .source(source.to_owned()),
        )),
        sapid::code_highlight()
            .theme(theme)
            .language(sapid::CodeLanguage::Rust)
            .source(source.to_owned())
            .line_numbers(true)
            .class("code-content"),
    ))
}

pub fn feature(content: &str) -> impl Widget {
    view()
        .class("feature-item")
        .fill((copy("feature-check", "✓"), copy("feature-copy", content.to_owned())))
}

pub fn doc_nav(previous: Option<Route>, next: Option<Route>) -> impl Widget {
    let mut links: Vec<Box<dyn Widget>> = Vec::new();
    if let Some(route) = previous {
        links.push(Box::new(view().class("docs-prev").fill((
            copy("docs-nav-kicker", "PREVIOUS"),
            link(route.title(), route, "docs-nav-link"),
        ))));
    }
    if let Some(route) = next {
        links.push(Box::new(
            view().class("docs-next").fill((
                copy("docs-nav-kicker", "NEXT"),
                link(route.title(), route, "docs-nav-link"),
            )),
        ));
    }
    view().role(Role::Navigation).name("Tutorial pagination").class("docs-pagination").fill(links)
}

pub fn docs_layout(current: Route, content: impl Widget) -> impl Widget {
    let navigation: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-sidebar-title", "START HERE")),
        Box::new(nav_link("Getting started", Route::Docs, current, false)),
        Box::new(nav_link("Architecture", Route::Architecture, current, false)),
        Box::new(copy("docs-sidebar-title", "CORE CONCEPTS")),
        Box::new(nav_link("Reactivity", Route::Reactivity, current, false)),
        Box::new(nav_link("Built-in widgets", Route::Widgets, current, false)),
        Box::new(nav_link("Custom widgets", Route::CustomWidgets, current, false)),
        Box::new(nav_link("Styling", Route::Styling, current, false)),
        Box::new(copy("docs-sidebar-title", "RENDERING & PLATFORMS")),
        Box::new(nav_link("Web", Route::Web, current, false)),
        Box::new(nav_link("SSR & hydration", Route::Ssr, current, false)),
        Box::new(nav_link("Platforms", Route::Platforms, current, false)),
        Box::new(external("API on docs.rs ↗", "https://docs.rs/glory", "docs-external-link")),
    ];
    view().role(Role::Main).class("docs-page").fill((
        view().role(Role::Navigation).name("Documentation").class("docs-sidebar").fill(navigation),
        view().class("docs-article").fill(content),
    ))
}
