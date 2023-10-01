use super::Sequence;
use leptos::log;
use wasm_bindgen::prelude::*;

/// how long a single thirtytwoth note should be
const CONST: f64 = 0.06;
/// how long the pause between notes should be
const TWEEN: f64 = 0.09;
/// duration of ramp
const RAMPT: f64 = 0.01;

pub fn play_seq(seq: Sequence, frequency: Option<f32>) -> Result<(), JsValue> {
    let frequency = frequency.unwrap_or(440.);

    let ctx = web_sys::AudioContext::new()?;
    let now = ctx.current_time();

    let osc = ctx.create_oscillator()?;
    osc.frequency().set_value(frequency);
    osc.start()?;

    let gain = ctx.create_gain()?;
    gain.gain().set_value(0.);

    osc.connect_with_audio_node(&gain)?;
    gain.connect_with_audio_node(&ctx.destination())?;

    let mut aggr: f64 = 0.;
    for note in seq.iter().flat_map(|r| r.beats()) {
        let duration: f64 = CONST * (note.abs() as f64);

        if note.is_positive() {
            // volume graph
            //     _______
            //    /       \
            // __/         \__

            gain.gain().set_value_at_time(0., now + aggr).unwrap();

            gain.gain()
                .set_value_at_time(1., now + aggr + RAMPT)
                .unwrap();

            gain.gain()
                .set_value_at_time(1., now + aggr + duration - RAMPT)
                .unwrap();

            gain.gain()
                .linear_ramp_to_value_at_time(0., now + aggr + duration)
                .unwrap();
        }

        aggr += duration + TWEEN;
    }

    Ok(())
}
