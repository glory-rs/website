use glory::display::prelude::*;

const LABEL: Field = Field::new("label");
static LABEL_VIEW: Template = Template::new(&[span(&[], &[field(LABEL)])]);

pub fn label(scope: &Scope) -> impl IntoView {
    let value = scope.cage(String::from("Hello"));
    view(&LABEL_VIEW).bind(LABEL, value)
}
