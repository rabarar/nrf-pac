#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Nfctagstate(pub u8);
impl Nfctagstate {
    #[doc = "Disabled or sense"]
    pub const DISABLED: Self = Self(0);
    #[doc = "RampUp"]
    pub const RAMPUP: Self = Self(0x02);
    #[doc = "Idle"]
    pub const IDLE: Self = Self(0x03);
    #[doc = "Receive"]
    pub const RECEIVE: Self = Self(0x04);
    #[doc = "FrameDelay"]
    pub const FRAMEDELAY: Self = Self(0x05);
    #[doc = "Transmit"]
    pub const TRANSMIT: Self = Self(0x06);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mode(pub u8);
impl Mode {
    #[doc = "Auto collision resolution enabled"]
    pub const ENABLED: Self = Self(0);
    #[doc = "Auto collision resolution disabled"]
    pub const DISABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fieldpresent(pub u8);
impl Fieldpresent {
    #[doc = "No valid field detected"]
    pub const NOFIELD: Self = Self(0);
    #[doc = "Valid field detected"]
    pub const FIELDPRESENT: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TxdFrameconfigSof(pub u8);
impl TxdFrameconfigSof {
    #[doc = "SoF symbol not added"]
    pub const NOSOF: Self = Self(0);
    #[doc = "SoF symbol added"]
    pub const SOF: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Crcmoderx(pub u8);
impl Crcmoderx {
    #[doc = "CRC is not expected in RX frames"]
    pub const NOCRCRX: Self = Self(0);
    #[doc = "Last 16 bits in RX frame is CRC, CRC is checked and CRCSTATUS updated"]
    pub const CRC16RX: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Crcmodetx(pub u8);
impl Crcmodetx {
    #[doc = "CRC is not added to the frame"]
    pub const NOCRCTX: Self = Self(0);
    #[doc = "16 bit CRC added to the frame based on all the data read from RAM that is used in the frame"]
    pub const CRC16TX: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Paritystatus(pub u8);
impl Paritystatus {
    #[doc = "Frame received with parity OK"]
    pub const PARITYOK: Self = Self(0);
    #[doc = "Frame received with parity error"]
    pub const PARITYERROR: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TxdFrameconfigParity(pub u8);
impl TxdFrameconfigParity {
    #[doc = "Parity is not added to TX frames"]
    pub const NOPARITY: Self = Self(0);
    #[doc = "Parity is added to TX frames"]
    pub const PARITY: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RxdFrameconfigParity(pub u8);
impl RxdFrameconfigParity {
    #[doc = "Parity is not expected in RX frames"]
    pub const NOPARITY: Self = Self(0);
    #[doc = "Parity is expected in RX frames"]
    pub const PARITY: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lockdetect(pub u8);
impl Lockdetect {
    #[doc = "Not locked to field"]
    pub const NOTLOCKED: Self = Self(0);
    #[doc = "Locked to field"]
    pub const LOCKED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Overrun(pub u8);
impl Overrun {
    #[doc = "No overrun detected"]
    pub const NOOVERRUN: Self = Self(0);
    #[doc = "Overrun error"]
    pub const OVERRUN: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sleepstate(pub u8);
impl Sleepstate {
    #[doc = "State is IDLE."]
    pub const IDLE: Self = Self(0);
    #[doc = "State is SLEEP_A."]
    pub const SLEEPA: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Crcerror(pub u8);
impl Crcerror {
    #[doc = "Valid CRC detected"]
    pub const CRCCORRECT: Self = Self(0);
    #[doc = "CRC received does not match local check"]
    pub const CRCERROR: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RxdFrameconfigSof(pub u8);
impl RxdFrameconfigSof {
    #[doc = "SoF symbol is not expected in RX frames"]
    pub const NOSOF: Self = Self(0);
    #[doc = "SoF symbol is expected in RX frames"]
    pub const SOF: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Discardmode(pub u8);
impl Discardmode {
    #[doc = "Unused bits are discarded at end of frame (EoF)"]
    pub const DISCARDEND: Self = Self(0);
    #[doc = "Unused bits are discarded at start of frame (SoF)"]
    pub const DISCARDSTART: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Nfcidsize(pub u8);
impl Nfcidsize {
    #[doc = "NFCID1 size: single (4 bytes)"]
    pub const NFCID1SINGLE: Self = Self(0);
    #[doc = "NFCID1 size: double (7 bytes)"]
    pub const NFCID1DOUBLE: Self = Self(0x01);
    #[doc = "NFCID1 size: triple (10 bytes)"]
    pub const NFCID1TRIPLE: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Framedelaymode(pub u8);
impl Framedelaymode {
    #[doc = "Transmission is independent of frame timer and will start when the STARTTX task is triggered. No timeout."]
    pub const FREERUN: Self = Self(0);
    #[doc = "Frame is transmitted between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    pub const WINDOW: Self = Self(0x01);
    #[doc = "Frame is transmitted exactly at FRAMEDELAYMAX"]
    pub const EXACTVAL: Self = Self(0x02);
    #[doc = "Frame is transmitted on a bit grid between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    pub const WINDOWGRID: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bitframesdd(pub u8);
impl Bitframesdd {
    #[doc = "SDD pattern 00000"]
    pub const SDD00000: Self = Self(0);
    #[doc = "SDD pattern 00001"]
    pub const SDD00001: Self = Self(0x01);
    #[doc = "SDD pattern 00010"]
    pub const SDD00010: Self = Self(0x02);
    #[doc = "SDD pattern 00100"]
    pub const SDD00100: Self = Self(0x04);
    #[doc = "SDD pattern 01000"]
    pub const SDD01000: Self = Self(0x08);
    #[doc = "SDD pattern 10000"]
    pub const SDD10000: Self = Self(0x10);
}
