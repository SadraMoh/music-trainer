use super::create_seq;
use super::Sequence;
use crate::{
    components::{Icon, RhythmBoard, Scene, Scores},
    types::Rhythm,
};
use leptos::*;

#[component]
pub fn RhythmIdentifier(cx: Scope) -> impl IntoView {
    let (seq, _) = create_signal::<Sequence>(cx, create_seq());
    let seq_beats = move || {
        seq()
            .iter()
            .flat_map(|r| r.beats().to_owned())
            .collect::<Vec<_>>()
    };

    let (id_counter, set_id_counter) = create_signal::<u128>(cx, 0);
    let (picked, set_picked) = create_signal(cx, Vec::<(u128, Rhythm)>::new());

    let is_correct = move || {
        let a = picked()
            .iter()
            .flat_map(|r| r.1.beats().to_owned())
            .collect::<Vec<_>>();

        a.eq(&seq_beats())
    };

    let add_rhythm = move |rhythm: Rhythm| {
        set_id_counter.update(|prev| *prev += 1);
        set_picked.update(|prev| prev.push((id_counter(), rhythm)));
    };

    let remove_rhythm = move |id_to_be_deleted: u128| {
        set_picked.update(|prev| {
            prev.retain(|(id, _)| *id != id_to_be_deleted);
        })
    };

    let play_audio = move || super::player::play_seq(seq());

    view! {
        cx,
        <div class="rhythm-identifier">
            <Scores correct_count={MaybeSignal::Static(1)} wrong_count={MaybeSignal::Static(2)}  />
            <button class="play-audio" on:click=move |_| { play_audio().unwrap_or_default() }>
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
            { move || if is_correct() { "correct!" } else {""}.to_string() }
            <RhythmBoard on_note=add_rhythm />
        </div>
    }
}
