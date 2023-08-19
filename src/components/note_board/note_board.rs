use crate::types::Note;
use leptos::*;

#[component]
pub fn NoteBoard(cx: Scope, #[prop(optional)] answer: Option<Note>) -> impl IntoView {
    let buttons = Note::ALL;

    view! {
        cx,
        <div class="note-board">
            <div class="note-board-grid">
                {move || buttons.iter().map(|button| {

                    let style = format!("grid-area: {}", button.to_string().replace("#", "s"));
                    let class = format!("{}", match answer {
                        Some(x) if x == *button => "correct",
                        Some(_) => "wrong",
                        None => "",
                    });

                    view! { cx,
                        <button tabindex=-1 class=class style=style >{*button}</button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
