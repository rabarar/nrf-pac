#[doc = "Low Power Comparator"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpcomp(pub *mut u8);
unsafe impl Send for Lpcomp {}
unsafe impl Sync for Lpcomp {}
impl Lpcomp {
    #[doc = "Start comparator"]
    #[inline(always)]
    pub fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Stop comparator"]
    #[inline(always)]
    pub fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Sample comparator value"]
    #[inline(always)]
    pub fn tasks_sample(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "LPCOMP is ready and output is valid"]
    #[inline(always)]
    pub fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "Downward crossing"]
    #[inline(always)]
    pub fn events_down(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "Upward crossing"]
    #[inline(always)]
    pub fn events_up(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }
    #[doc = "Downward or upward crossing"]
    #[inline(always)]
    pub fn events_cross(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(268usize)) }
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
    #[doc = "Compare result"]
    #[inline(always)]
    pub fn result(self) -> crate::common::Reg<regs::Result, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1024usize)) }
    }
    #[doc = "Enable LPCOMP"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Input pin select"]
    #[inline(always)]
    pub fn psel(self) -> crate::common::Reg<regs::Psel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "Reference select"]
    #[inline(always)]
    pub fn refsel(self) -> crate::common::Reg<regs::Refsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "External reference select"]
    #[inline(always)]
    pub fn extrefsel(self) -> crate::common::Reg<regs::Extrefsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1292usize)) }
    }
    #[doc = "Analog detect configuration"]
    #[inline(always)]
    pub fn anadetect(self) -> crate::common::Reg<regs::Anadetect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1312usize)) }
    }
    #[doc = "Comparator hysteresis enable"]
    #[inline(always)]
    pub fn hyst(self) -> crate::common::Reg<regs::Hyst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1336usize)) }
    }
}
pub mod regs;
pub mod vals;
