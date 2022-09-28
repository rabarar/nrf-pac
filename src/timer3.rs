#[doc = "Timer/Counter 3"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3(pub *mut u8);
unsafe impl Send for Timer3 {}
unsafe impl Sync for Timer3 {}
impl Timer3 {
    #[doc = "Start Timer"]
    #[inline(always)]
    pub fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Stop Timer"]
    #[inline(always)]
    pub fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Increment Timer (Counter mode only)"]
    #[inline(always)]
    pub fn tasks_count(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Clear time"]
    #[inline(always)]
    pub fn tasks_clear(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Deprecated register - Shut down timer"]
    #[inline(always)]
    pub fn tasks_shutdown(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Description collection\\[n\\]: Capture Timer value to CC\\[n\\] register"]
    #[inline(always)]
    pub fn tasks_capture(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize + n * 4usize)) }
    }
    #[doc = "Description collection\\[n\\]: Compare event on CC\\[n\\] match"]
    #[inline(always)]
    pub fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(320usize + n * 4usize)) }
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
    #[doc = "Timer mode selection"]
    #[inline(always)]
    pub fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "Configure the number of bits used by the TIMER"]
    #[inline(always)]
    pub fn bitmode(self) -> crate::common::Reg<regs::Bitmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "Timer prescaler register"]
    #[inline(always)]
    pub fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize)) }
    }
    #[doc = "Description collection\\[n\\]: Capture/Compare register n"]
    #[inline(always)]
    pub fn cc(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(1344usize + n * 4usize)) }
    }
}
pub mod regs;
pub mod vals;
