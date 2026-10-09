//! Native PTP (ISO 15740) driver for OpenCake.
//!
//! Status: skeleton. First milestone is `oc-ptp-probe`, used to profile the
//! Fujifilm X-T30 in each USB connection mode (see docs/x-t30.md).

pub mod codes;
pub mod vendor;

/// USB vendor ID for Fujifilm cameras.
pub const FUJIFILM_VID: u16 = 0x04cb;
