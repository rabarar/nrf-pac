#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cpol(pub u8);
impl Cpol {
    #[doc = "Active high"]
    pub const ACTIVEHIGH: Self = Self(0);
    #[doc = "Active low"]
    pub const ACTIVELOW: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u8);
impl Enable {
    #[doc = "Disable SPI"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable SPI"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MosiConnect(pub u8);
impl MosiConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MisoConnect(pub u8);
impl MisoConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cpha(pub u8);
impl Cpha {
    #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
    pub const LEADING: Self = Self(0);
    #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
    pub const TRAILING: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Order(pub u8);
impl Order {
    #[doc = "Most significant bit shifted out first"]
    pub const MSBFIRST: Self = Self(0);
    #[doc = "Least significant bit shifted out first"]
    pub const LSBFIRST: Self = Self(0x01);
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
pub struct Frequency(pub u32);
impl Frequency {
    #[doc = "125 kbps"]
    pub const K125: Self = Self(0x0200_0000);
    #[doc = "250 kbps"]
    pub const K250: Self = Self(0x0400_0000);
    #[doc = "500 kbps"]
    pub const K500: Self = Self(0x0800_0000);
    #[doc = "1 Mbps"]
    pub const M1: Self = Self(0x1000_0000);
    #[doc = "2 Mbps"]
    pub const M2: Self = Self(0x2000_0000);
    #[doc = "4 Mbps"]
    pub const M4: Self = Self(0x4000_0000);
    #[doc = "8 Mbps"]
    pub const M8: Self = Self(0x8000_0000);
}
