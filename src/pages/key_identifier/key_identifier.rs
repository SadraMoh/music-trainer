use super::scores::Scores;
use crate::components::{NoteBoard, Scene};
use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn KeyIdentifier(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="flex flex-col align-center">
            <Scores />
            <Scene>
                "This is a scene"
            </Scene>
            <NoteBoard />
        </div>
    }
}
