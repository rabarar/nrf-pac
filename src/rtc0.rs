#[doc = "Real time counter 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc0(pub *mut u8);
unsafe impl Send for Rtc0 {}
unsafe impl Sync for Rtc0 {}
impl Rtc0 {
    #[doc = "Start RTC COUNTER"]
    #[inline(always)]
    pub fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Stop RTC COUNTER"]
    #[inline(always)]
    pub fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Clear RTC COUNTER"]
    #[inline(always)]
    pub fn tasks_clear(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Set COUNTER to 0xFFFFF0"]
    #[inline(always)]
    pub fn tasks_trigovrflw(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Event on COUNTER increment"]
    #[inline(always)]
    pub fn events_tick(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "Event on COUNTER overflow"]
    #[inline(always)]
    pub fn events_ovrflw(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "Description collection\\[n\\]: Compare event on CC\\[n\\] match"]
    #[inline(always)]
    pub fn events_compare(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(320usize + n * 4usize)) }
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
    #[doc = "Enable or disable event routing"]
    #[inline(always)]
    pub fn evten(self) -> crate::common::Reg<regs::Evten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(832usize)) }
    }
    #[doc = "Enable event routing"]
    #[inline(always)]
    pub fn evtenset(self) -> crate::common::Reg<regs::Evtenset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(836usize)) }
    }
    #[doc = "Disable event routing"]
    #[inline(always)]
    pub fn evtenclr(self) -> crate::common::Reg<regs::Evtenclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(840usize)) }
    }
    #[doc = "Current COUNTER value"]
    #[inline(always)]
    pub fn counter(self) -> crate::common::Reg<regs::Counter, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped"]
    #[inline(always)]
    pub fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "Description collection\\[n\\]: Compare register n"]
    #[inline(always)]
    pub fn cc(self, n: usize) -> crate::common::Reg<regs::Cc, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(1344usize + n * 4usize)) }
    }
}
pub mod regs;
pub mod vals;
