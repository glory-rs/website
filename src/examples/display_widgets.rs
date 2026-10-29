use glory::display::prelude::*;
use sapid::{Button, Theme};

fn save(theme: Theme) -> impl Widget {
    sapid::button().theme(theme).label("Factory")
}

pub fn widgets(theme: Theme) -> impl Widget {
    view! {
        <div>
            <Button theme={theme.clone()} label="Imported"/>
            <sapid::Button theme={theme.clone()} label="Qualified"/>
            <{save(theme)}/>
        </div>
    }
}
