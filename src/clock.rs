#[doc = "Clock control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clock(pub *mut u8);
unsafe impl Send for Clock {}
unsafe impl Sync for Clock {}
impl Clock {
    #[doc = "Start HFXO crystal oscillator"]
    #[inline(always)]
    pub fn tasks_hfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Stop HFXO crystal oscillator"]
    #[inline(always)]
    pub fn tasks_hfclkstop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Start LFCLK"]
    #[inline(always)]
    pub fn tasks_lfclkstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Stop LFCLK"]
    #[inline(always)]
    pub fn tasks_lfclkstop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Start calibration of LFRC"]
    #[inline(always)]
    pub fn tasks_cal(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Start calibration timer"]
    #[inline(always)]
    pub fn tasks_ctstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Stop calibration timer"]
    #[inline(always)]
    pub fn tasks_ctstop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "HFXO crystal oscillator started"]
    #[inline(always)]
    pub fn events_hfclkstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "LFCLK started"]
    #[inline(always)]
    pub fn events_lfclkstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "Calibration of LFRC completed"]
    #[inline(always)]
    pub fn events_done(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(268usize)) }
    }
    #[doc = "Calibration timer timeout"]
    #[inline(always)]
    pub fn events_ctto(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
    }
    #[doc = "Calibration timer has been started and is ready to process new tasks"]
    #[inline(always)]
    pub fn events_ctstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }
    #[doc = "Calibration timer has been stopped and is ready to process new tasks"]
    #[inline(always)]
    pub fn events_ctstopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(300usize)) }
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
    #[doc = "Status indicating that HFCLKSTART task has been triggered"]
    #[inline(always)]
    pub fn hfclkrun(self) -> crate::common::Reg<regs::Hfclkrun, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1032usize)) }
    }
    #[doc = "HFCLK status"]
    #[inline(always)]
    pub fn hfclkstat(self) -> crate::common::Reg<regs::Hfclkstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1036usize)) }
    }
    #[doc = "Status indicating that LFCLKSTART task has been triggered"]
    #[inline(always)]
    pub fn lfclkrun(self) -> crate::common::Reg<regs::Lfclkrun, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1044usize)) }
    }
    #[doc = "LFCLK status"]
    #[inline(always)]
    pub fn lfclkstat(self) -> crate::common::Reg<regs::Lfclkstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1048usize)) }
    }
    #[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
    #[inline(always)]
    pub fn lfclksrccopy(self) -> crate::common::Reg<regs::Lfclksrccopy, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1052usize)) }
    }
    #[doc = "Clock source for the LFCLK"]
    #[inline(always)]
    pub fn lfclksrc(self) -> crate::common::Reg<regs::Lfclksrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1304usize)) }
    }
    #[doc = "HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task."]
    #[inline(always)]
    pub fn hfxodebounce(self) -> crate::common::Reg<regs::Hfxodebounce, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1320usize)) }
    }
    #[doc = "Calibration timer interval"]
    #[inline(always)]
    pub fn ctiv(self) -> crate::common::Reg<regs::Ctiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1336usize)) }
    }
    #[doc = "Clocking options for the trace port debug interface"]
    #[inline(always)]
    pub fn traceconfig(self) -> crate::common::Reg<regs::Traceconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1372usize)) }
    }
    #[doc = "LFRC mode configuration"]
    #[inline(always)]
    pub fn lfrcmode(self) -> crate::common::Reg<regs::Lfrcmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1460usize)) }
    }
}
pub mod regs;
pub mod vals;
