use glory::prelude::*;

pub fn hello() -> impl Widget {
    col().fill((
        text().kind(TextKind::Heading(1)).content("Hello, Glory"),
        text().content("Semantic UI, handwritten in Rust."),
    ))
}
