//! Driver-agnostic camera abstractions (design doc §6.2).
//!
//! Every driver (ptp-native, gphoto2, hot folder, FTP, vendor bridge) exposes the
//! same `CameraDriver` / `CameraSession` surface; the UI greys out features based
//! on `Caps`.

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Caps(u32);

impl Caps {
    pub const REMOTE_CAPTURE: Caps = Caps(1 << 0);
    pub const SET_PROPS: Caps = Caps(1 << 1);
    pub const DOWNLOAD: Caps = Caps(1 << 2);
    pub const LIVEVIEW: Caps = Caps(1 << 3);
    pub const EVENTS: Caps = Caps(1 << 4);

    pub const fn empty() -> Self { Caps(0) }
    pub const fn union(self, other: Caps) -> Self { Caps(self.0 | other.0) }
    pub const fn contains(self, other: Caps) -> bool { self.0 & other.0 == other.0 }
}

#[derive(Debug, Clone)]
pub struct CameraInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: String,
    pub model: String,
    pub serial: Option<String>,
}

#[derive(Debug, Clone)]
pub enum CameraEvent {
    ObjectAdded { handle: u32 },
    PropertyChanged { code: u16 },
    CardFull,
    Disconnected,
}

#[derive(Debug, Clone)]
pub struct DownloadedObject {
    pub filename: String,
    pub data: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum CameraError {
    #[error("camera not found")]
    NotFound,
    #[error("operation not supported by this camera/mode")]
    Unsupported,
    #[error("transport error: {0}")]
    Transport(String),
    #[error("camera responded with code 0x{0:04x}")]
    Response(u16),
    #[error("timed out after {0:?}")]
    Timeout(Duration),
}

pub type Result<T> = std::result::Result<T, CameraError>;

pub trait CameraDriver {
    type Session: CameraSession;
    fn name(&self) -> &'static str;
    fn enumerate(&self) -> Result<Vec<CameraInfo>>;
    fn open(&self, camera: &CameraInfo) -> Result<Self::Session>;
}

pub trait CameraSession {
    fn caps(&self) -> Caps;
    fn capture(&mut self) -> Result<()>;
    fn download(&mut self, handle: u32) -> Result<DownloadedObject>;
    fn next_event(&mut self, timeout: Duration) -> Result<Option<CameraEvent>>;
}
