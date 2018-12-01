use ffi::AVColorPrimaries::*;
use ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Primaries {
    Reserved0,
    BT709,
    Unspecified,
    Reserved,
    BT470M,

    BT470BG,
    SMPTE170M,
    SMPTE240M,
    Film,
    BT2020,

    #[cfg(not(feature = "ffmpeg_rpi_zero_special"))]
    SMPTE428,

    #[cfg(feature = "ffmpeg_rpi_zero_special")]
    SMPTEST428_1,

    SMPTE431,
    SMPTE432,
    #[cfg(not(feature = "ffmpeg_rpi_zero_special"))]
    JEDEC_P22,
}

impl From<AVColorPrimaries> for Primaries {
    fn from(value: AVColorPrimaries) -> Primaries {
        match value {
            AVCOL_PRI_RESERVED0 => Primaries::Reserved0,
            AVCOL_PRI_BT709 => Primaries::BT709,
            AVCOL_PRI_UNSPECIFIED => Primaries::Unspecified,
            AVCOL_PRI_RESERVED => Primaries::Reserved,
            AVCOL_PRI_BT470M => Primaries::BT470M,

            AVCOL_PRI_BT470BG => Primaries::BT470BG,
            AVCOL_PRI_SMPTE170M => Primaries::SMPTE170M,
            AVCOL_PRI_SMPTE240M => Primaries::SMPTE240M,
            AVCOL_PRI_FILM => Primaries::Film,
            AVCOL_PRI_BT2020 => Primaries::BT2020,
            AVCOL_PRI_NB => Primaries::Reserved0,

            #[cfg(not(feature = "ffmpeg_rpi_zero_special"))]
            AVCOL_PRI_SMPTE428 => Primaries::SMPTE428,
            #[cfg(feature = "ffmpeg_rpi_zero_special")]
            AVCOL_PRI_SMPTEST428_1 => Primaries::SMPTEST428_1,
            AVCOL_PRI_SMPTE431 => Primaries::SMPTE431,
            AVCOL_PRI_SMPTE432 => Primaries::SMPTE432,
            #[cfg(not(feature = "ffmpeg_rpi_zero_special"))]
            AVCOL_PRI_JEDEC_P22 => Primaries::JEDEC_P22,
        }
    }
}

impl Into<AVColorPrimaries> for Primaries {
    fn into(self) -> AVColorPrimaries {
        match self {
            Primaries::Reserved0 => AVCOL_PRI_RESERVED0,
            Primaries::BT709 => AVCOL_PRI_BT709,
            Primaries::Unspecified => AVCOL_PRI_UNSPECIFIED,
            Primaries::Reserved => AVCOL_PRI_RESERVED,
            Primaries::BT470M => AVCOL_PRI_BT470M,

            Primaries::BT470BG => AVCOL_PRI_BT470BG,
            Primaries::SMPTE170M => AVCOL_PRI_SMPTE170M,
            Primaries::SMPTE240M => AVCOL_PRI_SMPTE240M,
            Primaries::Film => AVCOL_PRI_FILM,
            Primaries::BT2020 => AVCOL_PRI_BT2020,

            #[cfg(not(feature = "ffmpeg_rpi_zero_special"))]
            Primaries::SMPTE428 => AVCOL_PRI_SMPTE428,
            
            #[cfg(feature = "ffmpeg_rpi_zero_special")]
            Primaries::SMPTEST428_1 => AVCOL_PRI_SMPTEST428_1,

            Primaries::SMPTE431 => AVCOL_PRI_SMPTE431,
            Primaries::SMPTE432 => AVCOL_PRI_SMPTE432,
            #[cfg(not(feature = "ffmpeg_rpi_zero_special"))]
            Primaries::JEDEC_P22 => AVCOL_PRI_JEDEC_P22,
        }
    }
}
