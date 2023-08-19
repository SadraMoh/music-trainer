use super::scores::Scores;
use crate::{
    components::{NoteBoard, Scene},
    types::Note,
};
use leptos::*;

#[component]
pub fn KeyIdentifier(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="key-identifier">
            <Scores />
            <Scene>
                <img src="assets/icons/keys/C.svg" />
            </Scene>
            <NoteBoard answer=Note::B />
        </div>
    }
}
