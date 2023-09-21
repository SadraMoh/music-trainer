use crate::{
    components::{Icon, RhythmBoard, Scene, Scores},
    types::Rhythm,
};
use leptos::*;
use rand::Rng;

type Sequence = Vec<Rhythm>;

#[component]
pub fn RhythmIdentifier(cx: Scope) -> impl IntoView {
    let create_seq = || {
        let mut rng = rand::thread_rng();
        let seq_length = rng.gen_range(2..=6);
        let mut seq = vec![Rhythm::Crotchet; seq_length];
        seq.fill_with(|| rng.gen());

        seq
    };

    let (seq, set_seq) = create_signal::<Sequence>(cx, create_seq());
    let seq_beats = move || {
        seq()
            .iter()
            .flat_map(|r| r.beats().to_owned())
            .collect::<Vec<_>>()
    };

    let (id_counter, set_id_counter) = create_signal::<u128>(cx, 0);
    let (picked, set_picked) = create_signal(cx, Vec::<(u128, Rhythm)>::new());

    let add_rhythm = move |rhythm: Rhythm| {
        set_id_counter.update(|prev| *prev += 1);
        set_picked.update(|prev| prev.push((id_counter(), rhythm)));

        let a = picked()
            .iter()
            .flat_map(|r| r.1.beats().to_owned())
            .collect::<Vec<_>>();

        log!("{:?} {:?}, {:?}", a, seq_beats(), a.eq(&seq_beats()));
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
