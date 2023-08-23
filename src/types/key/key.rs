use super::{major_key::MajorKey, minor_key::MinorKey};
use crate::types::Note;
use std::fmt::Display;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Key {
    Major(MajorKey),
    Minor(MinorKey),
}

impl Into<Note> for Key {
    fn into(self) -> Note {
        match self {
            Key::Major(x) => x.into_note(),
            Key::Minor(x) => x.into_note(),
        }
    }
}

impl Key {
    pub fn into_note(self) -> Note {
        Into::<Note>::into(self)
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Key::Major(x) => x.to_string(),
                Key::Minor(x) => x.to_string(),
            }
        )
    }
}
