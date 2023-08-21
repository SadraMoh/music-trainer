use leptos::IntoView;
use std::fmt::Display;

use Note::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Note {
    C,
    CSharp,
    CFlat,
    D,
    DSharp,
    DFlat,
    E,
    ESharp,
    EFlat,
    F,
    FSharp,
    FFlat,
    G,
    GSharp,
    GFlat,
    A,
    ASharp,
    AFlat,
    B,
    BSharp,
    BFlat,
}

impl Note {
    pub const ALL_THEORETICAL: [Note; 21] = [
        C, CSharp, CFlat, D, DSharp, DFlat, E, ESharp, EFlat, F, FSharp, FFlat, G, GSharp, GFlat,
        A, ASharp, AFlat, B, BSharp, BFlat,
    ];

    pub const ALL: [Note; 17] = [
        C, CSharp, D, DSharp, DFlat, E, EFlat, F, FSharp, G, GSharp, GFlat, A, ASharp, AFlat, B,
        BFlat,
    ];

    pub const NATURALS: [Note; 7] = [C, D, E, F, G, A, B];

    pub fn is_natural(&self) -> bool {
        Self::NATURALS.contains(self)
    }

    pub const SHARPS: [Note; 5] = [CSharp, DSharp, FSharp, GSharp, ASharp];

    pub fn is_sharp(&self) -> bool {
        Self::SHARPS.contains(self)
    }

    pub const FLATS: [Note; 5] = [DFlat, EFlat, GFlat, AFlat, BFlat];

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
                C => "C",
                CSharp => "C#",
                D => "D",
                DSharp => "D#",
                DFlat => "Db",
                E => "E",
                EFlat => "Eb",
                F => "F",
                FSharp => "F#",
                G => "G",
                GSharp => "G#",
                GFlat => "Gb",
                A => "A",
                ASharp => "A#",
                AFlat => "Ab",
                B => "B",
                BFlat => "Bb",
                CFlat => "Cb",
                ESharp => "E#",
                FFlat => "Fb",
                BSharp => "B#",
            }
        )
    }
}

impl IntoView for Note {
    fn into_view(self, cx: leptos::Scope) -> leptos::View {
        self.to_string().into_view(cx)
    }
}
