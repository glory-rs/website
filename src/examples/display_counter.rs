use glory::display::prelude::*;

#[component]
pub fn Counter(scope: &Scope, initial: usize) -> impl IntoView {
    let count = scope.cage(initial);
    view! {
        <div class="counter">
            <span>{count}</span>
            <button on:click={move || count.revise(|n| *n += 1)}>"+"</button>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
pub fn launch() -> Result<App> {
    // The document contains an empty <main id="app"></main>.
    // Call from your wasm-bindgen start function and retain the returned App.
    App::launch::<Counter>("app", CounterProps { initial: 0 })
}
