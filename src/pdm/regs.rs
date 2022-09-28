#[doc = "Defines the routing of the connected PDM microphones' signals"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "Mono or stereo operation"]
    #[inline(always)]
    pub const fn operation(&self) -> super::vals::Operation {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Operation(val as u8)
    }
    #[doc = "Mono or stereo operation"]
    #[inline(always)]
    pub fn set_operation(&mut self, val: super::vals::Operation) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Defines on which PDM_CLK edge Left (or mono) is sampled"]
    #[inline(always)]
    pub const fn edge(&self) -> super::vals::Edge {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Edge(val as u8)
    }
    #[doc = "Defines on which PDM_CLK edge Left (or mono) is sampled"]
    #[inline(always)]
    pub fn set_edge(&mut self, val: super::vals::Edge) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
#[doc = "PDM clock generator control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdmclkctrl(pub u32);
impl Pdmclkctrl {
    #[doc = "PDM_CLK frequency"]
    #[inline(always)]
    pub const fn freq(&self) -> super::vals::Freq {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Freq(val as u32)
    }
    #[doc = "PDM_CLK frequency"]
    #[inline(always)]
    pub fn set_freq(&mut self, val: super::vals::Freq) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.0 as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pdmclkctrl {
    #[inline(always)]
    fn default() -> Pdmclkctrl {
        Pdmclkctrl(0)
    }
}
#[doc = "Number of samples to allocate memory for in EasyDMA mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Maxcnt(pub u32);
impl Maxcnt {
    #[doc = "Length of DMA RAM allocation in number of samples"]
    #[inline(always)]
    pub const fn buffsize(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Length of DMA RAM allocation in number of samples"]
    #[inline(always)]
    pub fn set_buffsize(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Maxcnt {
    #[inline(always)]
    fn default() -> Maxcnt {
        Maxcnt(0)
    }
}
#[doc = "PDM module enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable PDM module"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable(val as u8)
    }
    #[doc = "Enable or disable PDM module"]
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
#[doc = "Left output gain adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gainl(pub u32);
impl Gainl {
    #[doc = "Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
    #[inline(always)]
    pub const fn gainl(&self) -> super::vals::Gainl {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Gainl(val as u8)
    }
    #[doc = "Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
    #[inline(always)]
    pub fn set_gainl(&mut self, val: super::vals::Gainl) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.0 as u32) & 0x7f) << 0usize);
    }
}
impl Default for Gainl {
    #[inline(always)]
    fn default() -> Gainl {
        Gainl(0)
    }
}
#[doc = "Right output gain adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gainr(pub u32);
impl Gainr {
    #[doc = "Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
    #[inline(always)]
    pub const fn gainr(&self) -> super::vals::Gainr {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Gainr(val as u8)
    }
    #[doc = "Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
    #[inline(always)]
    pub fn set_gainr(&mut self, val: super::vals::Gainr) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.0 as u32) & 0x7f) << 0usize);
    }
}
impl Default for Gainr {
    #[inline(always)]
    fn default() -> Gainr {
        Gainr(0)
    }
}
#[doc = "Pin number configuration for PDM DIN signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Din(pub u32);
impl Din {
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
    pub const fn connect(&self) -> super::vals::DinConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::DinConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::DinConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Din {
    #[inline(always)]
    fn default() -> Din {
        Din(0)
    }
}
#[doc = "Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ratio(pub u32);
impl Ratio {
    #[doc = "Selects the ratio between PDM_CLK and output sample rate"]
    #[inline(always)]
    pub const fn ratio(&self) -> super::vals::Ratio {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ratio(val as u8)
    }
    #[doc = "Selects the ratio between PDM_CLK and output sample rate"]
    #[inline(always)]
    pub fn set_ratio(&mut self, val: super::vals::Ratio) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Ratio {
    #[inline(always)]
    fn default() -> Ratio {
        Ratio(0)
    }
}
#[doc = "Pin number configuration for PDM CLK signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clk(pub u32);
impl Clk {
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
    pub const fn connect(&self) -> super::vals::ClkConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ClkConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::ClkConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Clk {
    #[inline(always)]
    fn default() -> Clk {
        Clk(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for STARTED event"]
    #[inline(always)]
    pub const fn started(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for STARTED event"]
    #[inline(always)]
    pub fn set_started(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
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
    #[doc = "Write '1' to disable interrupt for END event"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for END event"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
