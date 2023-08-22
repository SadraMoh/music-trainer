use crate::types::{MajorKey, MinorKey, Note};
use leptos::*;

use super::NoteBoardLayout;

#[component]
pub fn NoteBoard(
    cx: Scope,
    #[prop(optional)] answer: Option<Note>,
    #[prop(default = NoteBoardLayout::MajorKeys)] layout: NoteBoardLayout,
) -> impl IntoView {
    let buttons: Vec<Note> = match layout {
        NoteBoardLayout::MajorKeys => MajorKey::ALL
            .iter()
            .map(|x| Into::<Note>::into(*x))
            .collect(),
        NoteBoardLayout::MinorKeys => MinorKey::ALL
            .iter()
            .map(|x| Into::<Note>::into(*x))
            .collect(),
        NoteBoardLayout::NaturalNotes => Note::ALL.iter().cloned().collect::<Vec<Note>>(),
    };

    let layout_class = match layout {
        NoteBoardLayout::MajorKeys => "layout-major-keys",
        NoteBoardLayout::MinorKeys => "layout-minor-keys",
        _ => "",
    };

    view! {
        cx,
        <div class="note-board">
            <div class=format!("note-board-grid {}", layout_class)>
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
