#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch(pub *mut u8);
unsafe impl Send for Ch {}
unsafe impl Sync for Ch {}
impl Ch {
    #[doc = "Description cluster\\[n\\]: Input positive pin selection for CH\\[n\\]"]
    #[inline(always)]
    pub fn pselp(self) -> crate::common::Reg<regs::Pselp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Input negative pin selection for CH\\[n\\]"]
    #[inline(always)]
    pub fn pseln(self) -> crate::common::Reg<regs::Pseln, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Input configuration for CH\\[n\\]"]
    #[inline(always)]
    pub fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Description cluster\\[n\\]: High/low limits for event monitoring of a channel"]
    #[inline(always)]
    pub fn limit(self) -> crate::common::Reg<regs::Limit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EventsCh(pub *mut u8);
unsafe impl Send for EventsCh {}
unsafe impl Sync for EventsCh {}
impl EventsCh {
    #[doc = "Description cluster\\[n\\]: Last result is equal or above CH\\[n\\].LIMIT.HIGH"]
    #[inline(always)]
    pub fn limith(self) -> crate::common::Reg<regs::Limith, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Last result is equal or below CH\\[n\\].LIMIT.LOW"]
    #[inline(always)]
    pub fn limitl(self) -> crate::common::Reg<regs::Limitl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
#[doc = "Successive approximation register (SAR) analog-to-digital converter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Saadc(pub *mut u8);
unsafe impl Send for Saadc {}
unsafe impl Sync for Saadc {}
impl Saadc {
    #[doc = "Starts the SAADC and prepares the result buffer in RAM"]
    #[inline(always)]
    pub fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Takes one SAADC sample"]
    #[inline(always)]
    pub fn tasks_sample(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Stops the SAADC and terminates all on-going conversions"]
    #[inline(always)]
    pub fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Starts offset auto-calibration"]
    #[inline(always)]
    pub fn tasks_calibrateoffset(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "The SAADC has started"]
    #[inline(always)]
    pub fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "The SAADC has filled up the result buffer"]
    #[inline(always)]
    pub fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "A conversion task has been completed. Depending on the configuration, multiple conversions might be needed for a result to be transferred to RAM."]
    #[inline(always)]
    pub fn events_done(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }
    #[doc = "Result ready for transfer to RAM"]
    #[inline(always)]
    pub fn events_resultdone(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(268usize)) }
    }
    #[doc = "Calibration is complete"]
    #[inline(always)]
    pub fn events_calibratedone(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
    }
    #[doc = "The SAADC has stopped"]
    #[inline(always)]
    pub fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(276usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn events_ch(self, n: usize) -> EventsCh {
        assert!(n < 8usize);
        unsafe { EventsCh(self.0.add(280usize + n * 8usize)) }
    }
    #[doc = "Enable or disable interrupt"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(768usize)) }
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
    #[doc = "Status"]
    #[inline(always)]
    pub fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1024usize)) }
    }
    #[doc = "Enable or disable SAADC"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn ch(self, n: usize) -> Ch {
        assert!(n < 8usize);
        unsafe { Ch(self.0.add(1296usize + n * 16usize)) }
    }
    #[doc = "Resolution configuration"]
    #[inline(always)]
    pub fn resolution(self) -> crate::common::Reg<regs::Resolution, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1520usize)) }
    }
    #[doc = "Oversampling configuration. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
    #[inline(always)]
    pub fn oversample(self) -> crate::common::Reg<regs::Oversample, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1524usize)) }
    }
    #[doc = "Controls normal or continuous sample rate"]
    #[inline(always)]
    pub fn samplerate(self) -> crate::common::Reg<regs::Samplerate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1528usize)) }
    }
    #[doc = "RESULT EasyDMA channel"]
    #[inline(always)]
    pub fn result(self) -> Result {
        unsafe { Result(self.0.add(1580usize)) }
    }
}
#[doc = "RESULT EasyDMA channel"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Result(pub *mut u8);
unsafe impl Send for Result {}
unsafe impl Sync for Result {}
impl Result {
    #[doc = "Data pointer"]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Maximum number of 16-bit samples to be written to output RAM buffer"]
    #[inline(always)]
    pub fn maxcnt(self) -> crate::common::Reg<regs::Maxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Number of 16-bit samples written to output RAM buffer since the previous START task"]
    #[inline(always)]
    pub fn amount(self) -> crate::common::Reg<regs::Amount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
pub mod regs;
pub mod vals;
