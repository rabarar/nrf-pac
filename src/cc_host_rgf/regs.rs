#[doc = "Controls lifecycle state (LCS) for CRYPTOCELL subsystem"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostIotLcs(pub u32);
impl HostIotLcs {
    #[doc = "Lifecycle state value. This field is write-once per reset."]
    #[inline(always)]
    pub const fn lcs(&self) -> super::vals::Lcs {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Lcs(val as u8)
    }
    #[doc = "Lifecycle state value. This field is write-once per reset."]
    #[inline(always)]
    pub fn set_lcs(&mut self, val: super::vals::Lcs) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.0 as u32) & 0x07) << 0usize);
    }
    #[doc = "This field is read-only and indicates if CRYPTOCELL LCS has been successfully configured since last reset"]
    #[inline(always)]
    pub const fn lcs_is_valid(&self) -> super::vals::LcsIsValid {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::LcsIsValid(val as u8)
    }
    #[doc = "This field is read-only and indicates if CRYPTOCELL LCS has been successfully configured since last reset"]
    #[inline(always)]
    pub fn set_lcs_is_valid(&mut self, val: super::vals::LcsIsValid) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.0 as u32) & 0x01) << 8usize);
    }
}
impl Default for HostIotLcs {
    #[inline(always)]
    fn default() -> HostIotLcs {
        HostIotLcs(0)
    }
}
#[doc = "This write-once register is the K_PRTL lock register. When this register is set, K_PRTL can not be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostIotKprtlLock(pub u32);
impl HostIotKprtlLock {
    #[doc = "This register is the K_PRTL lock register. When this register is set, K_PRTL can not be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub const fn host_iot_kprtl_lock(&self) -> super::vals::HostIotKprtlLock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::HostIotKprtlLock(val as u8)
    }
    #[doc = "This register is the K_PRTL lock register. When this register is set, K_PRTL can not be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn set_host_iot_kprtl_lock(&mut self, val: super::vals::HostIotKprtlLock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for HostIotKprtlLock {
    #[inline(always)]
    fn default() -> HostIotKprtlLock {
        HostIotKprtlLock(0)
    }
}
#[doc = "AES hardware key select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostCryptokeySel(pub u32);
impl HostCryptokeySel {
    #[doc = "Select the source of the HW key that is used by the AES engine"]
    #[inline(always)]
    pub const fn host_cryptokey_sel(&self) -> super::vals::HostCryptokeySel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::HostCryptokeySel(val as u8)
    }
    #[doc = "Select the source of the HW key that is used by the AES engine"]
    #[inline(always)]
    pub fn set_host_cryptokey_sel(&mut self, val: super::vals::HostCryptokeySel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
}
impl Default for HostCryptokeySel {
    #[inline(always)]
    fn default() -> HostCryptokeySel {
        HostCryptokeySel(0)
    }
}
