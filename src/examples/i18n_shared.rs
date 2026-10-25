use super::i18n_catalog::catalog;
use glory::{
    i18n::{Args, I18n, Locale},
    prelude::*,
};

// Both the server and browser call this before constructing the widget.
pub fn locale_for_path(path: &str) -> Locale {
    let path = path.split(['?', '#']).next().unwrap_or(path);
    let tag = if path == "/zh" || path.starts_with("/zh/") { "zh-CN" } else { "en" };
    tag.parse().expect("static language tag")
}

pub fn language_demo(initial: Locale) -> impl Widget {
    widget(move |scope| {
        let language = scope.cage(initial.clone());
        let count = scope.cage(1_i32);
        let i18n = I18n::new(catalog(), language);
        let message = i18n
            .message(scope.owner(), "inbox", move || {
                let mut args = Args::new();
                args.set("count", count.get());
                args
            })
            .expect("live message owner");
        let add = i18n.message(scope.owner(), "add", Args::new).expect("live message owner");
        // The application chooses its error policy. These resources are tested.
        let label = scope.bond(move || message.get().expect("valid inbox translation"));
        let add_label = scope.bond(move || add.get().expect("valid action translation"));
        col()
            .class("i18n-demo")
            .fill((
                text().class("i18n-demo-message").content(label),
                row().class("i18n-demo-actions").fill((
                    button()
                        .class("counter-button")
                        .label("English")
                        .on(activate, move |_| language.revise(|id| *id = "en".parse().unwrap())),
                    button().class("counter-button").label("简体中文").on(activate, move |_| {
                        language.revise(|id| *id = "zh-CN".parse().unwrap())
                    }),
                    button()
                        .class("counter-button counter-primary")
                        .label(add_label)
                        .on(activate, move |_| count.revise(|count| *count += 1)),
                )),
            ))
            .show_in(scope)
    })
}
