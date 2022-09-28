#[doc = "CRYPTOCELL HOST_RGF interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcHostRgf(pub *mut u8);
unsafe impl Send for CcHostRgf {}
unsafe impl Sync for CcHostRgf {}
impl CcHostRgf {
    #[doc = "AES hardware key select"]
    #[inline(always)]
    pub fn host_cryptokey_sel(
        self,
    ) -> crate::common::Reg<regs::HostCryptokeySel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(6712usize)) }
    }
    #[doc = "This write-once register is the K_PRTL lock register. When this register is set, K_PRTL can not be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn host_iot_kprtl_lock(
        self,
    ) -> crate::common::Reg<regs::HostIotKprtlLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(6732usize)) }
    }
    #[doc = "This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained."]
    #[inline(always)]
    pub fn host_iot_kdr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(6736usize)) }
    }
    #[doc = "This register holds bits 63:32 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn host_iot_kdr1(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(6740usize)) }
    }
    #[doc = "This register holds bits 95:64 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn host_iot_kdr2(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(6744usize)) }
    }
    #[doc = "This register holds bits 127:96 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn host_iot_kdr3(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(6748usize)) }
    }
    #[doc = "Controls lifecycle state (LCS) for CRYPTOCELL subsystem"]
    #[inline(always)]
    pub fn host_iot_lcs(self) -> crate::common::Reg<regs::HostIotLcs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(6752usize)) }
    }
}
pub mod regs;
pub mod vals;
