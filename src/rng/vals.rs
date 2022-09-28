#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dercen(pub u8);
impl Dercen {
    #[doc = "Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
