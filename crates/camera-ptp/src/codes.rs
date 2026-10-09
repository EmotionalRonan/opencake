//! Standard PTP operation, response and event codes used by OpenCake.

pub mod op {
    pub const GET_DEVICE_INFO: u16 = 0x1001;
    pub const OPEN_SESSION: u16 = 0x1002;
    pub const CLOSE_SESSION: u16 = 0x1003;
    pub const GET_STORAGE_IDS: u16 = 0x1004;
    pub const GET_OBJECT_HANDLES: u16 = 0x1007;
    pub const GET_OBJECT_INFO: u16 = 0x1008;
    pub const GET_OBJECT: u16 = 0x1009;
    pub const INITIATE_CAPTURE: u16 = 0x100E;
    pub const GET_DEVICE_PROP_VALUE: u16 = 0x1015;
    pub const SET_DEVICE_PROP_VALUE: u16 = 0x1016;
    pub const GET_PARTIAL_OBJECT: u16 = 0x101B;
}

pub mod response {
    pub const OK: u16 = 0x2001;
    pub const GENERAL_ERROR: u16 = 0x2002;
    pub const OPERATION_NOT_SUPPORTED: u16 = 0x2005;
    pub const DEVICE_BUSY: u16 = 0x2019;
}

pub mod event {
    pub const OBJECT_ADDED: u16 = 0x4002;
    pub const DEVICE_PROP_CHANGED: u16 = 0x4006;
    pub const STORE_FULL: u16 = 0x400A;
    pub const CAPTURE_COMPLETE: u16 = 0x400D;
}
