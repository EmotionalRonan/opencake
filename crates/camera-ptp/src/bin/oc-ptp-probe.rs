//! oc-ptp-probe: profile a PTP camera.
//!
//! Planned subcommands (docs/x-t30.md §5 group C):
//!   info                dump USB IDs + PTP DeviceInfo + StorageIDs as JSON
//!   capture [--download]  trigger the shutter, optionally fetch the new object
//!   props               read/write shutter, aperture, ISO, exposure compensation
//!
//! Status: `list` works (USB enumeration via nusb); the rest are stubs.

fn main() {
    let cmd = std::env::args().nth(1).unwrap_or_else(|| "list".into());
    match cmd.as_str() {
        "list" => list(),
        "info" | "capture" | "props" => {
            eprintln!("`{cmd}` is not implemented yet. Track progress in the X-T30 issues.");
            std::process::exit(2);
        }
        _ => {
            eprintln!("usage: oc-ptp-probe <list|info|capture|props>");
            std::process::exit(1);
        }
    }
}

fn list() {
    let devices = match nusb::list_devices() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("failed to list USB devices: {e}");
            std::process::exit(1);
        }
    };
    for dev in devices {
        let fuji = if dev.vendor_id() == camera_ptp::FUJIFILM_VID {
            "  <- Fujifilm"
        } else {
            ""
        };
        println!(
            "{:04x}:{:04x}  {} {}{}",
            dev.vendor_id(),
            dev.product_id(),
            dev.manufacturer_string().unwrap_or("?"),
            dev.product_string().unwrap_or("?"),
            fuji
        );
    }
}
