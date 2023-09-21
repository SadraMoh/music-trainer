use crate::types::Rhythm;
use rand::Rng;

pub fn create_seq() -> Vec<Rhythm> {
    let mut rng = rand::thread_rng();
    let seq_length = rng.gen_range(2..=6);
    let mut seq = vec![Rhythm::Crotchet; seq_length];

    loop {
        seq.fill_with(|| rng.gen());

        // never generate sequences that begin with a rest
        if seq.first().unwrap().beats().first().unwrap().is_negative() {
            continue;
        }

        // never generate sequences that end with a rest
        if seq.last().unwrap().beats().first().unwrap().is_negative() {
            continue;
        }

        break;
    }

    seq
}
