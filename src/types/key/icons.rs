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
    map.insert(MajorKey::C, ICON_C);
    map.insert(MajorKey::F, ICON_F);
    map.insert(MajorKey::G, ICON_G);
    map.insert(MajorKey::D, ICON_D);
    map.insert(MajorKey::A, ICON_A);
    map.insert(MajorKey::E, ICON_E);
    map.insert(MajorKey::B, ICON_B);
    map.insert(MajorKey::CFlat, ICON_CFLAT);
    map.insert(MajorKey::FSharp, ICON_FSHARP);
    map.insert(MajorKey::GFlat, ICON_GFLAT);
    map.insert(MajorKey::DFlat, ICON_DFLAT);
    map.insert(MajorKey::CSharp, ICON_CSHARP);
    map.insert(MajorKey::AFlat, ICON_AFLAT);
    map.insert(MajorKey::EFlat, ICON_EFLAT);
    map.insert(MajorKey::BFlat, ICON_BFLAT);

    Mutex::new(map)
});

static MINOR_KEY_ICON_MAP: Lazy<Mutex<BiMap<MinorKey, &'static str>>> = Lazy::new(|| {
    let mut map = BiMap::<MinorKey, &'static str>::with_capacity(15);
    map.insert(MinorKey::A, ICON_C);
    map.insert(MinorKey::E, ICON_G);
    map.insert(MinorKey::B, ICON_B);
    map.insert(MinorKey::FSharp, ICON_FSHARP);
    map.insert(MinorKey::CSharp, ICON_E);
    map.insert(MinorKey::GSharp, ICON_B);
    map.insert(MinorKey::AFlat, ICON_CFLAT);
    map.insert(MinorKey::DSharp, ICON_FSHARP);
    map.insert(MinorKey::EFlat, ICON_GFLAT);
    map.insert(MinorKey::BFlat, ICON_DFLAT);
    map.insert(MinorKey::ASharp, ICON_CSHARP);
    map.insert(MinorKey::F, ICON_AFLAT);
    map.insert(MinorKey::C, ICON_EFLAT);
    map.insert(MinorKey::G, ICON_BFLAT);
    map.insert(MinorKey::D, ICON_F);

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
