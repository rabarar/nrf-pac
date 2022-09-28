#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Anack(pub u8);
impl Anack {
    #[doc = "Error did not occur"]
    pub const NOTRECEIVED: Self = Self(0);
    #[doc = "Error occurred"]
    pub const RECEIVED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Overrun(pub u8);
impl Overrun {
    #[doc = "Error did not occur"]
    pub const NOTRECEIVED: Self = Self(0);
    #[doc = "Error occurred"]
    pub const RECEIVED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u8);
impl Enable {
    #[doc = "Disable TWIM"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable TWIM"]
    pub const ENABLED: Self = Self(0x06);
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
pub struct Dnack(pub u8);
impl Dnack {
    #[doc = "Error did not occur"]
    pub const NOTRECEIVED: Self = Self(0);
    #[doc = "Error occurred"]
    pub const RECEIVED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TxdListList(pub u8);
impl TxdListList {
    #[doc = "Disable EasyDMA list"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Use array list"]
    pub const ARRAYLIST: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Frequency(pub u32);
impl Frequency {
    #[doc = "100 kbps"]
    pub const K100: Self = Self(0x0198_0000);
    #[doc = "250 kbps"]
    pub const K250: Self = Self(0x0400_0000);
    #[doc = "400 kbps"]
    pub const K400: Self = Self(0x0640_0000);
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
pub struct RxdListList(pub u8);
impl RxdListList {
    #[doc = "Disable EasyDMA list"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Use array list"]
    pub const ARRAYLIST: Self = Self(0x01);
}
