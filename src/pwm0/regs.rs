#[doc = "Configuration of the decoder"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Decoder(pub u32);
impl Decoder {
    #[doc = "How a sequence is read from RAM and spread to the compare register"]
    #[inline(always)]
    pub const fn load(&self) -> super::vals::Load {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Load(val as u8)
    }
    #[doc = "How a sequence is read from RAM and spread to the compare register"]
    #[inline(always)]
    pub fn set_load(&mut self, val: super::vals::Load) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
    #[doc = "Selects source for advancing the active sequence"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mode(val as u8)
    }
    #[doc = "Selects source for advancing the active sequence"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.0 as u32) & 0x01) << 8usize);
    }
}
impl Default for Decoder {
    #[inline(always)]
    fn default() -> Decoder {
        Decoder(0)
    }
}
#[doc = "Configuration for PWM_CLK"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prescaler(pub u32);
impl Prescaler {
    #[doc = "Prescaler of PWM_CLK"]
    #[inline(always)]
    pub const fn prescaler(&self) -> super::vals::Prescaler {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Prescaler(val as u8)
    }
    #[doc = "Prescaler of PWM_CLK"]
    #[inline(always)]
    pub fn set_prescaler(&mut self, val: super::vals::Prescaler) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.0 as u32) & 0x07) << 0usize);
    }
}
impl Default for Prescaler {
    #[inline(always)]
    fn default() -> Prescaler {
        Prescaler(0)
    }
}
#[doc = "Description cluster\\[n\\]: Time added after the sequence"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enddelay(pub u32);
impl Enddelay {
    #[doc = "Time added after the sequence in PWM periods"]
    #[inline(always)]
    pub const fn cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Time added after the sequence in PWM periods"]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Enddelay {
    #[inline(always)]
    fn default() -> Enddelay {
        Enddelay(0)
    }
}
#[doc = "Description collection\\[n\\]: Output pin select for PWM channel n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Out(pub u32);
impl Out {
    #[doc = "Pin number"]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Pin number"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub const fn port(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub fn set_port(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::Connect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Connect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::Connect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Out {
    #[inline(always)]
    fn default() -> Out {
        Out(0)
    }
}
#[doc = "Shortcut register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between SEQEND\\[0\\] event and STOP task"]
    #[inline(always)]
    pub const fn seqend0_stop(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between SEQEND\\[0\\] event and STOP task"]
    #[inline(always)]
    pub fn set_seqend0_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Shortcut between SEQEND\\[1\\] event and STOP task"]
    #[inline(always)]
    pub const fn seqend1_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between SEQEND\\[1\\] event and STOP task"]
    #[inline(always)]
    pub fn set_seqend1_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Shortcut between LOOPSDONE event and SEQSTART\\[0\\] task"]
    #[inline(always)]
    pub const fn loopsdone_seqstart0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between LOOPSDONE event and SEQSTART\\[0\\] task"]
    #[inline(always)]
    pub fn set_loopsdone_seqstart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Shortcut between LOOPSDONE event and SEQSTART\\[1\\] task"]
    #[inline(always)]
    pub const fn loopsdone_seqstart1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between LOOPSDONE event and SEQSTART\\[1\\] task"]
    #[inline(always)]
    pub fn set_loopsdone_seqstart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Shortcut between LOOPSDONE event and STOP task"]
    #[inline(always)]
    pub const fn loopsdone_stop(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between LOOPSDONE event and STOP task"]
    #[inline(always)]
    pub fn set_loopsdone_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "Number of playbacks of a loop"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Loop(pub u32);
impl Loop {
    #[doc = "Number of playbacks of pattern cycles"]
    #[inline(always)]
    pub const fn cnt(&self) -> super::vals::LoopCnt {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::LoopCnt(val as u16)
    }
    #[doc = "Number of playbacks of pattern cycles"]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: super::vals::LoopCnt) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.0 as u32) & 0xffff) << 0usize);
    }
}
impl Default for Loop {
    #[inline(always)]
    fn default() -> Loop {
        Loop(0)
    }
}
#[doc = "Value up to which the pulse generator counter counts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Countertop(pub u32);
impl Countertop {
    #[doc = "Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM are used."]
    #[inline(always)]
    pub const fn countertop(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM are used."]
    #[inline(always)]
    pub fn set_countertop(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Countertop {
    #[inline(always)]
    fn default() -> Countertop {
        Countertop(0)
    }
}
#[doc = "Description cluster\\[n\\]: Number of additional PWM periods between samples loaded into compare register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Refresh(pub u32);
impl Refresh {
    #[doc = "Number of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
    #[inline(always)]
    pub const fn cnt(&self) -> super::vals::RefreshCnt {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        super::vals::RefreshCnt(val as u32)
    }
    #[doc = "Number of additional PWM periods between samples loaded into compare register (load every REFRESH.CNT+1 PWM periods)"]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: super::vals::RefreshCnt) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val.0 as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Refresh {
    #[inline(always)]
    fn default() -> Refresh {
        Refresh(0)
    }
}
#[doc = "Description cluster\\[n\\]: Number of values (duty cycles) in this sequence"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "Number of values (duty cycles) in this sequence"]
    #[inline(always)]
    pub const fn cnt(&self) -> super::vals::CntCnt {
        let val = (self.0 >> 0usize) & 0x7fff;
        super::vals::CntCnt(val as u16)
    }
    #[doc = "Number of values (duty cycles) in this sequence"]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: super::vals::CntCnt) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val.0 as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Cnt {
    #[inline(always)]
    fn default() -> Cnt {
        Cnt(0)
    }
}
#[doc = "Selects operating mode of the wave counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "Selects up mode or up-and-down mode for the counter"]
    #[inline(always)]
    pub const fn updown(&self) -> super::vals::Updown {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Updown(val as u8)
    }
    #[doc = "Selects up mode or up-and-down mode for the counter"]
    #[inline(always)]
    pub fn set_updown(&mut self, val: super::vals::Updown) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
#[doc = "PWM module enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable PWM module"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable(val as u8)
    }
    #[doc = "Enable or disable PWM module"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Enable {
    #[inline(always)]
    fn default() -> Enable {
        Enable(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for STOPPED event"]
    #[inline(always)]
    pub const fn stopped(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn set_stopped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable interrupt for SEQSTARTED\\[0\\] event"]
    #[inline(always)]
    pub const fn seqstarted0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for SEQSTARTED\\[0\\] event"]
    #[inline(always)]
    pub fn set_seqstarted0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to disable interrupt for SEQSTARTED\\[1\\] event"]
    #[inline(always)]
    pub const fn seqstarted1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for SEQSTARTED\\[1\\] event"]
    #[inline(always)]
    pub fn set_seqstarted1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to disable interrupt for SEQEND\\[0\\] event"]
    #[inline(always)]
    pub const fn seqend0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for SEQEND\\[0\\] event"]
    #[inline(always)]
    pub fn set_seqend0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to disable interrupt for SEQEND\\[1\\] event"]
    #[inline(always)]
    pub const fn seqend1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for SEQEND\\[1\\] event"]
    #[inline(always)]
    pub fn set_seqend1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write '1' to disable interrupt for PWMPERIODEND event"]
    #[inline(always)]
    pub const fn pwmperiodend(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for PWMPERIODEND event"]
    #[inline(always)]
    pub fn set_pwmperiodend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write '1' to disable interrupt for LOOPSDONE event"]
    #[inline(always)]
    pub const fn loopsdone(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for LOOPSDONE event"]
    #[inline(always)]
    pub fn set_loopsdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
