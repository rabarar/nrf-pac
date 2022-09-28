#[doc = "Clocking options for the trace port debug interface"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Traceconfig(pub u32);
impl Traceconfig {
    #[doc = "Speed of trace port clock. Note that the TRACECLK pin will output this clock divided by two."]
    #[inline(always)]
    pub const fn traceportspeed(&self) -> super::vals::Traceportspeed {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Traceportspeed(val as u8)
    }
    #[doc = "Speed of trace port clock. Note that the TRACECLK pin will output this clock divided by two."]
    #[inline(always)]
    pub fn set_traceportspeed(&mut self, val: super::vals::Traceportspeed) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
    #[doc = "Pin multiplexing of trace signals. See pin assignment chapter for more details."]
    #[inline(always)]
    pub const fn tracemux(&self) -> super::vals::Tracemux {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Tracemux(val as u8)
    }
    #[doc = "Pin multiplexing of trace signals. See pin assignment chapter for more details."]
    #[inline(always)]
    pub fn set_tracemux(&mut self, val: super::vals::Tracemux) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.0 as u32) & 0x03) << 16usize);
    }
}
impl Default for Traceconfig {
    #[inline(always)]
    fn default() -> Traceconfig {
        Traceconfig(0)
    }
}
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfclkrun(pub u32);
impl Lfclkrun {
    #[doc = "LFCLKSTART task triggered or not"]
    #[inline(always)]
    pub const fn status(&self) -> super::vals::LfclkrunStatus {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LfclkrunStatus(val as u8)
    }
    #[doc = "LFCLKSTART task triggered or not"]
    #[inline(always)]
    pub fn set_status(&mut self, val: super::vals::LfclkrunStatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Lfclkrun {
    #[inline(always)]
    fn default() -> Lfclkrun {
        Lfclkrun(0)
    }
}
#[doc = "LFRC mode configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfrcmode(pub u32);
impl Lfrcmode {
    #[doc = "Set LFRC mode"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mode(val as u8)
    }
    #[doc = "Set LFRC mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Active LFRC mode. This field is read only."]
    #[inline(always)]
    pub const fn status(&self) -> super::vals::LfrcmodeStatus {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::LfrcmodeStatus(val as u8)
    }
    #[doc = "Active LFRC mode. This field is read only."]
    #[inline(always)]
    pub fn set_status(&mut self, val: super::vals::LfrcmodeStatus) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
    }
}
impl Default for Lfrcmode {
    #[inline(always)]
    fn default() -> Lfrcmode {
        Lfrcmode(0)
    }
}
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfclksrccopy(pub u32);
impl Lfclksrccopy {
    #[doc = "Clock source"]
    #[inline(always)]
    pub const fn src(&self) -> super::vals::LfclksrccopySrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::LfclksrccopySrc(val as u8)
    }
    #[doc = "Clock source"]
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::LfclksrccopySrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
}
impl Default for Lfclksrccopy {
    #[inline(always)]
    fn default() -> Lfclksrccopy {
        Lfclksrccopy(0)
    }
}
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfclkrun(pub u32);
impl Hfclkrun {
    #[doc = "HFCLKSTART task triggered or not"]
    #[inline(always)]
    pub const fn status(&self) -> super::vals::HfclkrunStatus {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::HfclkrunStatus(val as u8)
    }
    #[doc = "HFCLKSTART task triggered or not"]
    #[inline(always)]
    pub fn set_status(&mut self, val: super::vals::HfclkrunStatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Hfclkrun {
    #[inline(always)]
    fn default() -> Hfclkrun {
        Hfclkrun(0)
    }
}
#[doc = "LFCLK status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfclkstat(pub u32);
impl Lfclkstat {
    #[doc = "Source of LFCLK"]
    #[inline(always)]
    pub const fn src(&self) -> super::vals::LfclkstatSrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::LfclkstatSrc(val as u8)
    }
    #[doc = "Source of LFCLK"]
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::LfclkstatSrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
    #[doc = "LFCLK state"]
    #[inline(always)]
    pub const fn state(&self) -> super::vals::LfclkstatState {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::LfclkstatState(val as u8)
    }
    #[doc = "LFCLK state"]
    #[inline(always)]
    pub fn set_state(&mut self, val: super::vals::LfclkstatState) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
    }
}
impl Default for Lfclkstat {
    #[inline(always)]
    fn default() -> Lfclkstat {
        Lfclkstat(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for HFCLKSTARTED event"]
    #[inline(always)]
    pub const fn hfclkstarted(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for HFCLKSTARTED event"]
    #[inline(always)]
    pub fn set_hfclkstarted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable interrupt for LFCLKSTARTED event"]
    #[inline(always)]
    pub const fn lfclkstarted(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for LFCLKSTARTED event"]
    #[inline(always)]
    pub fn set_lfclkstarted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable interrupt for DONE event"]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for DONE event"]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to disable interrupt for CTTO event"]
    #[inline(always)]
    pub const fn ctto(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for CTTO event"]
    #[inline(always)]
    pub fn set_ctto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to disable interrupt for CTSTARTED event"]
    #[inline(always)]
    pub const fn ctstarted(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for CTSTARTED event"]
    #[inline(always)]
    pub fn set_ctstarted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write '1' to disable interrupt for CTSTOPPED event"]
    #[inline(always)]
    pub const fn ctstopped(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for CTSTOPPED event"]
    #[inline(always)]
    pub fn set_ctstopped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfxodebounce(pub u32);
impl Hfxodebounce {
    #[doc = "HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
    #[inline(always)]
    pub const fn hfxodebounce(&self) -> super::vals::Hfxodebounce {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Hfxodebounce(val as u8)
    }
    #[doc = "HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
    #[inline(always)]
    pub fn set_hfxodebounce(&mut self, val: super::vals::Hfxodebounce) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.0 as u32) & 0xff) << 0usize);
    }
}
impl Default for Hfxodebounce {
    #[inline(always)]
    fn default() -> Hfxodebounce {
        Hfxodebounce(0)
    }
}
#[doc = "Calibration timer interval"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctiv(pub u32);
impl Ctiv {
    #[doc = "Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
    #[inline(always)]
    pub const fn ctiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
    #[inline(always)]
    pub fn set_ctiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctiv {
    #[inline(always)]
    fn default() -> Ctiv {
        Ctiv(0)
    }
}
#[doc = "HFCLK status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hfclkstat(pub u32);
impl Hfclkstat {
    #[doc = "Source of HFCLK"]
    #[inline(always)]
    pub const fn src(&self) -> super::vals::HfclkstatSrc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::HfclkstatSrc(val as u8)
    }
    #[doc = "Source of HFCLK"]
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::HfclkstatSrc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "HFCLK state"]
    #[inline(always)]
    pub const fn state(&self) -> super::vals::HfclkstatState {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::HfclkstatState(val as u8)
    }
    #[doc = "HFCLK state"]
    #[inline(always)]
    pub fn set_state(&mut self, val: super::vals::HfclkstatState) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
    }
}
impl Default for Hfclkstat {
    #[inline(always)]
    fn default() -> Hfclkstat {
        Hfclkstat(0)
    }
}
#[doc = "Clock source for the LFCLK"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lfclksrc(pub u32);
impl Lfclksrc {
    #[doc = "Clock source"]
    #[inline(always)]
    pub const fn src(&self) -> super::vals::LfclksrcSrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::LfclksrcSrc(val as u8)
    }
    #[doc = "Clock source"]
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::LfclksrcSrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub const fn bypass(&self) -> super::vals::Bypass {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Bypass(val as u8)
    }
    #[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub fn set_bypass(&mut self, val: super::vals::Bypass) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable or disable external source for LFCLK"]
    #[inline(always)]
    pub const fn external(&self) -> super::vals::External {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::External(val as u8)
    }
    #[doc = "Enable or disable external source for LFCLK"]
    #[inline(always)]
    pub fn set_external(&mut self, val: super::vals::External) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.0 as u32) & 0x01) << 17usize);
    }
}
impl Default for Lfclksrc {
    #[inline(always)]
    fn default() -> Lfclksrc {
        Lfclksrc(0)
    }
}
