use crate::components::{Icon, RhythmBoard, Scene, Scores};
use leptos::*;

#[component]
pub fn RhythmIdentifier(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="rhythm-identifier">
            <Scores correct_count={MaybeSignal::Static(1)} wrong_count={MaybeSignal::Static(2)}  />
            <button class="play-audio">
                <Icon name="play_arrow" />
            </button>
            <Scene>
                <div />
            </Scene>
            <RhythmBoard />
        </div>
    }
}
