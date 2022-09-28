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
pub struct Overflow(pub u8);
impl Overflow {
    #[doc = "Error did not occur"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "Error occurred"]
    pub const DETECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u8);
impl Enable {
    #[doc = "Disable TWIS"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable TWIS"]
    pub const ENABLED: Self = Self(0x09);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dnack(pub u8);
impl Dnack {
    #[doc = "Error did not occur"]
    pub const NOTRECEIVED: Self = Self(0);
    #[doc = "Error occurred"]
    pub const RECEIVED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Address1(pub u8);
impl Address1 {
    #[doc = "Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Address0(pub u8);
impl Address0 {
    #[doc = "Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Overread(pub u8);
impl Overread {
    #[doc = "Error did not occur"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "Error occurred"]
    pub const DETECTED: Self = Self(0x01);
}
