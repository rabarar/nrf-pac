#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u8);
impl Enable {
    #[doc = "Disable TWI"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable TWI"]
    pub const ENABLED: Self = Self(0x05);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dnack(pub u8);
impl Dnack {
    #[doc = "Read: error not present"]
    pub const NOTPRESENT: Self = Self(0);
    #[doc = "Read: error present"]
    pub const PRESENT: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Overrun(pub u8);
impl Overrun {
    #[doc = "Read: no overrun occured"]
    pub const NOTPRESENT: Self = Self(0);
    #[doc = "Read: overrun occured"]
    pub const PRESENT: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Frequency(pub u32);
impl Frequency {
    #[doc = "100 kbps"]
    pub const K100: Self = Self(0x0198_0000);
    #[doc = "250 kbps"]
    pub const K250: Self = Self(0x0400_0000);
    #[doc = "400 kbps (actual rate 410.256 kbps)"]
    pub const K400: Self = Self(0x0668_0000);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SclConnect(pub u8);
impl SclConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SdaConnect(pub u8);
impl SdaConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Anack(pub u8);
impl Anack {
    #[doc = "Read: error not present"]
    pub const NOTPRESENT: Self = Self(0);
    #[doc = "Read: error present"]
    pub const PRESENT: Self = Self(0x01);
}
