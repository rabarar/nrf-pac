#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct OverreadW(pub u8);
impl OverreadW {
    #[doc = "Write: clear error on writing '1'"]
    pub const CLEAR: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u8);
impl Enable {
    #[doc = "Disable SPI slave"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable SPI slave"]
    pub const ENABLED: Self = Self(0x02);
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
pub struct Semstat(pub u8);
impl Semstat {
    #[doc = "Semaphore is free"]
    pub const FREE: Self = Self(0);
    #[doc = "Semaphore is assigned to CPU"]
    pub const CPU: Self = Self(0x01);
    #[doc = "Semaphore is assigned to SPI slave"]
    pub const SPIS: Self = Self(0x02);
    #[doc = "Semaphore is assigned to SPI but a handover to the CPU is pending"]
    pub const CPUPENDING: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct OverflowW(pub u8);
impl OverflowW {
    #[doc = "Write: clear error on writing '1'"]
    pub const CLEAR: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CsnConnect(pub u8);
impl CsnConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
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
pub struct Cpol(pub u8);
impl Cpol {
    #[doc = "Active high"]
    pub const ACTIVEHIGH: Self = Self(0);
    #[doc = "Active low"]
    pub const ACTIVELOW: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct OverreadR(pub u8);
impl OverreadR {
    #[doc = "Read: error not present"]
    pub const NOTPRESENT: Self = Self(0);
    #[doc = "Read: error present"]
    pub const PRESENT: Self = Self(0x01);
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
pub struct SckConnect(pub u8);
impl SckConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct OverflowR(pub u8);
impl OverflowR {
    #[doc = "Read: error not present"]
    pub const NOTPRESENT: Self = Self(0);
    #[doc = "Read: error present"]
    pub const PRESENT: Self = Self(0x01);
}
