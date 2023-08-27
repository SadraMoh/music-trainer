use super::scores::Scores;
use crate::{
    components::{NoteBoard, NoteBoardLayout, Scene},
    types::Key,
};
use leptos::*;
use rand::prelude::*;
use MaybeSignal::*;

#[component]
pub fn KeyIdentifier(cx: Scope) -> impl IntoView {
    let mut rng = rand::thread_rng();

    let (question, set_question) = create_signal::<Key>(cx, rng.gen::<Key>());
    let question_icon = move || match question() {
        Key::Major(x) => x.icon_url(),
        Key::Minor(x) => x.icon_url(),
    };

    let new_key = move |_| {
        set_question.set(loop {
            let new_key = rng.gen::<Key>();
            if new_key != question() {
                break new_key;
            }
        });
    };

    let layout = move || NoteBoardLayout::from(question());

    let key_note = move || question().into_note();

    let on_note = |_| {
        log!("hello from identifier");
    };

    view! {
        cx,
        <div class="key-identifier">
            <Scores />
            <button class="dummy" on:click=new_key>{key_note}</button>
            <Scene>
                <img src=question_icon />
            </Scene>
            <NoteBoard on_note=on_note answer=Dynamic(key_note.derive_signal(cx)) layout=Dynamic(layout.derive_signal(cx)) />
        </div>
    }
}
