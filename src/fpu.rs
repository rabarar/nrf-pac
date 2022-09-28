#[doc = "FPU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpu(pub *mut u8);
unsafe impl Send for Fpu {}
unsafe impl Sync for Fpu {}
impl Fpu {
    #[doc = "Unused."]
    #[inline(always)]
    pub fn unused(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
