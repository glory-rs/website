use crate::Locale;
use crate::{Route, app::request_navigation};
use glory::prelude::*;
use glory::render::{Current, Role};
use sapid::Theme;

pub fn copy(class: &str, content: impl Into<Lotus<String>>) -> impl Widget {
    sapid::text().unstyled().class(class.to_owned()).content(content)
}

pub fn heading(level: u8, class: &str, content: impl Into<String>) -> impl Widget {
    sapid::title(level).unstyled().class(class.to_owned()).content(content.into())
}

pub fn link(locale: Locale, label: impl Into<String>, route: Route, class: &str) -> sapid::Anchor {
    sapid::anchor()
        .unstyled()
        .href(crate::SiteRoute::new(locale, route).path())
        .on_navigate(request_navigation)
        .label(label.into())
        .class(class.to_owned())
}

pub fn external(label: impl Into<String>, href: &str, class: &str) -> sapid::Anchor {
    sapid::anchor().unstyled().href(href.to_owned()).label(label.into()).class(class.to_owned())
}

fn language_link(locale: Locale, page: Route) -> sapid::Anchor {
    let target = crate::SiteRoute::new(locale.other(), page).path();
    sapid::anchor()
        .unstyled()
        .class("language-link")
        .label(locale.text("language-switch"))
        .name(locale.text("language-switch-name"))
        .href(target)
        .on_navigate(move |mut request| {
            #[cfg(target_arch = "wasm32")]
            if let Some(window) = web_sys::window() {
                let location = window.location();
                request.href = crate::SiteRoute::new(locale, page).switch_path(
                    &location.search().unwrap_or_default(),
                    &location.hash().unwrap_or_default(),
                );
            }
            #[cfg(not(target_arch = "wasm32"))]
            let _ = &mut request;
            request_navigation(request);
        })
}

fn nav_link(
    locale: Locale,
    label: impl Into<String>,
    route: Route,
    current: Route,
    mobile: bool,
) -> sapid::Anchor {
    let active = current == route || (route == Route::Docs && current == Route::Reactivity);
    let class = format!(
        "{}{}",
        if mobile { "mobile-nav-link" } else { "nav-link" },
        if active { " is-active" } else { "" }
    );
    link(locale, label, route, &class).current(if active { Some(Current::Page) } else { None })
}

pub fn header(locale: Locale, current: Route) -> impl Widget {
    widget(move |scope| {
        let open = scope.cage(false);
        let menu_label =
            open.map(
                move |open| {
                    if *open { locale.text("common-close") } else { locale.text("common-menu") }
                },
            );
        view()
            .role(Role::Banner)
            .class("site-header")
            .fill((
                view().class("header-inner").fill((
                    view().class("brand-wrap").fill((
                        image().source("/assets/glory.svg").decorative().class("brand-mark"),
                        link(locale, "Glory", Route::Home, "brand"),
                    )),
                    view()
                        .role(Role::Navigation)
                        .name(locale.text("common-primary-navigation"))
                        .class("desktop-nav")
                        .fill((
                            nav_link(
                                locale,
                                locale.text("common-docs"),
                                Route::Docs,
                                current,
                                false,
                            ),
                            nav_link(
                                locale,
                                locale.text("common-architecture"),
                                Route::Architecture,
                                current,
                                false,
                            ),
                            nav_link(
                                locale,
                                locale.text("common-widgets"),
                                Route::Widgets,
                                current,
                                false,
                            ),
                            nav_link(locale, locale.text("common-ssr"), Route::Ssr, current, false),
                            nav_link(
                                locale,
                                locale.text("common-platforms"),
                                Route::Platforms,
                                current,
                                false,
                            ),
                        )),
                    view().class("header-actions").fill((
                        copy("version", "v0.3.1"),
                        external("GitHub", "https://github.com/glory-rs/glory", "github-link"),
                        language_link(locale, current),
                        sapid::button()
                            .unstyled()
                            .class("menu-button")
                            .name(locale.text("common-toggle-navigation"))
                            .label(menu_label)
                            .on_press(move || open.revise(|open| *open = !*open)),
                    )),
                )),
                when(open, move || {
                    view()
                        .role(Role::Navigation)
                        .name(locale.text("common-mobile-navigation"))
                        .class("mobile-nav")
                        .fill((
                            nav_link(
                                locale,
                                locale.text("common-get-started"),
                                Route::Docs,
                                current,
                                true,
                            ),
                            nav_link(
                                locale,
                                locale.text("common-architecture"),
                                Route::Architecture,
                                current,
                                true,
                            ),
                            nav_link(
                                locale,
                                locale.text("common-built-in-widgets"),
                                Route::Widgets,
                                current,
                                true,
                            ),
                            nav_link(
                                locale,
                                locale.text("common-ssr-hydration"),
                                Route::Ssr,
                                current,
                                true,
                            ),
                            nav_link(
                                locale,
                                locale.text("common-platforms"),
                                Route::Platforms,
                                current,
                                true,
                            ),
                            external(
                                locale.text("common-github-repository"),
                                "https://github.com/glory-rs/glory",
                                "mobile-nav-link",
                            ),
                        ))
                }),
            ))
            .show_in(scope)
    })
}

pub fn footer(locale: Locale) -> impl Widget {
    view().role(Role::ContentInfo).class("site-footer").fill((
        view().class("footer-inner").fill((
            view().class("footer-brand").fill((
                image().source("/assets/glory.svg").decorative().class("footer-mark"),
                view().fill((
                    copy("footer-title", "Glory"),
                    copy("footer-tagline", locale.text("common-semantic-ui-handwritten-in-rust")),
                )),
            )),
            view().class("footer-links").fill((
                view().fill((
                    copy("footer-heading", locale.text("common-learn")),
                    link(
                        locale,
                        locale.text("common-architecture"),
                        Route::Architecture,
                        "footer-link",
                    ),
                    link(
                        locale,
                        locale.text("common-write-a-widget"),
                        Route::CustomWidgets,
                        "footer-link",
                    ),
                )),
                view().fill((
                    copy("footer-heading", locale.text("common-rendering")),
                    link(locale, locale.text("common-ssr-hydration"), Route::Ssr, "footer-link"),
                    link(locale, locale.text("common-platforms"), Route::Platforms, "footer-link"),
                )),
            )),
        )),
        view().class("footer-bottom").fill(copy(
            "",
            locale.text("common-built-with-glory-and-sapid-the-framework-is-under"),
        )),
    ))
}

pub fn code_panel(
    locale: Locale,
    theme: Theme,
    name: &str,
    source: &str,
    class: &str,
) -> impl Widget {
    view().class(format!("code-panel {class}")).fill((
        view().class("code-header").fill((
            copy("code-file", name.to_owned()),
            sapid::copy_button()
                .theme(theme.clone())
                .size(sapid::Size::ExtraSmall)
                .label(locale.text("common-copy-code"))
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

pub fn feature(content: impl Into<String>) -> impl Widget {
    view()
        .class("feature-item")
        .fill((copy("feature-check", "✓"), copy("feature-copy", content.into())))
}

pub fn doc_nav(locale: Locale, previous: Option<Route>, next: Option<Route>) -> impl Widget {
    let mut links: Vec<Box<dyn Widget>> = Vec::new();
    if let Some(route) = previous {
        links.push(Box::new(view().class("docs-prev").fill((
            copy("docs-nav-kicker", locale.text("common-previous")),
            link(locale, route.title(locale), route, "docs-nav-link"),
        ))));
    }
    if let Some(route) = next {
        links.push(Box::new(view().class("docs-next").fill((
            copy("docs-nav-kicker", locale.text("common-next")),
            link(locale, route.title(locale), route, "docs-nav-link"),
        ))));
    }
    view()
        .role(Role::Navigation)
        .name(locale.text("common-tutorial-pagination"))
        .class("docs-pagination")
        .fill(links)
}

pub fn docs_layout(locale: Locale, current: Route, content: impl Widget) -> impl Widget {
    let navigation: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-sidebar-title", locale.text("common-start-here"))),
        Box::new(nav_link(
            locale,
            locale.text("common-getting-started"),
            Route::Docs,
            current,
            false,
        )),
        Box::new(nav_link(
            locale,
            locale.text("common-architecture"),
            Route::Architecture,
            current,
            false,
        )),
        Box::new(copy("docs-sidebar-title", locale.text("common-core-concepts"))),
        Box::new(nav_link(
            locale,
            locale.text("common-reactivity"),
            Route::Reactivity,
            current,
            false,
        )),
        Box::new(nav_link(
            locale,
            locale.text("common-built-in-widgets"),
            Route::Widgets,
            current,
            false,
        )),
        Box::new(nav_link(
            locale,
            locale.text("common-custom-widgets"),
            Route::CustomWidgets,
            current,
            false,
        )),
        Box::new(nav_link(locale, locale.text("common-styling"), Route::Styling, current, false)),
        Box::new(copy("docs-sidebar-title", locale.text("common-rendering-platforms"))),
        Box::new(nav_link(locale, locale.text("common-web"), Route::Web, current, false)),
        Box::new(nav_link(locale, locale.text("common-ssr-hydration"), Route::Ssr, current, false)),
        Box::new(nav_link(
            locale,
            locale.text("common-platforms"),
            Route::Platforms,
            current,
            false,
        )),
        Box::new(external(
            locale.text("common-api-on-docs-rs"),
            "https://docs.rs/glory",
            "docs-external-link",
        )),
    ];
    view().role(Role::Main).class("docs-page").fill((
        view()
            .role(Role::Navigation)
            .name(locale.text("common-documentation"))
            .class("docs-sidebar")
            .fill(navigation),
        view().class("docs-article").fill(content),
    ))
}
