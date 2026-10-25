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

pub fn widgets() -> impl Widget {
    let content: Vec<Box<dyn Widget>> = vec![
        Box::new(copy("docs-kicker", "THE SEMANTIC TOOLBOX")),
        Box::new(heading(1, "docs-title", "Compose with built-in widgets.")),
        Box::new(copy(
            "docs-lead",
            "Glory's built-ins are finite semantic building blocks. Write ordinary Rust builders, connect owned state, and let the selected host supply layout, input, and presentation.",
        )),
        Box::new(view().class("docs-callout").fill((
            copy("docs-callout-title", "Know what is built in"),
            copy("", "Glory provides view/group/text/button/input/check/image/link/file/select/graphic/rich_editor, plus row/col/when/each/list. Cards and tabs are Sapid widgets, not Glory built-ins. There is no textarea builder: use input().multiline(true)."),
        ))),
        Box::new(heading(2, "docs-heading", "Import the builders")),
        Box::new(copy(
            "docs-paragraph",
            "The prelude includes the common builders, control flow, reactive values, events, and typed styles. Import file, select, graphic, and rich_editor explicitly from glory::ui; their configuration types live under glory::render.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "imports.rs", IMPORTS, "docs-code")),
        Box::new(heading(2, "docs-heading", "Structure, content, and controls")),
        Box::new(view().class("docs-host-grid").fill((
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "Structure"), copy("", "view creates a styled semantic container. group joins logical children without adding a layout box. row and col are flex-configured views."))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "Content"), copy("", "text carries inline or structured text, image loads an image with explicit alternative text, link targets a location, and graphic draws bounded vector paths."))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "Input"), copy("", "button, input, check, file, select, and rich_editor expose semantic properties and typed events rather than DOM elements."))),
        ))),
        Box::new(code_panel(Theme::blue_dark(), "profile.rs", CORE_CONTROLS, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "Every image needs alt(...) or decorative(). Use view for structural roles and physical queries; group is boxless. Link uses target(...), and controls accept static values or Lotus-compatible reactive handles.",
        )),
        Box::new(heading(2, "docs-heading", "Files, selection, graphics, and rich text")),
        Box::new(copy(
            "docs-paragraph",
            "Advanced controls use bounded configuration values. Select options have stable SemanticId keys, file pickers declare filters and budgets, graphics contain finite path commands, and rich editors consume a shared RichDocument.",
        )),
        Box::new(code_panel(Theme::blue_dark(), "controls.rs", ADVANCED_CONTROLS, "docs-code")),
        Box::new(heading(2, "docs-heading", "Choose the right composition tool")),
        Box::new(view().class("docs-host-grid").fill((
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "row / col"), copy("", "Use these shortcuts for horizontal or vertical flex layout. A later style call replaces their prepared Style, so redeclare display and flex_direction when replacing it."))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "when"), copy("", "Mount a branch from a static or reactive boolean. otherwise supplies the alternate branch; changing branches disposes the old branch's owned work."))),
            view().class("docs-host-card").fill((heading(3, "docs-card-title", "each / list"), copy("", "each mounts a complete keyed Vec and gives rows reactive item/index handles. list indexes a large source lazily and mounts only its visible window, overscan, and at most one focused row."))),
        ))),
        Box::new(code_panel(Theme::blue_dark(), "collections.rs", CONTROL_FLOW, "docs-code")),
        Box::new(copy(
            "docs-paragraph",
            "Use stable keys in both collection builders. Prefer each for small complete collections; use list with a bounded key lookup and fixed or measured row heights for large scrollable data.",
        )),
        Box::new(doc_nav(Some(Route::Reactivity), Some(Route::CustomWidgets))),
    ];
    docs_layout(Route::Widgets, view().fill(content))
}
