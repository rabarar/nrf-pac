#[doc = "Software interrupt 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swi0(pub *mut u8);
unsafe impl Send for Swi0 {}
unsafe impl Sync for Swi0 {}
impl Swi0 {
    #[doc = "Unused."]
    #[inline(always)]
    pub fn unused(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
