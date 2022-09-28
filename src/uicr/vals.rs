#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Vout(pub u8);
impl Vout {
    #[doc = "1.8 V"]
    pub const _1V8: Self = Self(0);
    #[doc = "2.1 V"]
    pub const _2V1: Self = Self(0x01);
    #[doc = "2.4 V"]
    pub const _2V4: Self = Self(0x02);
    #[doc = "2.7 V"]
    pub const _2V7: Self = Self(0x03);
    #[doc = "3.0 V"]
    pub const _3V0: Self = Self(0x04);
    #[doc = "3.3 V"]
    pub const _3V3: Self = Self(0x05);
    #[doc = "Default voltage: 1.8 V"]
    pub const DEFAULT: Self = Self(0x07);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cpufpben(pub u8);
impl Cpufpben {
    #[doc = "Disable CPU FPB unit. Writes into the FPB registers will be ignored."]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable CPU FPB unit (default behavior)"]
    pub const ENABLED: Self = Self(0xff);
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
pub struct Protect(pub u8);
impl Protect {
    #[doc = "Operation as GPIO pins. Same protection as normal GPIO pins"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Operation as NFC antenna pins. Configures the protection for NFC operation"]
    pub const NFC: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cpuniden(pub u8);
impl Cpuniden {
    #[doc = "Disable CPU ITM and ETM functionality"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable CPU ITM and ETM functionality (default behavior)"]
    pub const ENABLED: Self = Self(0xff);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pall(pub u8);
impl Pall {
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0);
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0xff);
}
