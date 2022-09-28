#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Stop(pub u8);
impl Stop {
    #[doc = "One stop bit"]
    pub const ONE: Self = Self(0);
    #[doc = "Two stop bits"]
    pub const TWO: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u8);
impl Enable {
    #[doc = "Disable UARTE"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable UARTE"]
    pub const ENABLED: Self = Self(0x08);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RtsConnect(pub u8);
impl RtsConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TxdConnect(pub u8);
impl TxdConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Baudrate(pub u32);
impl Baudrate {
    #[doc = "1200 baud (actual rate: 1205)"]
    pub const BAUD1200: Self = Self(0x0004_f000);
    #[doc = "2400 baud (actual rate: 2396)"]
    pub const BAUD2400: Self = Self(0x0009_d000);
    #[doc = "4800 baud (actual rate: 4808)"]
    pub const BAUD4800: Self = Self(0x0013_b000);
    #[doc = "9600 baud (actual rate: 9598)"]
    pub const BAUD9600: Self = Self(0x0027_5000);
    #[doc = "14400 baud (actual rate: 14401)"]
    pub const BAUD14400: Self = Self(0x003a_f000);
    #[doc = "19200 baud (actual rate: 19208)"]
    pub const BAUD19200: Self = Self(0x004e_a000);
    #[doc = "28800 baud (actual rate: 28777)"]
    pub const BAUD28800: Self = Self(0x0075_c000);
    #[doc = "31250 baud"]
    pub const BAUD31250: Self = Self(0x0080_0000);
    #[doc = "38400 baud (actual rate: 38369)"]
    pub const BAUD38400: Self = Self(0x009d_0000);
    #[doc = "56000 baud (actual rate: 55944)"]
    pub const BAUD56000: Self = Self(0x00e5_0000);
    #[doc = "57600 baud (actual rate: 57554)"]
    pub const BAUD57600: Self = Self(0x00eb_0000);
    #[doc = "76800 baud (actual rate: 76923)"]
    pub const BAUD76800: Self = Self(0x013a_9000);
    #[doc = "115200 baud (actual rate: 115108)"]
    pub const BAUD115200: Self = Self(0x01d6_0000);
    #[doc = "230400 baud (actual rate: 231884)"]
    pub const BAUD230400: Self = Self(0x03b0_0000);
    #[doc = "250000 baud"]
    pub const BAUD250000: Self = Self(0x0400_0000);
    #[doc = "460800 baud (actual rate: 457143)"]
    pub const BAUD460800: Self = Self(0x0740_0000);
    #[doc = "921600 baud (actual rate: 941176)"]
    pub const BAUD921600: Self = Self(0x0f00_0000);
    #[doc = "1Mega baud"]
    pub const BAUD1M: Self = Self(0x1000_0000);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Break(pub u8);
impl Break {
    #[doc = "Read: error not present"]
    pub const NOTPRESENT: Self = Self(0);
    #[doc = "Read: error present"]
    pub const PRESENT: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CtsConnect(pub u8);
impl CtsConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hwfc(pub u8);
impl Hwfc {
    #[doc = "Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ErrorsrcParity(pub u8);
impl ErrorsrcParity {
    #[doc = "Read: error not present"]
    pub const NOTPRESENT: Self = Self(0);
    #[doc = "Read: error present"]
    pub const PRESENT: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Overrun(pub u8);
impl Overrun {
    #[doc = "Read: error not present"]
    pub const NOTPRESENT: Self = Self(0);
    #[doc = "Read: error present"]
    pub const PRESENT: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Framing(pub u8);
impl Framing {
    #[doc = "Read: error not present"]
    pub const NOTPRESENT: Self = Self(0);
    #[doc = "Read: error present"]
    pub const PRESENT: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RxdConnect(pub u8);
impl RxdConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConfigParity(pub u8);
impl ConfigParity {
    #[doc = "Exclude parity bit"]
    pub const EXCLUDED: Self = Self(0);
    #[doc = "Include even parity bit"]
    pub const INCLUDED: Self = Self(0x07);
}
