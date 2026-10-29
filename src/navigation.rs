use crate::Route;

pub const SAPID_URL: &str = "https://sapid.taidge.org/";
pub const API_URL: &str =
    "https://github.com/glory-rs/glory/tree/v0.6.0/crates";

pub struct Chapter {
    pub route: Route,
    pub label: &'static str,
}

pub struct DocGroup {
    pub title: &'static str,
    pub description: &'static str,
    pub chapters: &'static [Chapter],
}

// The sidebar and documentation overview share this reading order.
// Global navigation represents the documentation section, never a chapter.
pub const DOC_GROUPS: &[DocGroup] = &[
    DocGroup {
        title: "common-start-here",
        description: "navigation-start-description",
        chapters: &[
            Chapter { route: Route::Docs, label: "common-getting-started" },
            Chapter { route: Route::Architecture, label: "common-architecture" },
        ],
    },
    DocGroup {
        title: "common-core-concepts",
        description: "navigation-concepts-description",
        chapters: &[Chapter { route: Route::Reactivity, label: "common-reactivity" }],
    },
    DocGroup {
        title: "navigation-building-interfaces",
        description: "navigation-building-description",
        chapters: &[
            Chapter { route: Route::Display, label: "display-nav" },
            Chapter { route: Route::Widgets, label: "common-built-in-widgets" },
            Chapter { route: Route::CustomWidgets, label: "common-custom-widgets" },
            Chapter { route: Route::Styling, label: "common-styling" },
        ],
    },
    DocGroup {
        title: "navigation-application-guides",
        description: "navigation-guides-description",
        chapters: &[
            Chapter { route: Route::I18n, label: "i18n-nav" },
            Chapter { route: Route::Web, label: "common-web" },
            Chapter { route: Route::Ssr, label: "common-ssr-hydration" },
        ],
    },
    DocGroup {
        title: "navigation-reference",
        description: "navigation-reference-description",
        chapters: &[Chapter { route: Route::Platforms, label: "common-platforms" }],
    },
];

pub fn is_documentation(route: Route) -> bool {
    DOC_GROUPS.iter().any(|group| group.chapters.iter().any(|chapter| chapter.route == route))
}
