use std::fmt::Display;

use super::super::Key;
use crate::types::Note;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum MajorKey {
    F,
    C,
    G,
    D,
    A,
    E,
    B,
    CFlat,
    FSharp,
    GFlat,
    DFlat,
    CSharp,
    AFlat,
    EFlat,
    BFlat,
}

impl MajorKey {
    pub const ALL: [MajorKey; 15] = [
        MajorKey::F,
        MajorKey::C,
        MajorKey::G,
        MajorKey::D,
        MajorKey::A,
        MajorKey::E,
        MajorKey::B,
        MajorKey::CFlat,
        MajorKey::FSharp,
        MajorKey::GFlat,
        MajorKey::DFlat,
        MajorKey::CSharp,
        MajorKey::AFlat,
        MajorKey::EFlat,
        MajorKey::BFlat,
    ];
}

impl Display for MajorKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_note())
    }
}

impl Into<Key> for MajorKey {
    fn into(self) -> Key {
        Key::Major(self)
    }
}

impl MajorKey {
    pub fn into_note(self) -> Note {
        Into::<Note>::into(self)
    }
}

impl Into<Note> for MajorKey {
    fn into(self) -> Note {
        match self {
            MajorKey::F => Note::F,
            MajorKey::C => Note::C,
            MajorKey::G => Note::G,
            MajorKey::D => Note::D,
            MajorKey::A => Note::A,
            MajorKey::E => Note::E,
            MajorKey::B => Note::B,
            MajorKey::CFlat => Note::CFlat,
            MajorKey::FSharp => Note::FSharp,
            MajorKey::GFlat => Note::GFlat,
            MajorKey::DFlat => Note::DFlat,
            MajorKey::CSharp => Note::CSharp,
            MajorKey::AFlat => Note::AFlat,
            MajorKey::EFlat => Note::EFlat,
            MajorKey::BFlat => Note::BFlat,
        }
    }
}
