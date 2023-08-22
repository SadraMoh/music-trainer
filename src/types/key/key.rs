use crate::types::Note;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Key {
    Major(MajorKey),
    Minor(MinorKey),
}

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

impl Into<Key> for MajorKey {
    fn into(self) -> Key {
        Key::Major(self)
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

impl Into<Key> for MinorKey {
    fn into(self) -> Key {
        Key::Minor(self)
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
