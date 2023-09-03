use std::rc::Rc;

use super::NoteBoardLayout;
use crate::types::{MajorKey, MinorKey, Note};
use leptos::*;

#[component]
pub fn NoteBoard<F>(
    cx: Scope,
    on_note: F,
    #[prop(optional)] answer: Option<MaybeSignal<Note>>,
    #[prop(default = MaybeSignal::Static(NoteBoardLayout::MajorKeys))] layout: MaybeSignal<
        NoteBoardLayout,
    >,
) -> impl IntoView
where
    F: Fn(Note) + 'static,
{
    let buttons = move || match layout() {
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

    let layout_class = move || match layout() {
        NoteBoardLayout::MajorKeys => "layout-major-keys",
        NoteBoardLayout::MinorKeys => "layout-minor-keys",
        _ => "layout-chromatic",
    };

    let on_note = Rc::new(on_note);

    let a = "john";

    log!("{}", a);

    on_cleanup(cx, move || log!("HELLO {}", a));

    view! {
        cx,
        <div class="note-board">
            <div class="layout-indicator">
                <span class=move || format!("major {}", if layout() == NoteBoardLayout::MajorKeys { "open" } else { "" })>"major"</span>
                <span class=move || format!("minor {}", if layout() == NoteBoardLayout::MinorKeys { "open" } else { "" })>"minor"</span>
            </div>
            <div class="board">
                <div class=move || format!("board-grid {}", layout_class())>
                    {move || buttons().iter().map(|button| {
                        let note = *button;
                        let local_on_note = on_note.clone();

                        let style = format!("grid-area: {}", note.to_string().replace("#", "s"));
                        let class = format!("{}", match answer {
                            Some(x) if x() == note => "correct",
                            Some(_) => "wrong",
                            None => "",
                        });

                        view! { cx,
                            <button on:click=move |_| { (local_on_note)(note); } tabindex=-1 class=class style=style >{note}</button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
