#[doc = "ARM TrustZone CryptoCell register interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cryptocell(pub *mut u8);
unsafe impl Send for Cryptocell {}
unsafe impl Sync for Cryptocell {}
impl Cryptocell {
    #[doc = "Enable CRYPTOCELL subsystem"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
}
pub mod regs;
pub mod vals;
