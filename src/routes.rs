#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Route {
    #[default]
    Home,
    Docs,
    Architecture,
    Reactivity,
    Widgets,
    CustomWidgets,
    Styling,
    Web,
    Ssr,
    Platforms,
    NotFound,
}

impl Route {
    pub const ALL: [Route; 10] = [
        Route::Home,
        Route::Docs,
        Route::Architecture,
        Route::Reactivity,
        Route::Widgets,
        Route::CustomWidgets,
        Route::Styling,
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
            "/docs/widgets" | "/docs/widgets/index.html" => Self::Widgets,
            "/docs/custom-widgets" | "/docs/custom-widgets/index.html" => Self::CustomWidgets,
            "/docs/styling" | "/docs/styling/index.html" => Self::Styling,
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
            Self::Widgets => "/docs/widgets/",
            Self::CustomWidgets => "/docs/custom-widgets/",
            Self::Styling => "/docs/styling/",
            Self::Web => "/docs/web/",
            Self::Ssr => "/docs/ssr/",
            Self::Platforms => "/docs/platforms/",
            Self::NotFound => "/404.html",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Home => "Glory — Semantic UI in Rust",
            Self::Docs => "Get started — Glory",
            Self::Architecture => "Architecture and design — Glory",
            Self::Reactivity => "Fine-grained reactivity — Glory",
            Self::Widgets => "Built-in widgets — Glory",
            Self::CustomWidgets => "Writing custom widgets — Glory",
            Self::Styling => "Styling and assets — Glory",
            Self::Web => "Web applications — Glory",
            Self::Ssr => "SSR and hydration — Glory",
            Self::Platforms => "Cross-platform applications — Glory",
            Self::NotFound => "Page not found — Glory",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Home => {
                "Learn Glory's semantic UI architecture, fine-grained reactivity, widgets, SSR, and cross-platform rendering in Rust."
            }
            Self::Docs => {
                "Install Glory, build your first semantic widget, and choose a learning path."
            }
            Self::Architecture => {
                "Understand Glory's retained semantic tree, typed patches, explicit ownership, and replaceable rendering hosts."
            }
            Self::Reactivity => {
                "Owned Cage, Bond, and Lotus values provide explicit fine-grained reactivity in Glory."
            }
            Self::Widgets => {
                "Compose interfaces with Glory's built-in semantic elements, control flow, and virtual lists."
            }
            Self::CustomWidgets => {
                "Implement the Widget trait, own reactive state, handle semantic events, and compose children in Rust."
            }
            Self::Styling => {
                "Use typed styles, external CSS, and bounded stylesheets across Glory rendering hosts."
            }
            Self::Web => "Launch and hydrate a Glory application in a WebAssembly browser host.",
            Self::Ssr => {
                "Render static HTML, create hydration snapshots, transfer resources, and stream Glory applications."
            }
            Self::Platforms => {
                "Share Glory widgets across Web, SSR, and Blitz desktop hosts with explicit platform boundaries."
            }
            Self::NotFound => "The requested Glory documentation page could not be found.",
        }
    }
}
