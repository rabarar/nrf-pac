#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bitmode(pub u8);
impl Bitmode {
    #[doc = "16 bit timer bit width"]
    pub const _16BIT: Self = Self(0);
    #[doc = "8 bit timer bit width"]
    pub const _08BIT: Self = Self(0x01);
    #[doc = "24 bit timer bit width"]
    pub const _24BIT: Self = Self(0x02);
    #[doc = "32 bit timer bit width"]
    pub const _32BIT: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mode(pub u8);
impl Mode {
    #[doc = "Select Timer mode"]
    pub const TIMER: Self = Self(0);
    #[doc = "Deprecated enumerator - Select Counter mode"]
    pub const COUNTER: Self = Self(0x01);
    #[doc = "Select Low Power Counter mode"]
    pub const LOWPOWERCOUNTER: Self = Self(0x02);
}
