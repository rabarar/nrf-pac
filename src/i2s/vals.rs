#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mckfreq(pub u32);
impl Mckfreq {
    #[doc = "32 MHz / 125 = 0.256 MHz"]
    pub const _32MDIV125: Self = Self(0x020c_0000);
    #[doc = "32 MHz / 63 = 0.5079365 MHz"]
    pub const _32MDIV63: Self = Self(0x0410_0000);
    #[doc = "32 MHz / 42 = 0.7619048 MHz"]
    pub const _32MDIV42: Self = Self(0x0600_0000);
    #[doc = "32 MHz / 32 = 1.0 MHz"]
    pub const _32MDIV32: Self = Self(0x0800_0000);
    #[doc = "32 MHz / 31 = 1.0322581 MHz"]
    pub const _32MDIV31: Self = Self(0x0840_0000);
    #[doc = "32 MHz / 30 = 1.0666667 MHz"]
    pub const _32MDIV30: Self = Self(0x0880_0000);
    #[doc = "32 MHz / 23 = 1.3913043 MHz"]
    pub const _32MDIV23: Self = Self(0x0b00_0000);
    #[doc = "32 MHz / 21 = 1.5238095"]
    pub const _32MDIV21: Self = Self(0x0c00_0000);
    #[doc = "32 MHz / 16 = 2.0 MHz"]
    pub const _32MDIV16: Self = Self(0x1000_0000);
    #[doc = "32 MHz / 15 = 2.1333333 MHz"]
    pub const _32MDIV15: Self = Self(0x1100_0000);
    #[doc = "32 MHz / 11 = 2.9090909 MHz"]
    pub const _32MDIV11: Self = Self(0x1600_0000);
    #[doc = "32 MHz / 10 = 3.2 MHz"]
    pub const _32MDIV10: Self = Self(0x1800_0000);
    #[doc = "32 MHz / 8 = 4.0 MHz"]
    pub const _32MDIV8: Self = Self(0x2000_0000);
    #[doc = "32 MHz / 6 = 5.3333333 MHz"]
    pub const _32MDIV6: Self = Self(0x2800_0000);
    #[doc = "32 MHz / 5 = 6.4 MHz"]
    pub const _32MDIV5: Self = Self(0x3000_0000);
    #[doc = "32 MHz / 4 = 8.0 MHz"]
    pub const _32MDIV4: Self = Self(0x4000_0000);
    #[doc = "32 MHz / 3 = 10.6666667 MHz"]
    pub const _32MDIV3: Self = Self(0x5000_0000);
    #[doc = "32 MHz / 2 = 16.0 MHz"]
    pub const _32MDIV2: Self = Self(0x8000_0000);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SdoutConnect(pub u8);
impl SdoutConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SckConnect(pub u8);
impl SckConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LrckConnect(pub u8);
impl LrckConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SdinConnect(pub u8);
impl SdinConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Swidth(pub u8);
impl Swidth {
    #[doc = "8 bit."]
    pub const _8BIT: Self = Self(0);
    #[doc = "16 bit."]
    pub const _16BIT: Self = Self(0x01);
    #[doc = "24 bit."]
    pub const _24BIT: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Channels(pub u8);
impl Channels {
    #[doc = "Stereo."]
    pub const STEREO: Self = Self(0);
    #[doc = "Left only."]
    pub const LEFT: Self = Self(0x01);
    #[doc = "Right only."]
    pub const RIGHT: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ratio(pub u8);
impl Ratio {
    #[doc = "LRCK = MCK / 32"]
    pub const _32X: Self = Self(0);
    #[doc = "LRCK = MCK / 48"]
    pub const _48X: Self = Self(0x01);
    #[doc = "LRCK = MCK / 64"]
    pub const _64X: Self = Self(0x02);
    #[doc = "LRCK = MCK / 96"]
    pub const _96X: Self = Self(0x03);
    #[doc = "LRCK = MCK / 128"]
    pub const _128X: Self = Self(0x04);
    #[doc = "LRCK = MCK / 192"]
    pub const _192X: Self = Self(0x05);
    #[doc = "LRCK = MCK / 256"]
    pub const _256X: Self = Self(0x06);
    #[doc = "LRCK = MCK / 384"]
    pub const _384X: Self = Self(0x07);
    #[doc = "LRCK = MCK / 512"]
    pub const _512X: Self = Self(0x08);
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
pub struct Format(pub u8);
impl Format {
    #[doc = "Original I2S format."]
    pub const I2S: Self = Self(0);
    #[doc = "Alternate (left- or right-aligned) format."]
    pub const ALIGNED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rxen(pub u8);
impl Rxen {
    #[doc = "Reception disabled and now data will be written to the RXD.PTR address."]
    pub const DISABLED: Self = Self(0);
    #[doc = "Reception enabled."]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Align(pub u8);
impl Align {
    #[doc = "Left-aligned."]
    pub const LEFT: Self = Self(0);
    #[doc = "Right-aligned."]
    pub const RIGHT: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mode(pub u8);
impl Mode {
    #[doc = "Master mode. SCK and LRCK generated from internal master clcok (MCK) and output on pins defined by PSEL.xxx."]
    pub const MASTER: Self = Self(0);
    #[doc = "Slave mode. SCK and LRCK generated by external master and received on pins defined by PSEL.xxx"]
    pub const SLAVE: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MckConnect(pub u8);
impl MckConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Txen(pub u8);
impl Txen {
    #[doc = "Transmission disabled and now data will be read from the RXD.TXD address."]
    pub const DISABLED: Self = Self(0);
    #[doc = "Transmission enabled."]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mcken(pub u8);
impl Mcken {
    #[doc = "Master clock generator disabled and PSEL.MCK not connected(available as GPIO)."]
    pub const DISABLED: Self = Self(0);
    #[doc = "Master clock generator running and MCK output on PSEL.MCK."]
    pub const ENABLED: Self = Self(0x01);
}
