use ffi::AVColorTransferCharacteristic::*;
use ffi::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TransferCharacteristic {
    Reserved0,
    BT709,
    Unspecified,
    Reserved,
    GAMMA22,
    GAMMA28,
    SMPTE170M,
    SMPTE240M,
    Linear,
    Log,
    LogSqrt,
    IEC61966_2_4,
    BT1361_ECG,
    IEC61966_2_1,
    BT2020_10,
    BT2020_12,
    #[cfg(not(feature = "ffmpeg_rpi_zero_special"))]
    SMPTE2084,
    #[cfg(not(feature = "ffmpeg_rpi_zero_special"))]
    SMPTE428,
    ARIB_STD_B67,

    #[cfg(feature = "ffmpeg_rpi_zero_special")]
    SMPTEST2084,
    #[cfg(feature = "ffmpeg_rpi_zero_special")]
    SMPTEST428_1,
}

impl From<AVColorTransferCharacteristic> for TransferCharacteristic {
    fn from(value: AVColorTransferCharacteristic) -> TransferCharacteristic {
        match value {
            AVCOL_TRC_RESERVED0 => TransferCharacteristic::Reserved0,
            AVCOL_TRC_BT709 => TransferCharacteristic::BT709,
            AVCOL_TRC_UNSPECIFIED => TransferCharacteristic::Unspecified,
            AVCOL_TRC_RESERVED => TransferCharacteristic::Reserved,
            AVCOL_TRC_GAMMA22 => TransferCharacteristic::GAMMA22,
            AVCOL_TRC_GAMMA28 => TransferCharacteristic::GAMMA28,
            AVCOL_TRC_SMPTE170M => TransferCharacteristic::SMPTE170M,
            AVCOL_TRC_SMPTE240M => TransferCharacteristic::SMPTE240M,
            AVCOL_TRC_LINEAR => TransferCharacteristic::Linear,
            AVCOL_TRC_LOG => TransferCharacteristic::Log,
            AVCOL_TRC_LOG_SQRT => TransferCharacteristic::LogSqrt,
            AVCOL_TRC_IEC61966_2_4 => TransferCharacteristic::IEC61966_2_4,
            AVCOL_TRC_BT1361_ECG => TransferCharacteristic::BT1361_ECG,
            AVCOL_TRC_IEC61966_2_1 => TransferCharacteristic::IEC61966_2_1,
            AVCOL_TRC_BT2020_10 => TransferCharacteristic::BT2020_10,
            AVCOL_TRC_BT2020_12 => TransferCharacteristic::BT2020_12,
            AVCOL_TRC_NB => TransferCharacteristic::Reserved0,
            #[cfg(not(feature = "ffmpeg_rpi_zero_special"))]
            AVCOL_TRC_SMPTE2084 => TransferCharacteristic::SMPTE2084,
            #[cfg(not(feature = "ffmpeg_rpi_zero_special"))]
            AVCOL_TRC_SMPTE428 => TransferCharacteristic::SMPTE428,
            AVCOL_TRC_ARIB_STD_B67 => TransferCharacteristic::ARIB_STD_B67,
            #[cfg(feature = "ffmpeg_rpi_zero_special")]
            AVCOL_TRC_SMPTEST2084 => TransferCharacteristic::SMPTEST2084,
            #[cfg(feature = "ffmpeg_rpi_zero_special")]
            AVCOL_TRC_SMPTEST428_1 => TransferCharacteristic::SMPTEST428_1,
        }
    }
}

impl Into<AVColorTransferCharacteristic> for TransferCharacteristic {
    fn into(self) -> AVColorTransferCharacteristic {
        match self {
            TransferCharacteristic::Reserved0 => AVCOL_TRC_RESERVED0,
            TransferCharacteristic::BT709 => AVCOL_TRC_BT709,
            TransferCharacteristic::Unspecified => AVCOL_TRC_UNSPECIFIED,
            TransferCharacteristic::Reserved => AVCOL_TRC_RESERVED,
            TransferCharacteristic::GAMMA22 => AVCOL_TRC_GAMMA22,
            TransferCharacteristic::GAMMA28 => AVCOL_TRC_GAMMA28,
            TransferCharacteristic::SMPTE170M => AVCOL_TRC_SMPTE170M,
            TransferCharacteristic::SMPTE240M => AVCOL_TRC_SMPTE240M,
            TransferCharacteristic::Linear => AVCOL_TRC_LINEAR,
            TransferCharacteristic::Log => AVCOL_TRC_LOG,
            TransferCharacteristic::LogSqrt => AVCOL_TRC_LOG_SQRT,
            TransferCharacteristic::IEC61966_2_4 => AVCOL_TRC_IEC61966_2_4,
            TransferCharacteristic::BT1361_ECG => AVCOL_TRC_BT1361_ECG,
            TransferCharacteristic::IEC61966_2_1 => AVCOL_TRC_IEC61966_2_1,
            TransferCharacteristic::BT2020_10 => AVCOL_TRC_BT2020_10,
            TransferCharacteristic::BT2020_12 => AVCOL_TRC_BT2020_12,
            #[cfg(not(feature = "ffmpeg_rpi_zero_special"))]
            TransferCharacteristic::SMPTE2084 => AVCOL_TRC_SMPTE2084,
            #[cfg(not(feature = "ffmpeg_rpi_zero_special"))]
            TransferCharacteristic::SMPTE428 => AVCOL_TRC_SMPTE428,
            TransferCharacteristic::ARIB_STD_B67 => AVCOL_TRC_ARIB_STD_B67,
            #[cfg(feature = "ffmpeg_rpi_zero_special")]
             TransferCharacteristic::SMPTEST2084 => AVCOL_TRC_SMPTEST2084,
            #[cfg(feature = "ffmpeg_rpi_zero_special")]
            TransferCharacteristic::SMPTEST428_1 => AVCOL_TRC_SMPTEST428_1,
        }
    }
}