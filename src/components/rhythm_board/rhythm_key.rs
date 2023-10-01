use crate::types::Rhythm;
use leptos::*;

#[component]
pub fn RhythmKey<F>(cx: Scope, rhythm: MaybeSignal<Rhythm>, on_click: F) -> impl IntoView
where
    F: Fn(Rhythm) + 'static,
{
    let handle_click = move |_| on_click(rhythm());

    view! {
        cx,
        <button on:click=handle_click >
          <img src=move || rhythm().icon_url() />
        </button>
    }
}
