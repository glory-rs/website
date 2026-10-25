use crate::Locale;
use crate::{Route, components::link, components::*};
use glory::prelude::*;
use glory::render::Role;

pub fn not_found(locale: Locale) -> impl Widget {
    view().role(Role::Main).class("not-found").fill((
        copy("not-found-code", "404"),
        heading(1, "docs-title", locale.text("not-found-this-page-drifted-away")),
        copy("docs-lead", locale.text("not-found-the-address-does-not-match-a-glory-guide-or")),
        link(locale, locale.text("not-found-return-home"), Route::Home, "button button-primary"),
    ))
}
