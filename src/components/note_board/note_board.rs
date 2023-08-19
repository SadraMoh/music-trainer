use crate::types::Note;
use leptos::*;

#[component]
pub fn NoteBoard(cx: Scope) -> impl IntoView {
    let buttons = Note::all();

    view! {
        cx,
        <div class="note-board">
            <div class="note-board-grid">
                {move || buttons.iter().map(|button| view! { cx,
                    <button tabindex=-1 style=format!("grid-area: {}", button.to_string().replace("#", "s"))>{button.to_string()}</button>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
