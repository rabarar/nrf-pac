#[doc = "AES ECB Mode Encryption"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecb(pub *mut u8);
unsafe impl Send for Ecb {}
unsafe impl Sync for Ecb {}
impl Ecb {
    #[doc = "Start ECB block encrypt"]
    #[inline(always)]
    pub fn tasks_startecb(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Abort a possible executing ECB operation"]
    #[inline(always)]
    pub fn tasks_stopecb(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "ECB block encrypt complete"]
    #[inline(always)]
    pub fn events_endecb(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error"]
    #[inline(always)]
    pub fn events_errorecb(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(772usize)) }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(776usize)) }
    }
    #[doc = "ECB block encrypt memory pointers"]
    #[inline(always)]
    pub fn ecbdataptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
}
pub mod regs;
