use std::fmt::Display;

use super::super::Key;
use crate::types::Note;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum MinorKey {
    A,
    E,
    B,
    FSharp,
    CSharp,
    GSharp,
    AFlat,
    DSharp,
    EFlat,
    BFlat,
    ASharp,
    F,
    C,
    G,
    D,
}

impl MinorKey {
    pub const ALL: [MinorKey; 15] = [
        MinorKey::A,
        MinorKey::E,
        MinorKey::B,
        MinorKey::FSharp,
        MinorKey::CSharp,
        MinorKey::GSharp,
        MinorKey::AFlat,
        MinorKey::DSharp,
        MinorKey::EFlat,
        MinorKey::BFlat,
        MinorKey::ASharp,
        MinorKey::F,
        MinorKey::C,
        MinorKey::G,
        MinorKey::D,
    ];
}

impl Display for MinorKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_note())
    }
}

impl Into<Key> for MinorKey {
    fn into(self) -> Key {
        Key::Minor(self)
    }
}

impl MinorKey {
    pub fn into_note(self) -> Note {
        Into::<Note>::into(self)
    }
}

impl Into<Note> for MinorKey {
    fn into(self) -> Note {
        match self {
            MinorKey::A => Note::A,
            MinorKey::E => Note::E,
            MinorKey::B => Note::B,
            MinorKey::FSharp => Note::FSharp,
            MinorKey::CSharp => Note::CSharp,
            MinorKey::GSharp => Note::GSharp,
            MinorKey::AFlat => Note::AFlat,
            MinorKey::DSharp => Note::DSharp,
            MinorKey::EFlat => Note::EFlat,
            MinorKey::BFlat => Note::BFlat,
            MinorKey::ASharp => Note::ASharp,
            MinorKey::F => Note::F,
            MinorKey::C => Note::C,
            MinorKey::G => Note::G,
            MinorKey::D => Note::D,
        }
    }
}
