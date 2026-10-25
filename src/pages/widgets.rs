use crate::Locale;
use crate::{Route, components::*};
use glory::prelude::*;
use sapid::Theme;

const IMPORTS: &str = r#"use glory::prelude::*;

// These four builders are intentionally imported from glory::ui.
use glory::ui::{file, graphic, rich_editor, select};"#;

const CORE_CONTROLS: &str = r#"use glory::prelude::*;

fn profile_editor() -> impl Widget {
    widget(|scope| {
        let bio = scope.cage(String::new());
        let public = scope.cage(false);

        view().class("profile").fill((
            text()
                .kind(TextKind::Heading(1))
                .content("Edit profile"),
            image()
                .source("/assets/avatar.png")
                .alt("Ada’s profile photo"),
            group().fill(text().content("Public information")),
            input()
                .name("Biography")
                .multiline(true)
                .value(bio)
                .on(value_changed, move |event| {
                    if let Event::ValueChanged { value, .. } = event {
                        bio.revise(|old| *old = value);
                    }
                }),
            check()
                .label("Show this profile publicly")
                .checked(public)
                .on(checked_changed, move |event| {
                    if let Event::CheckedChanged(value) = event {
                        public.revise(|old| *old = value);
                    }
                }),
            link()
                .target("/docs/")
                .label("Read the documentation"),
            row().fill(button().label("Save profile")),
        ))
        .show_in(scope)
    })
}"#;

const ADVANCED_CONTROLS: &str = r#"use glory::prelude::*;
use glory::render::{
    FileConfig, FileFilter, GraphicPoint, GraphicProps, PathCommand, RichBlock,
    RichBlockKind, RichDocument, RichMarks, RichRun, SelectEntry, SelectOption,
    SelectProps,
};
use glory::ui::{file, graphic, rich_editor, select};
use std::rc::Rc;

fn advanced_controls() -> impl Widget {
    let picker = file()
        .label("Choose images")
        .configuration(FileConfig {
            multiple: true,
            max_files: 2,
            filters: vec![
                FileFilter::Mime("image/*".into()),
                FileFilter::Extension(".png".into()),
            ],
            ..Default::default()
        });

    let rust = SemanticId::new("rust").expect("valid option key");
    let languages = select()
        .configuration(SelectProps {
            entries: Rc::new(vec![SelectEntry::Option(SelectOption {
                key: rust.clone(),
                label: "Rust".into(),
                value: "rust".into(),
                disabled: false,
            })]),
            placeholder: "Choose a language".into(),
            ..Default::default()
        })
        .selected(vec![rust]);

    let warning = graphic().drawing(GraphicProps {
        width: 24.0,
        height: 24.0,
        commands: vec![
            PathCommand::Move(GraphicPoint::new(12.0, 2.0)),
            PathCommand::Line(GraphicPoint::new(22.0, 22.0)),
            PathCommand::Line(GraphicPoint::new(2.0, 22.0)),
            PathCommand::Close,
        ],
        fill: Some(Color(240, 170, 40, 255)),
        alternative: Some("Warning".into()),
        ..Default::default()
    });

    let document = Rc::new(
        RichDocument::new(vec![RichBlock {
            kind: RichBlockKind::Paragraph,
            runs: vec![RichRun {
                text: "A bold beginning".into(),
                marks: RichMarks {
                    bold: true,
                    ..Default::default()
                },
            }],
        }])
        .expect("valid rich document"),
    );

    col().fill((
        picker,
        languages,
        warning,
        rich_editor()
            .name("Article")
            .document(document)
            .enabled(true),
    ))
}"#;

const CONTROL_FLOW: &str = r#"use glory::prelude::*;

fn results(loading: Cage<bool>, ids: Cage<Vec<u64>>) -> impl Widget {
    col().fill((
        when(loading, || text().content("Loading…"))
            .otherwise(|| text().content("Results")),
        each(ids, |id| *id, |id, index| {
            row().fill((
                text().content(index.map(|index| format!("{}. ", index + 1))),
                text().content(id.map(|id| format!("Item {id}"))),
            ))
        }),
        list(
            100_000_usize,
            |index| index as u64,
            |_, key| text().content(format!("Virtual row {key}")),
        )
        .height(240.0)
        .row_height(32.0)
        .overscan(3),
    ))
}"#;

pub fn widgets(locale: Locale) -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", locale.text("widgets-the-semantic-toolbox"))),
        Box::new(heading(1, "docs-title", locale.text("widgets-compose-with-built-in-widgets"))),
        Box::new(copy(
            "docs-lead",
            locale.text("widgets-glory-s-built-ins-are-finite-semantic-building-blocks"),
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", locale.text("widgets-know-what-is-built-in")),
            copy(
                "",
                locale.text("widgets-glory-provides-view-group-text-button-input-check-image"),
            ),
        ))),
        Box::new(heading(2, "docs-heading", locale.text("widgets-import-the-builders"))),
        Box::new(copy(
            "docs-paragraph",
            locale.text("widgets-the-prelude-includes-the-common-builders-control-flow-reactive"),
        )),
        Box::new(code_panel(locale, Theme::blue_dark(), "imports.rs", IMPORTS, "docs-code")),
        Box::new(heading(2, "docs-heading", locale.text("widgets-structure-content-and-controls"))),
        Box::new(view().class("docs-host-grid").fill((
            view().class("docs-host-card").fill((
                heading(3, "docs-card-title", locale.text("widgets-structure")),
                copy(
                    "",
                    locale.text(
                        "widgets-view-creates-a-styled-semantic-container-group-joins-logical",
                    ),
                ),
            )),
            view().class("docs-host-card").fill((
                heading(3, "docs-card-title", locale.text("widgets-content")),
                copy(
                    "",
                    locale.text("widgets-text-carries-inline-or-structured-text-image-loads-an"),
                ),
            )),
            view().class("docs-host-card").fill((
                heading(3, "docs-card-title", locale.text("widgets-input")),
                copy(
                    "",
                    locale.text("widgets-button-input-check-file-select-and-rich-editor-expose"),
                ),
            )),
        ))),
        Box::new(code_panel(locale, Theme::blue_dark(), "profile.rs", CORE_CONTROLS, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            locale.text("widgets-every-image-needs-alt-or-decorative-use-view-for"),
        )),
        Box::new(heading(
            2,
            "docs-heading",
            locale.text("widgets-files-selection-graphics-and-rich-text"),
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text(
                "widgets-advanced-controls-use-bounded-configuration-values-select-options-have",
            ),
        )),
        Box::new(code_panel(
            locale,
            Theme::blue_dark(),
            "controls.rs",
            ADVANCED_CONTROLS,
            "docs-code",
        )),
        Box::new(heading(
            2,
            "docs-heading",
            locale.text("widgets-choose-the-right-composition-tool"),
        )),
        Box::new(view().class("docs-host-grid").fill((
            view().class("docs-host-card").fill(
                (
                    heading(3, "docs-card-title", locale.text("widgets-row-col")),
                    copy(
                        "",
                        locale.text(
                            "widgets-use-these-shortcuts-for-horizontal-or-vertical-flex-layout",
                        ),
                    ),
                ),
            ),
            view().class("docs-host-card").fill((
                heading(3, "docs-card-title", "when"),
                copy("", locale.text("widgets-mount-a-branch-from-a-static-or-reactive-boolean")),
            )),
            view().class("docs-host-card").fill((
                heading(3, "docs-card-title", locale.text("widgets-each-list")),
                copy("", locale.text("widgets-each-mounts-a-complete-keyed-vec-and-gives-rows")),
            )),
        ))),
        Box::new(code_panel(
            locale,
            Theme::blue_dark(),
            "collections.rs",
            CONTROL_FLOW,
            "docs-code",
        )),
        Box::new(copy(
            "docs-paragraph",
            locale.text("widgets-use-stable-keys-in-both-collection-builders-prefer-each"),
        )),
        Box::new(doc_nav(locale, Some(Route::Reactivity), Some(Route::CustomWidgets))),
    ];
    docs_layout(locale, Route::Widgets, view().fill(content))
}
