use glory::prelude::*;

pub fn save_action() -> impl Widget {
    view()
        .class("save-action")
        .fill(sapid::button().theme(sapid::Theme::blue_dark()).label("Save changes"))
}
