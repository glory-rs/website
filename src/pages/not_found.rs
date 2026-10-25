use crate::{Route, components::link, components::*};
use glory::prelude::*;
use glory::render::Role;

pub fn not_found() -> impl Widget {
    view().role(Role::Main).class("not-found").fill((
        copy("not-found-code", "404"),
        heading(1, "docs-title", "This page drifted away."),
        copy("docs-lead", "The address does not match a Glory guide or ecosystem page."),
        link("Return home  →", Route::Home, "button button-primary"),
    ))
}
