#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Write(pub u8);
impl Write {
    #[doc = "Allow write and erase instructions to region n"]
    pub const ENABLE: Self = Self(0);
    #[doc = "Block write and erase instructions to region n"]
    pub const DISABLE: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Read(pub u8);
impl Read {
    #[doc = "Allow read instructions to region n"]
    pub const ENABLE: Self = Self(0);
    #[doc = "Block read instructions to region n"]
    pub const DISABLE: Self = Self(0x01);
}
