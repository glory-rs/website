use crate::Locale;
use crate::navigation::{self, API_URL, SAPID_URL};
use crate::{Route, app::request_navigation};
use glory::prelude::*;
use glory::render::{Current, Role};
use sapid::Theme;

pub fn copy(class: &str, content: impl Into<Vary<String>>) -> impl Widget {
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

fn language_picker(locale: Locale, page: Route) -> impl Widget {
    let mut menu = sapid::dropdown_menu(format!("site-language-{}-{page:?}", locale.tag()))
        .unstyled()
        .class("language-options")
        .label(locale.native_name());
    for target in Locale::ALL {
        menu = menu.item(
            sapid::menu_item(target.tag())
                .label(target.native_name())
                .class(if target == locale {
                    "language-option is-current"
                } else {
                    "language-option"
                })
                .on_activate(move || {
                    if target == locale {
                        return;
                    }
                    #[cfg(target_arch = "wasm32")]
                    if let Some(window) = web_sys::window() {
                        let location = window.location();
                        let href = crate::SiteRoute::new(locale, page).switch_path(
                            &location.search().unwrap_or_default(),
                            &location.hash().unwrap_or_default(),
                        );
                        if let Err(error) = crate::web::navigate(&href) {
                            web_sys::console::error_1(&error.into());
                        }
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    let _ = page;
                }),
        );
    }
    view().name(locale.text("language-menu")).class("language-picker").fill(menu)
}

fn nav_link(
    locale: Locale,
    label: impl Into<String>,
    route: Route,
    current: Route,
    mobile: bool,
) -> sapid::Anchor {
    let active = current == route;
    let class = format!(
        "{}{}",
        if mobile { "mobile-nav-link" } else { "nav-link" },
        if active { " is-active" } else { "" }
    );
    link(locale, label, route, &class).current(if active { Some(Current::Page) } else { None })
}

fn primary_links(locale: Locale, current: Route, mobile: bool) -> Vec<Box<dyn Widget>> {
    let in_docs = navigation::is_documentation(current);
    let section = if in_docs { Route::Docs } else { current };
    let docs = nav_link(locale, locale.text("common-docs"), Route::Docs, section, mobile).current(
        if !in_docs {
            None
        } else if current == Route::Docs {
            Some(Current::Page)
        } else {
            Some(Current::Location)
        },
    );
    let class = if mobile { "mobile-nav-link" } else { "nav-link" };
    vec![
        Box::new(docs),
        Box::new(external(locale.text("navigation-api"), API_URL, class)),
        Box::new(external("Sapid ↗", SAPID_URL, class)),
    ]
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
                        image()
                            .source(crate::assets::media("glory.svg"))
                            .decorative()
                            .class("brand-mark"),
                        link(locale, "Glory", Route::Home, "brand"),
                    )),
                    view()
                        .role(Role::Navigation)
                        .name(locale.text("common-primary-navigation"))
                        .class("desktop-nav")
                        .fill(primary_links(locale, current, false)),
                    view().class("header-actions").fill((
                        copy("version", "v0.6.0"),
                        external("GitHub", "https://github.com/glory-rs/glory", "github-link"),
                        language_picker(locale, current),
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
                        .fill({
                            let mut links = primary_links(locale, current, true);
                            links.push(Box::new(external(
                                locale.text("common-github-repository"),
                                "https://github.com/glory-rs/glory",
                                "mobile-nav-link",
                            )));
                            links
                        })
                }),
            ))
            .show_in(scope)
    })
}

pub fn footer(locale: Locale) -> impl Widget {
    view().role(Role::ContentInfo).class("site-footer").fill((
        view().class("footer-inner").fill((
            view().class("footer-brand").fill((
                image().source(crate::assets::media("glory.svg")).decorative().class("footer-mark"),
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
    source_panel(locale, theme, name, source, class, sapid::CodeLanguage::Rust)
}

pub fn source_panel(
    locale: Locale,
    theme: Theme,
    name: &str,
    source: &str,
    class: &str,
    language: sapid::CodeLanguage,
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
            .language(language)
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
    let mut navigation: Vec<Box<dyn Widget>> = Vec::new();
    for group in navigation::DOC_GROUPS {
        navigation.push(Box::new(copy("docs-sidebar-title", locale.text(group.title))));
        for chapter in group.chapters {
            navigation.push(Box::new(nav_link(
                locale,
                locale.text(chapter.label),
                chapter.route,
                current,
                false,
            )));
        }
    }
    navigation.push(Box::new(external(locale.text("navigation-api"), API_URL, "nav-link")));
    navigation.push(Box::new(external(
        locale.text("navigation-sapid-library"),
        SAPID_URL,
        "nav-link",
    )));
    view().role(Role::Main).class("docs-page").fill((
        view()
            .role(Role::Navigation)
            .name(locale.text("common-documentation"))
            .class("docs-sidebar")
            .fill(navigation),
        view().class("docs-article").fill(content),
    ))
}

pub fn documentation_map(locale: Locale) -> impl Widget {
    let cards: Vec<Box<dyn Widget>> = navigation::DOC_GROUPS
        .iter()
        .map(|group| {
            let chapters: Vec<Box<dyn Widget>> = group
                .chapters
                .iter()
                .map(|chapter| {
                    Box::new(link(locale, locale.text(chapter.label), chapter.route, "text-link"))
                        as Box<dyn Widget>
                })
                .collect();
            Box::new(view().class("docs-map-card").fill((
                heading(3, "docs-card-title", locale.text(group.title)),
                copy("docs-paragraph", locale.text(group.description)),
                view().class("docs-map-links").fill(chapters),
            ))) as Box<dyn Widget>
        })
        .collect();
    view().class("docs-map").fill(cards)
}
