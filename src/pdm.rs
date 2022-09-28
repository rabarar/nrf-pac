#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sample(pub *mut u8);
unsafe impl Send for Sample {}
unsafe impl Sync for Sample {}
impl Sample {
    #[doc = "RAM address pointer to write samples to with EasyDMA"]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Number of samples to allocate memory for in EasyDMA mode"]
    #[inline(always)]
    pub fn maxcnt(self) -> crate::common::Reg<regs::Maxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
#[doc = "Pulse Density Modulation (Digital Microphone) Interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdm(pub *mut u8);
unsafe impl Send for Pdm {}
unsafe impl Sync for Pdm {}
impl Pdm {
    #[doc = "Starts continuous PDM transfer"]
    #[inline(always)]
    pub fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Stops PDM transfer"]
    #[inline(always)]
    pub fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "PDM transfer has started"]
    #[inline(always)]
    pub fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "PDM transfer has finished"]
    #[inline(always)]
    pub fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
    #[inline(always)]
    pub fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
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
    #[doc = "PDM module enable register"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "PDM clock generator control"]
    #[inline(always)]
    pub fn pdmclkctrl(self) -> crate::common::Reg<regs::Pdmclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "Defines the routing of the connected PDM microphones' signals"]
    #[inline(always)]
    pub fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "Left output gain adjustment"]
    #[inline(always)]
    pub fn gainl(self) -> crate::common::Reg<regs::Gainl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1304usize)) }
    }
    #[doc = "Right output gain adjustment"]
    #[inline(always)]
    pub fn gainr(self) -> crate::common::Reg<regs::Gainr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1308usize)) }
    }
    #[doc = "Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly."]
    #[inline(always)]
    pub fn ratio(self) -> crate::common::Reg<regs::Ratio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1312usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn psel(self) -> Psel {
        unsafe { Psel(self.0.add(1344usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn sample(self) -> Sample {
        unsafe { Sample(self.0.add(1376usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psel(pub *mut u8);
unsafe impl Send for Psel {}
unsafe impl Sync for Psel {}
impl Psel {
    #[doc = "Pin number configuration for PDM CLK signal"]
    #[inline(always)]
    pub fn clk(self) -> crate::common::Reg<regs::Clk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Pin number configuration for PDM DIN signal"]
    #[inline(always)]
    pub fn din(self) -> crate::common::Reg<regs::Din, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
pub mod regs;
pub mod vals;
