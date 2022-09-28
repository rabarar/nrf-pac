#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lcs(pub u8);
impl Lcs {
    #[doc = "CC310 operates in debug mode"]
    pub const DEBUG: Self = Self(0);
    #[doc = "CC310 operates in secure mode"]
    pub const SECURE: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HostCryptokeySel(pub u8);
impl HostCryptokeySel {
    #[doc = "Use device root key K_DR from CRYPTOCELL AO power domain"]
    pub const K_DR: Self = Self(0);
    #[doc = "Use hard-coded RTL key K_PRTL"]
    pub const K_PRTL: Self = Self(0x01);
    #[doc = "Use provided session key"]
    pub const SESSION: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LcsIsValid(pub u8);
impl LcsIsValid {
    #[doc = "A valid LCS is not yet retained in the CRYPTOCELL AO power domain"]
    pub const INVALID: Self = Self(0);
    #[doc = "A valid LCS is successfully retained in the CRYPTOCELL AO power domain"]
    pub const VALID: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HostIotKprtlLock(pub u8);
impl HostIotKprtlLock {
    #[doc = "K_PRTL can be selected for use from register HOST_CRYPTOKEY_SEL"]
    pub const DISABLED: Self = Self(0);
    #[doc = "K_PRTL has been locked until next power-on reset (POR). If K_PRTL is selected anyway, a zeroed key will be used instead."]
    pub const ENABLED: Self = Self(0x01);
}
