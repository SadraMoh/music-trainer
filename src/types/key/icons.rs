use const_format::concatcp;

use super::{Key, MajorKey, MinorKey};
use crate::types::PATH_ICONS;

static ICON_A: &str = concatcp!(PATH_ICONS, "/keys/A.svg");
static ICON_AFLAT: &str = concatcp!(PATH_ICONS, "/keys/Ab.svg");
static ICON_B: &str = concatcp!(PATH_ICONS, "/keys/B.svg");
static ICON_BFLAT: &str = concatcp!(PATH_ICONS, "/keys/Bb.svg");
static ICON_CSHARP: &str = concatcp!(PATH_ICONS, "/keys/Cs.svg");
static ICON_C: &str = concatcp!(PATH_ICONS, "/keys/C.svg");
static ICON_CFLAT: &str = concatcp!(PATH_ICONS, "/keys/Cb.svg");
static ICON_D: &str = concatcp!(PATH_ICONS, "/keys/D.svg");
static ICON_DFLAT: &str = concatcp!(PATH_ICONS, "/keys/Db.svg");
static ICON_E: &str = concatcp!(PATH_ICONS, "/keys/E.svg");
static ICON_EFLAT: &str = concatcp!(PATH_ICONS, "/keys/Eb.svg");
static ICON_FSHARP: &str = concatcp!(PATH_ICONS, "/keys/Fs.svg");
static ICON_F: &str = concatcp!(PATH_ICONS, "/keys/F.svg");
static ICON_G: &str = concatcp!(PATH_ICONS, "/keys/G.svg");
static ICON_GFLAT: &str = concatcp!(PATH_ICONS, "/keys/Gb.svg");

impl Key {
    pub fn icon_path(&self) -> &str {
        match self {
            Key::Major(key) => match key {
                MajorKey::C => "C",
                MajorKey::F => todo!(),
                MajorKey::G => todo!(),
                MajorKey::D => todo!(),
                MajorKey::A => todo!(),
                MajorKey::E => todo!(),
                MajorKey::B => todo!(),
                MajorKey::CFlat => todo!(),
                MajorKey::FSharp => todo!(),
                MajorKey::GFlat => todo!(),
                MajorKey::DFlat => todo!(),
                MajorKey::CSharp => todo!(),
                MajorKey::AFlat => todo!(),
                MajorKey::EFlat => todo!(),
                MajorKey::BFlat => todo!(),
            },
            Key::Minor(key) => match key {
                MinorKey::A => "",
                MinorKey::E => "",
                MinorKey::B => "",
                MinorKey::FSharp => "",
                MinorKey::CSharp => "",
                MinorKey::GSharp => "",
                MinorKey::AFlat => "",
                MinorKey::DSharp => "",
                MinorKey::EFlat => "",
                MinorKey::BFlat => "",
                MinorKey::ASharp => "",
                MinorKey::F => "",
                MinorKey::C => "",
                MinorKey::G => "",
                MinorKey::D => "",
            },
        }
    }
}
