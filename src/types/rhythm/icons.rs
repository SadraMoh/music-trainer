use crate::types::PATH_ICONS;
use const_format::concatcp;

use super::Rhythm;

static ICON_HALF_REST: &'static str = concatcp!(PATH_ICONS, "/notes/half-rest.svg");
static ICON_QUARTER_REST: &'static str = concatcp!(PATH_ICONS, "/notes/quarter-rest.svg");
static ICON_EIGHTH_REST: &'static str = concatcp!(PATH_ICONS, "/notes/eighth-rest.svg");
static ICON_SIXTHTEENTH_REST: &'static str = concatcp!(PATH_ICONS, "/notes/sixteenth-rest.svg");
static ICON_SEMIBREVE: &'static str = concatcp!(PATH_ICONS, "/notes/semibreve.svg");
static ICON_MINIM: &'static str = concatcp!(PATH_ICONS, "/notes/minim.svg");
static ICON_CROTCHET: &'static str = concatcp!(PATH_ICONS, "/notes/crotchet.svg");
static ICON_QUAVER: &'static str = concatcp!(PATH_ICONS, "/notes/quaver.svg");
static ICON_QUAVER_QUAVER: &'static str = concatcp!(PATH_ICONS, "/notes/quaver-quaver.svg");
static ICON_QUAVER_SEMIQUAVER: &'static str = concatcp!(PATH_ICONS, "/notes/quaver-semiquaver.svg");
static ICON_QUAVER_SEMIQUAVER_DOT: &'static str =
    concatcp!(PATH_ICONS, "/notes/quaver-semiquaver-dot.svg");
static ICON_QUAVER_DOT_SEMIQUAVER: &'static str =
    concatcp!(PATH_ICONS, "/notes/quaver-dot-semiquaver.svg");
static ICON_SEMIQUAVER_SEMIQUAVER: &'static str =
    concatcp!(PATH_ICONS, "/notes/semiquaver-semiquaver.svg");
static ICON_QUAVER_SEMIQUAVER_SEMIQUAVER: &'static str =
    concatcp!(PATH_ICONS, "/notes/quaver-semiquaver-semiquaver.svg");
static ICON_SEMIQUAVER_SEMIQUAVER_QUAVER: &'static str =
    concatcp!(PATH_ICONS, "/notes/semiquaver-semiquaver-quaver.svg");
static ICON_SEMIQUAVER_QUAVER_SEMIQUAVER: &'static str =
    concatcp!(PATH_ICONS, "/notes/semiquaver-quaver-semiquaver.svg");
static ICON_QUAVER_QUAVER_QUAVER: &'static str =
    concatcp!(PATH_ICONS, "/notes/quaver-quaver-quaver.svg");
static ICON_SEMIQUAVER_SEMIQUAVER_SEMIQUAVER_SEMIQUAVER: &'static str = concatcp!(
    PATH_ICONS,
    "/notes/semiquaver-semiquaver-semiquaver-semiquaver.svg"
);
static ICON_QUAVER_SEMIQUAVER_SEMIQUAVER_QUAVER: &'static str =
    concatcp!(PATH_ICONS, "/notes/quaver-semiquaver-semiquaver-quaver.svg");
static ICON_SEMIQUAVER_SEMIQUAVER_QUAVER_QUAVER: &'static str =
    concatcp!(PATH_ICONS, "/notes/semiquaver-semiquaver-quaver-quaver.svg");
static ICON_QUAVER_QUAVER_SEMIQUAVER_SEMIQUAVER: &'static str =
    concatcp!(PATH_ICONS, "/notes/quaver-quaver-semiquaver-semiquaver.svg");

impl Rhythm {
    pub fn icon_url(&self) -> &'static str {
        use Rhythm as R;
        match self {
            R::HalfRest => ICON_HALF_REST,
            R::QuarterRest => ICON_QUARTER_REST,
            R::EighthRest => ICON_EIGHTH_REST,
            R::SixthteenthRest => ICON_SIXTHTEENTH_REST,
            R::Semibreve => ICON_SEMIBREVE,
            R::Minim => ICON_MINIM,
            R::Crotchet => ICON_CROTCHET,
            R::Quaver => ICON_QUAVER,
            R::QuaverQuaver => ICON_QUAVER_QUAVER,
            R::QuaverSemiquaver => ICON_QUAVER_SEMIQUAVER,
            R::QuaverSemiquaverDot => ICON_QUAVER_SEMIQUAVER_DOT,
            R::QuaverDotSemiquaver => ICON_QUAVER_DOT_SEMIQUAVER,
            R::SemiQuaverSemiquaver => ICON_SEMIQUAVER_SEMIQUAVER,
            R::QuaverSemiquaverSemiquaver => ICON_QUAVER_SEMIQUAVER_SEMIQUAVER,
            R::SemiquaverSemiquaverQuaver => ICON_SEMIQUAVER_SEMIQUAVER_QUAVER,
            R::SemiquaverQuaverSemiquaver => ICON_SEMIQUAVER_QUAVER_SEMIQUAVER,
            R::QuaverQuaverQuaver => ICON_QUAVER_QUAVER_QUAVER,
            R::SemiquaverSemiquaverSemiquaverSemiquaver => {
                ICON_SEMIQUAVER_SEMIQUAVER_SEMIQUAVER_SEMIQUAVER
            }
            R::QuaverSemiquaverSemiquaverQuaver => ICON_QUAVER_SEMIQUAVER_SEMIQUAVER_QUAVER,
            R::SemiquaverSemiquaverQuaverQuaver => ICON_SEMIQUAVER_SEMIQUAVER_QUAVER_QUAVER,
            R::QuaverQuaverSemiquaverSemiquaver => ICON_QUAVER_QUAVER_SEMIQUAVER_SEMIQUAVER,
        }
    }
}
