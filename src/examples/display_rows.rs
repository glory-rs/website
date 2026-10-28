use glory::display::prelude::*;

struct Item {
    id: usize,
    label: String,
}

pub fn rows(scope: &Scope) -> impl IntoView {
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
