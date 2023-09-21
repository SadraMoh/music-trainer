use std::{
    fmt::{Debug, Display},
    slice::Iter,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Rhythm {
    // WholeRest,
    HalfRest,
    QuarterRest,
    EighthRest,
    SixthteenthRest,
    // -
    Semibreve,
    Minim,
    Crotchet,
    Quaver,
    // Semiquaver,
    // -
    QuaverQuaver,
    QuaverSemiquaver,
    QuaverSemiquaverDot,
    QuaverDotSemiquaver,
    SemiQuaverSemiquaver,
    // -
    QuaverSemiquaverSemiquaver,
    SemiquaverSemiquaverQuaver,
    SemiquaverQuaverSemiquaver,
    QuaverQuaverQuaver,
    SemiquaverSemiquaverSemiquaverSemiquaver,
    QuaverSemiquaverSemiquaverQuaver,
    SemiquaverSemiquaverQuaverQuaver,
    QuaverQuaverSemiquaverSemiquaver,
}

impl Rhythm {
    const HALF_REST: [u8; 1] = [32];
    const QUARTER_REST: [u8; 1] = [16];
    const EIGHTH_REST: [u8; 1] = [8];
    const SIXTHTEENTH_REST: [u8; 1] = [4];
    const SEMIBREVE: [u8; 1] = [32];
    const MINIM: [u8; 1] = [16];
    const CROTCHET: [u8; 1] = [8];
    const QUAVER: [u8; 1] = [4];
    // const SEMIQUAVER: [u8; 1] = [2];
    const QUAVER_QUAVER: [u8; 2] = [4, 4];
    const QUAVER_SEMIQUAVER: [u8; 2] = [4, 2];
    const QUAVER_SEMIQUAVER_DOT: [u8; 2] = [4, 3];
    const QUAVER_DOT_SEMIQUAVER: [u8; 2] = [6, 2];
    const SEMIQUAVER_SEMIQUAVER: [u8; 2] = [2, 2];
    const QUAVER_SEMIQUAVER_SEMIQUAVER: [u8; 3] = [4, 2, 2];
    const SEMIQUAVER_SEMIQUAVER_QUAVER: [u8; 3] = [2, 2, 4];
    const SEMIQUAVER_QUAVER_SEMIQUAVER: [u8; 3] = [4, 2, 2];
    const QUAVER_QUAVER_QUAVER: [u8; 3] = [4, 4, 4];
    const SEMIQUAVER_SEMIQUAVER_SEMIQUAVER_SEMIQUAVER: [u8; 4] = [2, 2, 2, 2];
    const QUAVER_SEMIQUAVER_SEMIQUAVER_QUAVER: [u8; 4] = [4, 2, 2, 4];
    const SEMIQUAVER_SEMIQUAVER_QUAVER_QUAVER: [u8; 4] = [2, 2, 4, 4];
    const QUAVER_QUAVER_SEMIQUAVER_SEMIQUAVER: [u8; 4] = [4, 4, 2, 2];

    pub const ALL: [Rhythm; 21] = [
        Rhythm::HalfRest,
        Rhythm::QuarterRest,
        Rhythm::EighthRest,
        Rhythm::SixthteenthRest,
        Rhythm::Semibreve,
        Rhythm::Minim,
        Rhythm::Crotchet,
        Rhythm::Quaver,
        Rhythm::QuaverQuaver,
        Rhythm::QuaverSemiquaver,
        Rhythm::QuaverSemiquaverDot,
        Rhythm::QuaverDotSemiquaver,
        Rhythm::SemiQuaverSemiquaver,
        Rhythm::QuaverSemiquaverSemiquaver,
        Rhythm::SemiquaverSemiquaverQuaver,
        Rhythm::SemiquaverQuaverSemiquaver,
        Rhythm::QuaverQuaverQuaver,
        Rhythm::SemiquaverSemiquaverSemiquaverSemiquaver,
        Rhythm::QuaverSemiquaverSemiquaverQuaver,
        Rhythm::SemiquaverSemiquaverQuaverQuaver,
        Rhythm::QuaverQuaverSemiquaverSemiquaver,
    ];

    fn iters(&self) -> Iter<'_, u8> {
        use Rhythm as R;
        match self {
            R::HalfRest => R::HALF_REST.iter(),
            R::QuarterRest => R::QUARTER_REST.iter(),
            R::EighthRest => R::EIGHTH_REST.iter(),
            R::SixthteenthRest => R::SIXTHTEENTH_REST.iter(),
            R::Semibreve => R::SEMIBREVE.iter(),
            R::Minim => R::MINIM.iter(),
            R::Crotchet => R::CROTCHET.iter(),
            R::Quaver => R::QUAVER.iter(),
            R::QuaverQuaver => R::QUAVER_QUAVER.iter(),
            R::QuaverSemiquaver => R::QUAVER_SEMIQUAVER.iter(),
            R::QuaverSemiquaverDot => R::QUAVER_SEMIQUAVER_DOT.iter(),
            R::QuaverDotSemiquaver => R::QUAVER_DOT_SEMIQUAVER.iter(),
            R::SemiQuaverSemiquaver => R::SEMIQUAVER_SEMIQUAVER.iter(),
            R::QuaverSemiquaverSemiquaver => R::QUAVER_SEMIQUAVER_SEMIQUAVER.iter(),
            R::SemiquaverSemiquaverQuaver => R::SEMIQUAVER_SEMIQUAVER_QUAVER.iter(),
            R::SemiquaverQuaverSemiquaver => R::SEMIQUAVER_QUAVER_SEMIQUAVER.iter(),
            R::QuaverQuaverQuaver => R::QUAVER_QUAVER_QUAVER.iter(),
            R::SemiquaverSemiquaverSemiquaverSemiquaver => {
                R::SEMIQUAVER_SEMIQUAVER_SEMIQUAVER_SEMIQUAVER.iter()
            }
            R::QuaverSemiquaverSemiquaverQuaver => R::QUAVER_SEMIQUAVER_SEMIQUAVER_QUAVER.iter(),
            R::SemiquaverSemiquaverQuaverQuaver => R::SEMIQUAVER_SEMIQUAVER_QUAVER_QUAVER.iter(),
            R::QuaverQuaverSemiquaverSemiquaver => R::QUAVER_QUAVER_SEMIQUAVER_SEMIQUAVER.iter(),
        }
    }

    /// 32 -> semibreve
    /// 16 -> minim
    /// 8 -> crotchet
    /// 4 -> quaver
    /// 2 -> semiquaver
    /// 1 -> thirthytwoth
    pub fn duration(&self) -> u8 {
        Self::iters(self).sum()
    }

    pub fn beats(&self) -> &[u8] {
        Self::iters(self).as_slice()
    }
}
