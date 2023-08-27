use super::scores::Scores;
use crate::{
    components::{NoteBoard, NoteBoardLayout, Scene},
    types::{Key, MajorKey, Note},
};
use leptos::*;
use rand::prelude::*;
use MaybeSignal::*;

#[component]
pub fn KeyIdentifier(cx: Scope) -> impl IntoView {
    let (question, set_question) = create_signal::<Key>(cx, Key::Major(MajorKey::C));
    let question_icon = move || match question() {
        Key::Major(x) => x.icon_url(),
        Key::Minor(x) => x.icon_url(),
    };

    let (correct_count, set_correct_count) = create_signal::<u128>(cx, 0);
    let (wrong_count, set_wrong_count) = create_signal::<u128>(cx, 0);

    let layout = move || NoteBoardLayout::from(question());

    let key_note = move || question().into_note();

    let new_key = move |chosen_note: Note| {
        if chosen_note == key_note() {
            set_correct_count
        } else {
            set_wrong_count
        }
        .update(|prev| *prev += 1);

        let mut rng = rand::thread_rng();
        set_question.set(loop {
            let new_key = rng.gen::<Key>();
            if new_key != question() {
                break new_key;
            }
        });
    };

    view! {
        cx,
        <div class="key-identifier">
            <Scores correct_count=Dynamic(correct_count.derive_signal(cx)) wrong_count=Dynamic(wrong_count.derive_signal(cx)) />
            <Scene>
                <img src=move || question_icon() />
            </Scene>
            <NoteBoard on_note=new_key answer=Dynamic(key_note.derive_signal(cx)) layout=Dynamic(layout.derive_signal(cx)) />
        </div>
    }
}
