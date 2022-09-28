#[doc = "Random Number Generator"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rng(pub *mut u8);
unsafe impl Send for Rng {}
unsafe impl Sync for Rng {}
impl Rng {
    #[doc = "Task starting the random number generator"]
    #[inline(always)]
    pub fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Task stopping the random number generator"]
    #[inline(always)]
    pub fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Event being generated for every new random number written to the VALUE register"]
    #[inline(always)]
    pub fn events_valrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "Shortcut register"]
    #[inline(always)]
    pub fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(512usize)) }
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
    #[doc = "Configuration register"]
    #[inline(always)]
    pub fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "Output random number"]
    #[inline(always)]
    pub fn value(self) -> crate::common::Reg<regs::Value, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
}
pub mod regs;
pub mod vals;
