#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Part(pub u32);
impl Part {
    #[doc = "nRF52840"]
    pub const N52840: Self = Self(0x0005_2840);
    #[doc = "Unspecified"]
    pub const UNSPECIFIED: Self = Self(0xffff_ffff);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Package(pub u32);
impl Package {
    #[doc = "QIxx - 73-pin aQFN"]
    pub const QI: Self = Self(0x2004);
    #[doc = "Unspecified"]
    pub const UNSPECIFIED: Self = Self(0xffff_ffff);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Flash(pub u32);
impl Flash {
    #[doc = "128 kByte FLASH"]
    pub const K128: Self = Self(0x80);
    #[doc = "256 kByte FLASH"]
    pub const K256: Self = Self(0x0100);
    #[doc = "512 kByte FLASH"]
    pub const K512: Self = Self(0x0200);
    #[doc = "1 MByte FLASH"]
    pub const K1024: Self = Self(0x0400);
    #[doc = "2 MByte FLASH"]
    pub const K2048: Self = Self(0x0800);
    #[doc = "Unspecified"]
    pub const UNSPECIFIED: Self = Self(0xffff_ffff);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Deviceaddrtype(pub u8);
impl Deviceaddrtype {
    #[doc = "Public address"]
    pub const PUBLIC: Self = Self(0);
    #[doc = "Random address"]
    pub const RANDOM: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ram(pub u32);
impl Ram {
    #[doc = "16 kByte RAM"]
    pub const K16: Self = Self(0x10);
    #[doc = "32 kByte RAM"]
    pub const K32: Self = Self(0x20);
    #[doc = "64 kByte RAM"]
    pub const K64: Self = Self(0x40);
    #[doc = "128 kByte RAM"]
    pub const K128: Self = Self(0x80);
    #[doc = "256 kByte RAM"]
    pub const K256: Self = Self(0x0100);
    #[doc = "Unspecified"]
    pub const UNSPECIFIED: Self = Self(0xffff_ffff);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Prodtest(pub u32);
impl Prodtest {
    #[doc = "Production tests done"]
    pub const DONE: Self = Self(0xbb42_319f);
    #[doc = "Production tests not done"]
    pub const NOTDONE: Self = Self(0xffff_ffff);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Variant(pub u32);
impl Variant {
    #[doc = "AAAA"]
    pub const AAAA: Self = Self(0x4141_4141);
    #[doc = "AAAB"]
    pub const AAAB: Self = Self(0x4141_4142);
    #[doc = "AABA"]
    pub const AABA: Self = Self(0x4141_4241);
    #[doc = "AABB"]
    pub const AABB: Self = Self(0x4141_4242);
    #[doc = "AACA"]
    pub const AACA: Self = Self(0x4141_4341);
    #[doc = "BAAA"]
    pub const BAAA: Self = Self(0x4241_4141);
    #[doc = "CAAA"]
    pub const CAAA: Self = Self(0x4341_4141);
    #[doc = "Unspecified"]
    pub const UNSPECIFIED: Self = Self(0xffff_ffff);
}
