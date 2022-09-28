#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sampleper(pub u8);
impl Sampleper {
    #[doc = "128 us"]
    pub const _128US: Self = Self(0);
    #[doc = "256 us"]
    pub const _256US: Self = Self(0x01);
    #[doc = "512 us"]
    pub const _512US: Self = Self(0x02);
    #[doc = "1024 us"]
    pub const _1024US: Self = Self(0x03);
    #[doc = "2048 us"]
    pub const _2048US: Self = Self(0x04);
    #[doc = "4096 us"]
    pub const _4096US: Self = Self(0x05);
    #[doc = "8192 us"]
    pub const _8192US: Self = Self(0x06);
    #[doc = "16384 us"]
    pub const _16384US: Self = Self(0x07);
    #[doc = "32768 us"]
    pub const _32MS: Self = Self(0x08);
    #[doc = "65536 us"]
    pub const _65MS: Self = Self(0x09);
    #[doc = "131072 us"]
    pub const _131MS: Self = Self(0x0a);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u8);
impl Enable {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Aconnect(pub u8);
impl Aconnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LedConnect(pub u8);
impl LedConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dbfen(pub u8);
impl Dbfen {
    #[doc = "Debounce input filters disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Debounce input filters enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Reportper(pub u8);
impl Reportper {
    #[doc = "10 samples / report"]
    pub const _10SMPL: Self = Self(0);
    #[doc = "40 samples / report"]
    pub const _40SMPL: Self = Self(0x01);
    #[doc = "80 samples / report"]
    pub const _80SMPL: Self = Self(0x02);
    #[doc = "120 samples / report"]
    pub const _120SMPL: Self = Self(0x03);
    #[doc = "160 samples / report"]
    pub const _160SMPL: Self = Self(0x04);
    #[doc = "200 samples / report"]
    pub const _200SMPL: Self = Self(0x05);
    #[doc = "240 samples / report"]
    pub const _240SMPL: Self = Self(0x06);
    #[doc = "280 samples / report"]
    pub const _280SMPL: Self = Self(0x07);
    #[doc = "1 sample / report"]
    pub const _1SMPL: Self = Self(0x08);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ledpol(pub u8);
impl Ledpol {
    #[doc = "Led active on output pin low"]
    pub const ACTIVELOW: Self = Self(0);
    #[doc = "Led active on output pin high"]
    pub const ACTIVEHIGH: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bconnect(pub u8);
impl Bconnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
