use super::RhythmKey;
use crate::types::Rhythm;
use leptos::{MaybeSignal::Static, *};
use std::rc::Rc;

#[component]
pub fn RhythmBoard<F>(cx: Scope, on_note: F) -> impl IntoView
where
    F: Fn(Rhythm) + 'static,
{
    let on_click = Rc::new(on_note);

    let next_handler = move || {
        let handler = on_click.clone();
        move |rhythm: Rhythm| (handler)(rhythm)
    };

    view! {
        cx,
        <div class="rhythm-board">
          <div class="rests">
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::HalfRest) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::QuarterRest) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::EighthRest) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::SixthteenthRest) />
          </div>
          <div class="singles">
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::Semibreve) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::Minim) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::Crotchet) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::Quaver) />
          </div>
          <div class="doubles">
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::QuaverQuaver) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::QuaverSemiquaver) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::QuaverSemiquaverDot) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::QuaverDotSemiquaver) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::SemiQuaverSemiquaver) />
          </div>
          <div class="multiples">
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::QuaverSemiquaverSemiquaver) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::SemiquaverSemiquaverQuaver) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::SemiquaverQuaverSemiquaver) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::QuaverQuaverQuaver) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::SemiquaverSemiquaverSemiquaverSemiquaver) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::QuaverSemiquaverSemiquaverQuaver) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::SemiquaverSemiquaverQuaverQuaver) />
            <RhythmKey on_click=next_handler() rhythm=Static(Rhythm::QuaverQuaverSemiquaverSemiquaver) />
          </div>
        </div>
    }
}
