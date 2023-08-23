use bimap::BiMap;
use const_format::concatcp;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use super::{MajorKey, MinorKey};
use crate::types::PATH_ICONS;

static ICON_A: &'static str = concatcp!(PATH_ICONS, "/keys/A.svg");
static ICON_AFLAT: &'static str = concatcp!(PATH_ICONS, "/keys/Ab.svg");
static ICON_B: &'static str = concatcp!(PATH_ICONS, "/keys/B.svg");
static ICON_BFLAT: &'static str = concatcp!(PATH_ICONS, "/keys/Bb.svg");
static ICON_CSHARP: &'static str = concatcp!(PATH_ICONS, "/keys/Cs.svg");
static ICON_C: &'static str = concatcp!(PATH_ICONS, "/keys/C.svg");
static ICON_CFLAT: &'static str = concatcp!(PATH_ICONS, "/keys/Cb.svg");
static ICON_D: &'static str = concatcp!(PATH_ICONS, "/keys/D.svg");
static ICON_DFLAT: &'static str = concatcp!(PATH_ICONS, "/keys/Db.svg");
static ICON_E: &'static str = concatcp!(PATH_ICONS, "/keys/E.svg");
static ICON_EFLAT: &'static str = concatcp!(PATH_ICONS, "/keys/Eb.svg");
static ICON_FSHARP: &'static str = concatcp!(PATH_ICONS, "/keys/Fs.svg");
static ICON_F: &'static str = concatcp!(PATH_ICONS, "/keys/F.svg");
static ICON_G: &'static str = concatcp!(PATH_ICONS, "/keys/G.svg");
static ICON_GFLAT: &'static str = concatcp!(PATH_ICONS, "/keys/Gb.svg");

static MAJOR_KEY_ICON_MAP: Lazy<Mutex<BiMap<MajorKey, &'static str>>> = Lazy::new(|| {
    let mut map = BiMap::<MajorKey, &'static str>::with_capacity(15);
    MajorKey::ALL.iter().into_iter().for_each(|key| {
        let key = *key;
        match key {
            MajorKey::C => map.insert(key, ICON_C),
            MajorKey::F => map.insert(key, ICON_F),
            MajorKey::G => map.insert(key, ICON_G),
            MajorKey::D => map.insert(key, ICON_D),
            MajorKey::A => map.insert(key, ICON_A),
            MajorKey::E => map.insert(key, ICON_E),
            MajorKey::B => map.insert(key, ICON_B),
            MajorKey::CFlat => map.insert(key, ICON_CFLAT),
            MajorKey::FSharp => map.insert(key, ICON_FSHARP),
            MajorKey::GFlat => map.insert(key, ICON_GFLAT),
            MajorKey::DFlat => map.insert(key, ICON_DFLAT),
            MajorKey::CSharp => map.insert(key, ICON_CSHARP),
            MajorKey::AFlat => map.insert(key, ICON_AFLAT),
            MajorKey::EFlat => map.insert(key, ICON_EFLAT),
            MajorKey::BFlat => map.insert(key, ICON_BFLAT),
        };
    });

    Mutex::new(map)
});

static MINOR_KEY_ICON_MAP: Lazy<Mutex<BiMap<MinorKey, &'static str>>> = Lazy::new(|| {
    let mut map = BiMap::<MinorKey, &'static str>::with_capacity(15);
    MinorKey::ALL.iter().into_iter().for_each(|key| {
        let key = *key;
        match key {
            MinorKey::A => map.insert(key, ICON_C),
            MinorKey::E => map.insert(key, ICON_G),
            MinorKey::B => map.insert(key, ICON_D),
            MinorKey::FSharp => map.insert(key, ICON_A),
            MinorKey::CSharp => map.insert(key, ICON_E),
            MinorKey::GSharp => map.insert(key, ICON_B),
            MinorKey::AFlat => map.insert(key, ICON_CFLAT),
            MinorKey::DSharp => map.insert(key, ICON_FSHARP),
            MinorKey::EFlat => map.insert(key, ICON_GFLAT),
            MinorKey::BFlat => map.insert(key, ICON_DFLAT),
            MinorKey::ASharp => map.insert(key, ICON_CSHARP),
            MinorKey::F => map.insert(key, ICON_AFLAT),
            MinorKey::C => map.insert(key, ICON_EFLAT),
            MinorKey::G => map.insert(key, ICON_BFLAT),
            MinorKey::D => map.insert(key, ICON_F),
        };
    });

    Mutex::new(map)
});

impl MajorKey {
    pub fn icon_url(&self) -> &'static str {
        let map = MAJOR_KEY_ICON_MAP
            .lock()
            .expect("lazily initialized static");

        map.get_by_left(self)
            .expect("an icon exists for every enum variant")
    }
}

impl MinorKey {
    pub fn icon_url(&self) -> &'static str {
        let map = MINOR_KEY_ICON_MAP
            .lock()
            .expect("lazily initialized static");

        map.get_by_left(self)
            .expect("an icon exists for every enum variant")
    }
}
