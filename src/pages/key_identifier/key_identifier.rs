use super::scores::Scores;
use crate::{
    components::{NoteBoard, Scene},
    types::{Key, MajorKey, Note},
};
use leptos::*;

#[component]
pub fn KeyIdentifier(cx: Scope) -> impl IntoView {
    let (question, set_question) = create_signal::<Key>(cx, Key::Major(MajorKey::C));
    let question_icon = move || match question() {
        Key::Major(x) => x.icon_url(),
        Key::Minor(x) => x.icon_url(),
    };

    view! {
        cx,
        <div class="key-identifier">
            <Scores />
            <Scene>
                <img src=question_icon />
            </Scene>
            <NoteBoard answer=Note::C />
        </div>
    }
}
