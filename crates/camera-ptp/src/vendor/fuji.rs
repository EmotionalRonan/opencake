//! Fujifilm vendor profile.
//!
//! X-T30 (first gen) has no official tethering. Its firmware exposes still
//! capture over USB for gimbals/drones in the "USB video shooting auto/fixed"
//! connection modes. Capabilities below are placeholders until the hands-on
//! tests in docs/x-t30.md fill them in. No Fujifilm SDK code is used here.

use camera_core::Caps;

/// X-T30 product ID observed in libgphoto2 issue #509 (mode not recorded).
pub const X_T30_PID_UNVERIFIED: u16 = 0x02e3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FujiUsbMode {
    CardReader,
    RawConvBackup,
    GimbalAuto,
    GimbalFixed,
}

/// Capabilities per mode. Unknown until level 1–3 experiments complete.
pub fn x_t30_caps(mode: FujiUsbMode) -> Caps {
    match mode {
        FujiUsbMode::GimbalAuto | FujiUsbMode::GimbalFixed => Caps::empty(), // TODO: REMOTE_CAPTURE? SET_PROPS? DOWNLOAD?
        _ => Caps::empty(),
    }
}
