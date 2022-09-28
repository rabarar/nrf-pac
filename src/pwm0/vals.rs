#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Prescaler(pub u8);
impl Prescaler {
    #[doc = "Divide by 1 (16 MHz)"]
    pub const DIV_1: Self = Self(0);
    #[doc = "Divide by 2 (8 MHz)"]
    pub const DIV_2: Self = Self(0x01);
    #[doc = "Divide by 4 (4 MHz)"]
    pub const DIV_4: Self = Self(0x02);
    #[doc = "Divide by 8 (2 MHz)"]
    pub const DIV_8: Self = Self(0x03);
    #[doc = "Divide by 16 (1 MHz)"]
    pub const DIV_16: Self = Self(0x04);
    #[doc = "Divide by 32 (500 kHz)"]
    pub const DIV_32: Self = Self(0x05);
    #[doc = "Divide by 64 (250 kHz)"]
    pub const DIV_64: Self = Self(0x06);
    #[doc = "Divide by 128 (125 kHz)"]
    pub const DIV_128: Self = Self(0x07);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u8);
impl Enable {
    #[doc = "Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RefreshCnt(pub u32);
impl RefreshCnt {
    #[doc = "Update every PWM period"]
    pub const CONTINUOUS: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CntCnt(pub u16);
impl CntCnt {
    #[doc = "Sequence is disabled, and shall not be started as it is empty"]
    pub const DISABLED: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mode(pub u8);
impl Mode {
    #[doc = "SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
    pub const REFRESHCOUNT: Self = Self(0);
    #[doc = "NEXTSTEP task causes a new value to be loaded to internal compare registers"]
    pub const NEXTSTEP: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Updown(pub u8);
impl Updown {
    #[doc = "Up counter, edge-aligned PWM duty cycle"]
    pub const UP: Self = Self(0);
    #[doc = "Up and down counter, center-aligned PWM duty cycle"]
    pub const UPANDDOWN: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Load(pub u8);
impl Load {
    #[doc = "1st half word (16-bit) used in all PWM channels 0..3"]
    pub const COMMON: Self = Self(0);
    #[doc = "1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
    pub const GROUPED: Self = Self(0x01);
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
    pub const INDIVIDUAL: Self = Self(0x02);
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
    pub const WAVEFORM: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Connect(pub u8);
impl Connect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LoopCnt(pub u16);
impl LoopCnt {
    #[doc = "Looping disabled (stop at the end of the sequence)"]
    pub const DISABLED: Self = Self(0);
}
