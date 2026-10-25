#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Route {
    #[default]
    Home,
    Docs,
    Reactivity,
    Sapid,
    NotFound,
}

impl Route {
    pub const ALL: [Route; 4] = [Route::Home, Route::Docs, Route::Reactivity, Route::Sapid];

    pub fn parse(path: &str) -> Self {
        let path = path.split(['?', '#']).next().unwrap_or(path);
        let normalized = if path.len() > 1 { path.trim_end_matches('/') } else { path };
        match normalized {
            "" | "/" | "/index.html" => Self::Home,
            "/docs" | "/docs/index.html" => Self::Docs,
            "/docs/reactivity" | "/docs/reactivity/index.html" => Self::Reactivity,
            "/sapid" | "/sapid/index.html" => Self::Sapid,
            _ => Self::NotFound,
        }
    }

    pub fn path(self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Docs => "/docs/",
            Self::Reactivity => "/docs/reactivity/",
            Self::Sapid => "/sapid/",
            Self::NotFound => "/404.html",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Home => "Glory — Semantic UI in Rust",
            Self::Docs => "Get started — Glory",
            Self::Reactivity => "Fine-grained reactivity — Glory",
            Self::Sapid => "Sapid — Widgets for Glory",
            Self::NotFound => "Page not found — Glory",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Home => {
                "Build semantic, reactive Rust applications with handwritten builders and replaceable rendering hosts."
            }
            Self::Docs => {
                "Learn how to build and run a Glory application for Web, SSR, and native hosts."
            }
            Self::Reactivity => {
                "Owned Cage, Bond, and Lotus values provide explicit fine-grained reactivity in Glory."
            }
            Self::Sapid => "Sapid is a composable semantic widget library for Glory applications.",
            Self::NotFound => "The requested Glory documentation page could not be found.",
        }
    }
}
