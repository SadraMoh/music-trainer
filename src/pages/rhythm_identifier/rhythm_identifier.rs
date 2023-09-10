use crate::{
    components::{Icon, RhythmBoard, Scene, Scores},
    types::Rhythm,
};
use leptos::*;

#[component]
pub fn RhythmIdentifier(cx: Scope) -> impl IntoView {
    let (id_counter, set_id_counter) = create_signal::<u128>(cx, 0);
    let (picked, set_picked) = create_signal(cx, Vec::<(u128, Rhythm)>::new());

    let add_rhythm = move |rhythm: Rhythm| {
        set_id_counter.update(|prev| *prev += 1);
        set_picked.update(|prev| prev.push((id_counter(), rhythm)));
    };

    let remove_rhythm = move |id_to_be_deleted: u128| {
        set_picked.update(|prev| {
            prev.retain(|(id, _)| *id != id_to_be_deleted);
        })
    };

    view! {
        cx,
        <div class="rhythm-identifier">
            <Scores correct_count={MaybeSignal::Static(1)} wrong_count={MaybeSignal::Static(2)}  />
            <button class="play-audio">
                <Icon name="play_arrow" />
            </button>
            <Scene>
                <For each=move || picked() key=|(id, _)| { *id } view=move |cx, (id, rhythm)| {
                    let rhythm = rhythm.to_owned();

                    view! {
                        cx,
                        <button class="rhythm" on:click=move |_| remove_rhythm(id)>
                            <img src=move || rhythm.icon_url() />
                        </button>
                    }
                } />
            </Scene>
            <RhythmBoard on_note=add_rhythm />
        </div>
    }
}
