use glory::display::prelude::*;

struct Item {
    id: usize,
    label: String,
}

pub fn rows(scope: &Scope) -> impl Widget {
    let items = scope
        .keyed_list(vec![Item { id: 1, label: "First".into() }], |item| item.id)
        .expect("unique keys");
    let visible = scope.cage(true);
    view! {
        <div>
            <button on:click={move || visible.revise(|v| *v = !*v)}>"Toggle"</button>
            <button on:click={move || {
                items.update_key(&1, |item| item.label.push('!')).unwrap();
            }}>"Edit first"</button>
            <Show when={move || visible.get()}>
                <Each each={items} children={move |row| {
                    let label = field!(row.label);
                    view! { <span>{label}</span> }
                }}/>
            </Show>
        </div>
    }
}

pub fn vector_rows(scope: &Scope) -> impl Widget {
    let values = glory::Vary::from(scope.cage(vec![(1_usize, String::from("First"))]));
    view! {
        <div>
            <Each each={values} key={|item| item.0} children={|row| {
                let label = row.field("label", |item| &item.1, |item| &mut item.1);
                view! { <span>{label}{row.index}</span> }
            }}/>
        </div>
    }
}
