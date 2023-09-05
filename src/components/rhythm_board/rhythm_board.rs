use super::RhythmKey;
use crate::types::Rhythm;
use leptos::{MaybeSignal::Static, *};
use std::rc::Rc;

#[component]
pub fn RhythmBoard(cx: Scope) -> impl IntoView {
    let on_click = move |_rhythm: Rhythm| {};

    let on_click = Rc::new(on_click);

    view! {
        cx,
        <div class="rhythm-board">
          <div class="rests">
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::HalfRest) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::QuarterRest) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::EighthRest) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::SixthteenthRest) />
          </div>
          <div class="singles">
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::Semibreve) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::Minim) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::Crotchet) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::Quaver) />
          </div>
          <div class="doubles">
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::QuaverQuaver) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::QuaverSemiquaver) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::QuaverSemiquaverDot) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::QuaverDotSemiquaver) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::SemiQuaverSemiquaver) />
          </div>
          <div class="multiples">
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::QuaverSemiquaverSemiquaver) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::SemiquaverSemiquaverQuaver) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::SemiquaverQuaverSemiquaver) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::QuaverQuaverQuaver) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::SemiquaverSemiquaverSemiquaverSemiquaver) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::QuaverSemiquaverSemiquaverQuaver) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::SemiquaverSemiquaverQuaverQuaver) />
            <RhythmKey on_click=*on_click rhythm=Static(Rhythm::QuaverQuaverSemiquaverSemiquaver) />
          </div>
        </div>
    }
}
