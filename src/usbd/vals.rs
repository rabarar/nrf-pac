#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ready(pub u8);
impl Ready {
    #[doc = "USBEVENT was not issued due to USBD peripheral ready"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "USBD peripheral is ready"]
    pub const READY: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Resume(pub u8);
impl Resume {
    #[doc = "Resume not detected"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "Resume detected"]
    pub const DETECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Suspend(pub u8);
impl Suspend {
    #[doc = "Suspend not detected"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "Suspend detected"]
    pub const DETECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Zero(pub u8);
impl Zero {
    #[doc = "No zero-length data received, use value in SIZE"]
    pub const NORMAL: Self = Self(0);
    #[doc = "Zero-length data received, ignore value in SIZE"]
    pub const ZERODATA: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Isooutcrc(pub u8);
impl Isooutcrc {
    #[doc = "No error detected"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "Error detected"]
    pub const DETECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Usbwuallowed(pub u8);
impl Usbwuallowed {
    #[doc = "Wake up not allowed"]
    pub const NOTALLOWED: Self = Self(0);
    #[doc = "Wake up allowed"]
    pub const ALLOWED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Value(pub u8);
impl Value {
    #[doc = "No action on data toggle when writing the register with this value"]
    pub const NOP: Self = Self(0);
    #[doc = "Data toggle is DATA0 on endpoint set by EP and IO"]
    pub const DATA0: Self = Self(0x01);
    #[doc = "Data toggle is DATA1 on endpoint set by EP and IO"]
    pub const DATA1: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct State(pub u8);
impl State {
    #[doc = "D+ forced low, D- forced high (K state) for a timing preset in hardware (50 us or 5 ms, depending on bus state)"]
    pub const RESUME: Self = Self(0x01);
    #[doc = "D+ forced high, D- forced low (J state)"]
    pub const J: Self = Self(0x02);
    #[doc = "D+ forced low, D- forced high (K state)"]
    pub const K: Self = Self(0x04);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EpinGetstatus(pub u16);
impl EpinGetstatus {
    #[doc = "Endpoint is not halted"]
    pub const NOTHALTED: Self = Self(0);
    #[doc = "Endpoint is halted"]
    pub const HALTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lowpower(pub u8);
impl Lowpower {
    #[doc = "Software must write this value to exit low power mode and before performing a remote wake-up"]
    pub const FORCENORMAL: Self = Self(0);
    #[doc = "Software must write this value to enter low power mode after DMA and software have finished interacting with the USB peripheral"]
    pub const LOWPOWER: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Split(pub u16);
impl Split {
    #[doc = "Full buffer dedicated to either iso IN or OUT"]
    pub const ONEDIR: Self = Self(0);
    #[doc = "Lower half for IN, upper half for OUT"]
    pub const HALFIN: Self = Self(0x80);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Recipient(pub u8);
impl Recipient {
    #[doc = "Device"]
    pub const DEVICE: Self = Self(0);
    #[doc = "Interface"]
    pub const INTERFACE: Self = Self(0x01);
    #[doc = "Endpoint"]
    pub const ENDPOINT: Self = Self(0x02);
    #[doc = "Other"]
    pub const OTHER: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EpoutGetstatus(pub u16);
impl EpoutGetstatus {
    #[doc = "Endpoint is not halted"]
    pub const NOTHALTED: Self = Self(0);
    #[doc = "Endpoint is halted"]
    pub const HALTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Type(pub u8);
impl Type {
    #[doc = "Standard"]
    pub const STANDARD: Self = Self(0);
    #[doc = "Class"]
    pub const CLASS: Self = Self(0x01);
    #[doc = "Vendor"]
    pub const VENDOR: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Stall(pub u8);
impl Stall {
    #[doc = "Don't stall selected endpoint"]
    pub const UNSTALL: Self = Self(0);
    #[doc = "Stall selected endpoint"]
    pub const STALL: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Direction(pub u8);
impl Direction {
    #[doc = "Host-to-device"]
    pub const HOSTTODEVICE: Self = Self(0);
    #[doc = "Device-to-host"]
    pub const DEVICETOHOST: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EpstallIo(pub u8);
impl EpstallIo {
    #[doc = "Selects OUT endpoint"]
    pub const OUT: Self = Self(0);
    #[doc = "Selects IN endpoint"]
    pub const IN: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DtoggleIo(pub u8);
impl DtoggleIo {
    #[doc = "Selects OUT endpoint"]
    pub const OUT: Self = Self(0);
    #[doc = "Selects IN endpoint"]
    pub const IN: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Brequest(pub u8);
impl Brequest {
    #[doc = "Standard request GET_STATUS"]
    pub const STD_GET_STATUS: Self = Self(0);
    #[doc = "Standard request CLEAR_FEATURE"]
    pub const STD_CLEAR_FEATURE: Self = Self(0x01);
    #[doc = "Standard request SET_FEATURE"]
    pub const STD_SET_FEATURE: Self = Self(0x03);
    #[doc = "Standard request SET_ADDRESS"]
    pub const STD_SET_ADDRESS: Self = Self(0x05);
    #[doc = "Standard request GET_DESCRIPTOR"]
    pub const STD_GET_DESCRIPTOR: Self = Self(0x06);
    #[doc = "Standard request SET_DESCRIPTOR"]
    pub const STD_SET_DESCRIPTOR: Self = Self(0x07);
    #[doc = "Standard request GET_CONFIGURATION"]
    pub const STD_GET_CONFIGURATION: Self = Self(0x08);
    #[doc = "Standard request SET_CONFIGURATION"]
    pub const STD_SET_CONFIGURATION: Self = Self(0x09);
    #[doc = "Standard request GET_INTERFACE"]
    pub const STD_GET_INTERFACE: Self = Self(0x0a);
    #[doc = "Standard request SET_INTERFACE"]
    pub const STD_SET_INTERFACE: Self = Self(0x0b);
    #[doc = "Standard request SYNCH_FRAME"]
    pub const STD_SYNCH_FRAME: Self = Self(0x0c);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Response(pub u8);
impl Response {
    #[doc = "Endpoint does not respond in that case"]
    pub const NORESP: Self = Self(0);
    #[doc = "Endpoint responds with a zero-length data packet in that case"]
    pub const ZERODATA: Self = Self(0x01);
}
