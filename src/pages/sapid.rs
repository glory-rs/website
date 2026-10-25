use crate::{Route, components::link, components::*};
use glory::prelude::*;
use sapid::{ButtonVariant, Size, Theme};

pub fn sapid_page() -> impl Widget {
    widget(|scope| {
        let enabled = scope.cage(true);
        let theme = Theme::blue_dark();
        docs_layout(
            Route::Sapid,
            view().fill((
                copy("docs-kicker", "THE GLORY ECOSYSTEM"),
                heading(1, "docs-title", "Meet Sapid."),
                copy(
                    "docs-lead",
                    "A composable semantic widget library for Glory. Thoughtful controls, expressive themes, and accessible behavior—still handwritten in Rust.",
                ),
                view().class("sapid-live-card").fill((
                    view().fill((
                        copy("preview-kicker", "A LIVE SAPID EXAMPLE"),
                        heading(2, "preview-title", "Small details, included."),
                        copy("preview-copy", "Use real controls in your docs, dashboards, and applications."),
                    )),
                    view().class("sapid-control-row").fill((
                        sapid::switch()
                            .theme(theme.clone())
                            .name("Enable notifications")
                            .checked(enabled)
                            .on_checked_change(move |value| enabled.revise(|old| *old = value)),
                        copy("", "Notifications"),
                        sapid::button()
                            .theme(theme.clone())
                            .size(Size::Small)
                            .label("Save preferences"),
                        sapid::button()
                            .theme(theme.clone())
                            .variant(ButtonVariant::Secondary)
                            .size(Size::Small)
                            .label("Cancel"),
                    )),
                    view().class("sapid-tags").fill((
                        sapid::badge().theme(theme.clone()).content("Typed callbacks"),
                        sapid::badge().theme(theme.clone()).content("Owned state"),
                        sapid::badge().theme(theme.clone()).content("Host agnostic"),
                    )),
                )),
                heading(2, "docs-heading", "Designed to compose"),
                copy("docs-paragraph", "Sapid widgets implement Glory's Widget trait. Choose your own host, application state, layout, and visual language; the library does not install a router or runtime policy."),
                view().class("docs-host-grid").fill((
                    view().class("docs-host-card").fill((heading(3, "docs-card-title", "Forms & input"), copy("", "Text fields, checkboxes, selects, validation, editors, and typed change callbacks."))),
                    view().class("docs-host-card").fill((heading(3, "docs-card-title", "Application UI"), copy("", "Navigation, overlays, cards, tabs, accordions, feedback, and application shells."))),
                    view().class("docs-host-card").fill((heading(3, "docs-card-title", "Data & media"), copy("", "Code highlighting, JSON tools, dates, charts, files, and chat-oriented widgets."))),
                )),
                heading(2, "docs-heading", "Explore the full showcase"),
                copy("docs-paragraph", "The Sapid repository contains the complete Glory + Sapid documentation application, widget guides, source examples, theme studio, and browser acceptance suites."),
                view().class("docs-actions").fill((
                    external("Open Sapid on GitHub  ↗", "https://github.com/glory-rs/sapid", "button button-primary"),
                    link("Learn Glory reactivity", Route::Reactivity, "text-link"),
                )),
            )),
        )
        .show_in(scope)
    })
}
