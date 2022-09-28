#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u8);
impl Enable {
    #[doc = "CRYPTOCELL subsystem disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "CRYPTOCELL subsystem enabled"]
    pub const ENABLED: Self = Self(0x01);
}
