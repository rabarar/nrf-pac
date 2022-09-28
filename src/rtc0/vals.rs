#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtensetCompare2w(pub u8);
impl EvtensetCompare2w {
    #[doc = "Enable"]
    pub const SET: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Compare3(pub u8);
impl Compare3 {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ovrflw(pub u8);
impl Ovrflw {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tick(pub u8);
impl Tick {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtensetOvrflwW(pub u8);
impl EvtensetOvrflwW {
    #[doc = "Enable"]
    pub const SET: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtensetCompare1r(pub u8);
impl EvtensetCompare1r {
    #[doc = "Read: Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Read: Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtenclrTickR(pub u8);
impl EvtenclrTickR {
    #[doc = "Read: Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Read: Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtenclrCompare2w(pub u8);
impl EvtenclrCompare2w {
    #[doc = "Disable"]
    pub const CLEAR: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtenclrOvrflwR(pub u8);
impl EvtenclrOvrflwR {
    #[doc = "Read: Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Read: Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtensetCompare0r(pub u8);
impl EvtensetCompare0r {
    #[doc = "Read: Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Read: Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtenclrTickW(pub u8);
impl EvtenclrTickW {
    #[doc = "Disable"]
    pub const CLEAR: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtenclrCompare1w(pub u8);
impl EvtenclrCompare1w {
    #[doc = "Disable"]
    pub const CLEAR: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtenclrCompare0r(pub u8);
impl EvtenclrCompare0r {
    #[doc = "Read: Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Read: Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Compare0(pub u8);
impl Compare0 {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtensetTickW(pub u8);
impl EvtensetTickW {
    #[doc = "Enable"]
    pub const SET: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtenclrCompare2r(pub u8);
impl EvtenclrCompare2r {
    #[doc = "Read: Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Read: Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtenclrCompare3r(pub u8);
impl EvtenclrCompare3r {
    #[doc = "Read: Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Read: Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtensetCompare2r(pub u8);
impl EvtensetCompare2r {
    #[doc = "Read: Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Read: Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtensetCompare3w(pub u8);
impl EvtensetCompare3w {
    #[doc = "Enable"]
    pub const SET: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtenclrCompare3w(pub u8);
impl EvtenclrCompare3w {
    #[doc = "Disable"]
    pub const CLEAR: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtenclrCompare0w(pub u8);
impl EvtenclrCompare0w {
    #[doc = "Disable"]
    pub const CLEAR: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtensetTickR(pub u8);
impl EvtensetTickR {
    #[doc = "Read: Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Read: Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Compare1(pub u8);
impl Compare1 {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtenclrOvrflwW(pub u8);
impl EvtenclrOvrflwW {
    #[doc = "Disable"]
    pub const CLEAR: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtensetCompare0w(pub u8);
impl EvtensetCompare0w {
    #[doc = "Enable"]
    pub const SET: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtensetOvrflwR(pub u8);
impl EvtensetOvrflwR {
    #[doc = "Read: Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Read: Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtensetCompare3r(pub u8);
impl EvtensetCompare3r {
    #[doc = "Read: Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Read: Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtensetCompare1w(pub u8);
impl EvtensetCompare1w {
    #[doc = "Enable"]
    pub const SET: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Compare2(pub u8);
impl Compare2 {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvtenclrCompare1r(pub u8);
impl EvtenclrCompare1r {
    #[doc = "Read: Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Read: Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
