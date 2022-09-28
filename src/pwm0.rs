#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psel(pub *mut u8);
unsafe impl Send for Psel {}
unsafe impl Sync for Psel {}
impl Psel {
    #[doc = "Description collection\\[n\\]: Output pin select for PWM channel n"]
    #[inline(always)]
    pub fn out(self, n: usize) -> crate::common::Reg<regs::Out, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize + n * 4usize)) }
    }
}
#[doc = "Pulse width modulation unit 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm0(pub *mut u8);
unsafe impl Send for Pwm0 {}
unsafe impl Sync for Pwm0 {}
impl Pwm0 {
    #[doc = "Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback"]
    #[inline(always)]
    pub fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Description collection\\[n\\]: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
    #[inline(always)]
    pub fn tasks_seqstart(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize + n * 4usize)) }
    }
    #[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
    #[inline(always)]
    pub fn tasks_nextstep(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Response to STOP task, emitted when PWM pulses are no longer generated"]
    #[inline(always)]
    pub fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "Description collection\\[n\\]: First PWM period started on sequence n"]
    #[inline(always)]
    pub fn events_seqstarted(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize + n * 4usize)) }
    }
    #[doc = "Description collection\\[n\\]: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
    #[inline(always)]
    pub fn events_seqend(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize + n * 4usize)) }
    }
    #[doc = "Emitted at the end of each PWM period"]
    #[inline(always)]
    pub fn events_pwmperiodend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(280usize)) }
    }
    #[doc = "Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
    #[inline(always)]
    pub fn events_loopsdone(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(284usize)) }
    }
    #[doc = "Shortcut register"]
    #[inline(always)]
    pub fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(512usize)) }
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
    #[doc = "PWM module enable register"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Selects operating mode of the wave counter"]
    #[inline(always)]
    pub fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "Value up to which the pulse generator counter counts"]
    #[inline(always)]
    pub fn countertop(self) -> crate::common::Reg<regs::Countertop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "Configuration for PWM_CLK"]
    #[inline(always)]
    pub fn prescaler(self) -> crate::common::Reg<regs::Prescaler, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1292usize)) }
    }
    #[doc = "Configuration of the decoder"]
    #[inline(always)]
    pub fn decoder(self) -> crate::common::Reg<regs::Decoder, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize)) }
    }
    #[doc = "Number of playbacks of a loop"]
    #[inline(always)]
    pub fn loop_(self) -> crate::common::Reg<regs::Loop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1300usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn seq(self, n: usize) -> Seq {
        assert!(n < 2usize);
        unsafe { Seq(self.0.add(1312usize + n * 32usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn psel(self) -> Psel {
        unsafe { Psel(self.0.add(1376usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seq(pub *mut u8);
unsafe impl Send for Seq {}
unsafe impl Sync for Seq {}
impl Seq {
    #[doc = "Description cluster\\[n\\]: Beginning address in RAM of this sequence"]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Number of values (duty cycles) in this sequence"]
    #[inline(always)]
    pub fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Number of additional PWM periods between samples loaded into compare register"]
    #[inline(always)]
    pub fn refresh(self) -> crate::common::Reg<regs::Refresh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Time added after the sequence"]
    #[inline(always)]
    pub fn enddelay(self) -> crate::common::Reg<regs::Enddelay, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
pub mod regs;
pub mod vals;
