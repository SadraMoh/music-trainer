use super::Sequence;
use crate::types::Rhythm;
use rand::Rng;

pub fn create_seq() -> Sequence {
    let mut rng = rand::thread_rng();
    let seq_length = rng.gen_range(2..=6);
    let mut seq = vec![Rhythm::Crotchet; seq_length];
    let beats = seq
        .iter()
        .flat_map(|r| r.beats().to_owned())
        .collect::<Vec<_>>();

    loop {
        seq.fill_with(|| rng.gen());

        // never generate sequences that begin with a rest
        if beats
            .first()
            .expect("sequence should have at least one element")
            .is_negative()
        {
            continue;
        }

        // never generate sequences that end with a rest
        if beats
            .last()
            .expect("sequence should have at least one element")
            .is_negative()
        {
            continue;
        }

        // never generate sequences with consecutive rests
        if beats.windows(2).any(|f| match f {
            [x, y] if x.is_negative() && y.is_negative() => true,
            _ => false,
        }) {
            continue;
        }

        break;
    }

    seq
}
