use super::Rhythm;
use rand::{distributions::Standard, prelude::*};

impl Distribution<Rhythm> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rhythm {
        let index = rng.gen_range(0..Rhythm::ALL.len());
        Rhythm::ALL
            .get(index)
            .expect("random number generator generates indexes within bounds")
            .clone()
    }
}
