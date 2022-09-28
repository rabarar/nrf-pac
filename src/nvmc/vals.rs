#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cacheprofen(pub u8);
impl Cacheprofen {
    #[doc = "Disable cache profiling"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable cache profiling"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ready(pub u8);
impl Ready {
    #[doc = "NVMC is busy (on-going write or erase operation)"]
    pub const BUSY: Self = Self(0);
    #[doc = "NVMC is ready"]
    pub const READY: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Wen(pub u8);
impl Wen {
    #[doc = "Read only access"]
    pub const REN: Self = Self(0);
    #[doc = "Write enabled"]
    pub const WEN: Self = Self(0x01);
    #[doc = "Erase enabled"]
    pub const EEN: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cacheen(pub u8);
impl Cacheen {
    #[doc = "Disable cache. Invalidates all cache entries."]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable cache"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Eraseuicr(pub u8);
impl Eraseuicr {
    #[doc = "No operation"]
    pub const NOOPERATION: Self = Self(0);
    #[doc = "Start erase of UICR"]
    pub const ERASE: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Eraseall(pub u8);
impl Eraseall {
    #[doc = "No operation"]
    pub const NOOPERATION: Self = Self(0);
    #[doc = "Start chip erase"]
    pub const ERASE: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Readynext(pub u8);
impl Readynext {
    #[doc = "NVMC cannot accept any write operation"]
    pub const BUSY: Self = Self(0);
    #[doc = "NVMC is ready"]
    pub const READY: Self = Self(0x01);
}
