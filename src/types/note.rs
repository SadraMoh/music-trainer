use std::fmt::Display;

use leptos::IntoView;

#[derive(Clone, Copy, PartialEq)]
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    DFlat,
    E,
    EFlat,
    F,
    FSharp,
    G,
    GSharp,
    GFlat,
    A,
    ASharp,
    AFlat,
    B,
    BFlat,
}

impl Note {
    pub const ALL: [Note; 17] = [
        Note::C,
        Note::CSharp,
        Note::D,
        Note::DSharp,
        Note::DFlat,
        Note::E,
        Note::EFlat,
        Note::F,
        Note::FSharp,
        Note::G,
        Note::GSharp,
        Note::GFlat,
        Note::A,
        Note::ASharp,
        Note::AFlat,
        Note::B,
        Note::BFlat,
    ];

    pub const NATURALS: [Note; 7] = [
        Note::C,
        Note::D,
        Note::E,
        Note::F,
        Note::G,
        Note::A,
        Note::B,
    ];

    pub fn is_natural(&self) -> bool {
        Self::NATURALS.contains(self)
    }

    pub const SHARPS: [Note; 5] = [
        Note::CSharp,
        Note::DSharp,
        Note::FSharp,
        Note::GSharp,
        Note::ASharp,
    ];

    pub fn is_sharp(&self) -> bool {
        Self::SHARPS.contains(self)
    }

    pub const FLATS: [Note; 5] = [
        Note::DFlat,
        Note::EFlat,
        Note::GFlat,
        Note::AFlat,
        Note::BFlat,
    ];

    pub fn is_flat(&self) -> bool {
        Self::FLATS.contains(self)
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Note::C => "C",
                Note::CSharp => "C#",
                Note::D => "D",
                Note::DSharp => "D#",
                Note::DFlat => "Db",
                Note::E => "E",
                Note::EFlat => "Eb",
                Note::F => "F",
                Note::FSharp => "F#",
                Note::G => "G",
                Note::GSharp => "G#",
                Note::GFlat => "Gb",
                Note::A => "A",
                Note::ASharp => "A#",
                Note::AFlat => "Ab",
                Note::B => "B",
                Note::BFlat => "Bb",
            }
        )
    }
}

impl IntoView for Note {
    fn into_view(self, cx: leptos::Scope) -> leptos::View {
        self.to_string().into_view(cx)
    }
}
