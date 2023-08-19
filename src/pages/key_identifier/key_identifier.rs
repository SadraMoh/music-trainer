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
        <div class="flex flex-col align-center">
            <Scores />
            <Scene>
                "This is a scene"
            </Scene>
            <NoteBoard answer=Note::B />
        </div>
    }
}
