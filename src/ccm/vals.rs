#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Micstatus(pub u8);
impl Micstatus {
    #[doc = "MIC check failed"]
    pub const CHECKFAILED: Self = Self(0);
    #[doc = "MIC check passed"]
    pub const CHECKPASSED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rateoverride(pub u8);
impl Rateoverride {
    #[doc = "1 Mbps"]
    pub const _1MBIT: Self = Self(0);
    #[doc = "2 Mbps"]
    pub const _2MBIT: Self = Self(0x01);
    #[doc = "125 Kbps"]
    pub const _125KBPS: Self = Self(0x02);
    #[doc = "500 Kbps"]
    pub const _500KBPS: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Datarate(pub u8);
impl Datarate {
    #[doc = "1 Mbps"]
    pub const _1MBIT: Self = Self(0);
    #[doc = "2 Mbps"]
    pub const _2MBIT: Self = Self(0x01);
    #[doc = "125 Kbps"]
    pub const _125KBPS: Self = Self(0x02);
    #[doc = "500 Kbps"]
    pub const _500KBPS: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Length(pub u8);
impl Length {
    #[doc = "Default length. Effective length of LENGTH field in encrypted/decrypted packet is 5 bits. A key-stream for packet payloads up to 27 bytes will be generated."]
    pub const DEFAULT: Self = Self(0);
    #[doc = "Extended length. Effective length of LENGTH field in encrypted/decrypted packet is 8 bits. A key-stream for packet payloads up to MAXPACKETSIZE bytes will be generated."]
    pub const EXTENDED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mode(pub u8);
impl Mode {
    #[doc = "AES CCM packet encryption mode"]
    pub const ENCRYPTION: Self = Self(0);
    #[doc = "AES CCM packet decryption mode"]
    pub const DECRYPTION: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u8);
impl Enable {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x02);
}
