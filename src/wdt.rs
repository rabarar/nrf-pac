#[doc = "Watchdog Timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt(pub *mut u8);
unsafe impl Send for Wdt {}
unsafe impl Sync for Wdt {}
impl Wdt {
    #[doc = "Start the watchdog"]
    #[inline(always)]
    pub fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Watchdog timeout"]
    #[inline(always)]
    pub fn events_timeout(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
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
    #[doc = "Run status"]
    #[inline(always)]
    pub fn runstatus(self) -> crate::common::Reg<regs::Runstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1024usize)) }
    }
    #[doc = "Request status"]
    #[inline(always)]
    pub fn reqstatus(self) -> crate::common::Reg<regs::Reqstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1028usize)) }
    }
    #[doc = "Counter reload value"]
    #[inline(always)]
    pub fn crv(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "Enable register for reload request registers"]
    #[inline(always)]
    pub fn rren(self) -> crate::common::Reg<regs::Rren, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1292usize)) }
    }
    #[doc = "Description collection\\[n\\]: Reload request n"]
    #[inline(always)]
    pub fn rr(self, n: usize) -> crate::common::Reg<regs::Rr, crate::common::W> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(1536usize + n * 4usize)) }
    }
}
pub mod regs;
pub mod vals;
