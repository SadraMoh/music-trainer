use crate::types::Rhythm;

mod player;
mod randomizer;
mod rhythm_identifier;

pub type Sequence = Vec<Rhythm>;

pub use {randomizer::create_seq, rhythm_identifier::RhythmIdentifier};
