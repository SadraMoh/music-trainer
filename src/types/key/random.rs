use rand::{distributions::Standard, prelude::*};

use super::{Key, MajorKey, MinorKey};

impl Distribution<MajorKey> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MajorKey {
        let index = rng.gen_range(0..MajorKey::ALL.len());
        MajorKey::ALL
            .get(index)
            .expect("random number generator generates indexes within bounds")
            .clone()
    }
}

impl Distribution<MinorKey> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MinorKey {
        let index = rng.gen_range(0..MinorKey::ALL.len());
        MinorKey::ALL
            .get(index)
            .expect("random number generator generates indexes within bounds")
            .clone()
    }
}

impl Distribution<Key> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Key {
        let index = rng.gen_range(0..2);
        match index {
            0 => rng.gen::<MajorKey>().into(),
            1 => rng.gen::<MinorKey>().into(),
            _ => unreachable!(),
        }
    }
}
