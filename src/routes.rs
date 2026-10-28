use crate::Locale;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SiteRoute {
    pub locale: Locale,
    pub page: Route,
}

impl SiteRoute {
    pub const fn new(locale: Locale, page: Route) -> Self {
        Self { locale, page }
    }
    pub fn parse(path: &str) -> Self {
        let path = path.split(['?', '#']).next().unwrap_or(path);
        if path == "/zh" || path.starts_with("/zh/") {
            Self::new(Locale::ZhCn, Route::parse(&path[3..]))
        } else {
            Self::new(Locale::En, Route::parse(path))
        }
    }
    pub fn path(self) -> String {
        match self.locale {
            Locale::En => self.page.path().to_owned(),
            Locale::ZhCn => format!("/zh{}", self.page.path()),
        }
    }
    pub fn title(self) -> String {
        self.page.title(self.locale)
    }
    pub fn description(self) -> String {
        self.page.description(self.locale)
    }
    /// Replace only the language/path, retaining the navigation context.
    pub fn switch_path(self, search: &str, hash: &str) -> String {
        format!("{}{}{}", Self::new(self.locale.other(), self.page).path(), search, hash)
    }
}

impl From<Route> for SiteRoute {
    fn from(page: Route) -> Self {
        Self::new(Locale::En, page)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Route {
    #[default]
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
    NotFound,
}

impl Route {
    pub const ALL: [Route; 12] = [
        Route::Home,
        Route::Docs,
        Route::Architecture,
        Route::Reactivity,
        Route::Display,
        Route::Widgets,
        Route::CustomWidgets,
        Route::Styling,
        Route::I18n,
        Route::Web,
        Route::Ssr,
        Route::Platforms,
    ];

    pub fn parse(path: &str) -> Self {
        let path = path.split(['?', '#']).next().unwrap_or(path);
        let normalized = if path.len() > 1 { path.trim_end_matches('/') } else { path };
        match normalized {
            "" | "/" | "/index.html" => Self::Home,
            "/docs" | "/docs/index.html" => Self::Docs,
            "/docs/architecture" | "/docs/architecture/index.html" => Self::Architecture,
            "/docs/reactivity" | "/docs/reactivity/index.html" => Self::Reactivity,
            "/docs/display" | "/docs/display/index.html" => Self::Display,
            "/docs/widgets" | "/docs/widgets/index.html" => Self::Widgets,
            "/docs/custom-widgets" | "/docs/custom-widgets/index.html" => Self::CustomWidgets,
            "/docs/styling" | "/docs/styling/index.html" => Self::Styling,
            "/docs/i18n" | "/docs/i18n/index.html" => Self::I18n,
            "/docs/web" | "/docs/web/index.html" => Self::Web,
            "/docs/ssr" | "/docs/ssr/index.html" => Self::Ssr,
            "/docs/platforms" | "/docs/platforms/index.html" => Self::Platforms,
            _ => Self::NotFound,
        }
    }

    pub fn path(self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Docs => "/docs/",
            Self::Architecture => "/docs/architecture/",
            Self::Reactivity => "/docs/reactivity/",
            Self::Display => "/docs/display/",
            Self::Widgets => "/docs/widgets/",
            Self::CustomWidgets => "/docs/custom-widgets/",
            Self::Styling => "/docs/styling/",
            Self::I18n => "/docs/i18n/",
            Self::Web => "/docs/web/",
            Self::Ssr => "/docs/ssr/",
            Self::Platforms => "/docs/platforms/",
            Self::NotFound => "/404.html",
        }
    }

    pub fn title(self, locale: Locale) -> String {
        match self {
            Self::Home => locale.text("meta-glory-semantic-ui-in-rust"),
            Self::Docs => locale.text("meta-get-started-glory"),
            Self::Architecture => locale.text("meta-architecture-and-design-glory"),
            Self::Reactivity => locale.text("meta-fine-grained-reactivity-glory"),
            Self::Display => locale.text("display-meta-title"),
            Self::Widgets => locale.text("meta-built-in-widgets-glory"),
            Self::CustomWidgets => locale.text("meta-writing-custom-widgets-glory"),
            Self::Styling => locale.text("meta-styling-and-assets-glory"),
            Self::I18n => locale.text("i18n-meta-title"),
            Self::Web => locale.text("meta-web-applications-glory"),
            Self::Ssr => locale.text("meta-ssr-and-hydration-glory"),
            Self::Platforms => locale.text("meta-cross-platform-applications-glory"),
            Self::NotFound => locale.text("meta-page-not-found-glory"),
        }
    }

    pub fn description(self, locale: Locale) -> String {
        match self {
            Self::Home => {
                locale.text("meta-learn-glory-s-semantic-ui-architecture-fine-grained-reactivity")
            }
            Self::Docs => {
                locale.text("meta-install-glory-build-your-first-semantic-widget-and-choose")
            }
            Self::Architecture => {
                locale.text("meta-understand-glory-s-retained-semantic-tree-typed-patches-explicit")
            }
            Self::Reactivity => {
                locale.text("meta-owned-cage-bond-and-vary-values-provide-explicit-fine")
            }
            Self::Display => locale.text("display-meta-description"),
            Self::Widgets => {
                locale.text("meta-compose-interfaces-with-glory-s-built-in-semantic-elements")
            }
            Self::CustomWidgets => {
                locale.text("meta-implement-the-widget-trait-own-reactive-state-handle-semantic")
            }
            Self::Styling => {
                locale.text("meta-use-typed-styles-external-css-and-bounded-stylesheets-across")
            }
            Self::I18n => locale.text("i18n-meta-description"),
            Self::Web => {
                locale.text("meta-launch-and-hydrate-a-glory-application-in-a-webassembly")
            }
            Self::Ssr => locale
                .text("meta-render-static-html-create-hydration-snapshots-transfer-resources-and"),
            Self::Platforms => {
                locale.text("meta-share-glory-widgets-across-web-ssr-and-blitz-desktop")
            }
            Self::NotFound => {
                locale.text("meta-the-requested-glory-documentation-page-could-not-be-found")
            }
        }
    }
}
