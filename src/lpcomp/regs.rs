#[doc = "Shortcut register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between READY event and SAMPLE task"]
    #[inline(always)]
    pub const fn ready_sample(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between READY event and SAMPLE task"]
    #[inline(always)]
    pub fn set_ready_sample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Shortcut between READY event and STOP task"]
    #[inline(always)]
    pub const fn ready_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between READY event and STOP task"]
    #[inline(always)]
    pub fn set_ready_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Shortcut between DOWN event and STOP task"]
    #[inline(always)]
    pub const fn down_stop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between DOWN event and STOP task"]
    #[inline(always)]
    pub fn set_down_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Shortcut between UP event and STOP task"]
    #[inline(always)]
    pub const fn up_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between UP event and STOP task"]
    #[inline(always)]
    pub fn set_up_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Shortcut between CROSS event and STOP task"]
    #[inline(always)]
    pub const fn cross_stop(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between CROSS event and STOP task"]
    #[inline(always)]
    pub fn set_cross_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "Enable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to enable interrupt for READY event"]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for READY event"]
    #[inline(always)]
    pub fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to enable interrupt for DOWN event"]
    #[inline(always)]
    pub const fn down(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for DOWN event"]
    #[inline(always)]
    pub fn set_down(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to enable interrupt for UP event"]
    #[inline(always)]
    pub const fn up(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for UP event"]
    #[inline(always)]
    pub fn set_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to enable interrupt for CROSS event"]
    #[inline(always)]
    pub const fn cross(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for CROSS event"]
    #[inline(always)]
    pub fn set_cross(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Comparator hysteresis enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hyst(pub u32);
impl Hyst {
    #[doc = "Comparator hysteresis enable"]
    #[inline(always)]
    pub const fn hyst(&self) -> super::vals::Hyst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hyst(val as u8)
    }
    #[doc = "Comparator hysteresis enable"]
    #[inline(always)]
    pub fn set_hyst(&mut self, val: super::vals::Hyst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Hyst {
    #[inline(always)]
    fn default() -> Hyst {
        Hyst(0)
    }
}
#[doc = "Enable LPCOMP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable LPCOMP"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Enable(val as u8)
    }
    #[doc = "Enable or disable LPCOMP"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
}
impl Default for Enable {
    #[inline(always)]
    fn default() -> Enable {
        Enable(0)
    }
}
#[doc = "Analog detect configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anadetect(pub u32);
impl Anadetect {
    #[doc = "Analog detect configuration"]
    #[inline(always)]
    pub const fn anadetect(&self) -> super::vals::Anadetect {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Anadetect(val as u8)
    }
    #[doc = "Analog detect configuration"]
    #[inline(always)]
    pub fn set_anadetect(&mut self, val: super::vals::Anadetect) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
}
impl Default for Anadetect {
    #[inline(always)]
    fn default() -> Anadetect {
        Anadetect(0)
    }
}
#[doc = "Input pin select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psel(pub u32);
impl Psel {
    #[doc = "Analog pin select"]
    #[inline(always)]
    pub const fn psel(&self) -> super::vals::Psel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Psel(val as u8)
    }
    #[doc = "Analog pin select"]
    #[inline(always)]
    pub fn set_psel(&mut self, val: super::vals::Psel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.0 as u32) & 0x07) << 0usize);
    }
}
impl Default for Psel {
    #[inline(always)]
    fn default() -> Psel {
        Psel(0)
    }
}
#[doc = "Reference select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Refsel(pub u32);
impl Refsel {
    #[doc = "Reference select"]
    #[inline(always)]
    pub const fn refsel(&self) -> super::vals::Refsel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Refsel(val as u8)
    }
    #[doc = "Reference select"]
    #[inline(always)]
    pub fn set_refsel(&mut self, val: super::vals::Refsel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.0 as u32) & 0x0f) << 0usize);
    }
}
impl Default for Refsel {
    #[inline(always)]
    fn default() -> Refsel {
        Refsel(0)
    }
}
#[doc = "External reference select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extrefsel(pub u32);
impl Extrefsel {
    #[doc = "External analog reference select"]
    #[inline(always)]
    pub const fn extrefsel(&self) -> super::vals::Extrefsel {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Extrefsel(val as u8)
    }
    #[doc = "External analog reference select"]
    #[inline(always)]
    pub fn set_extrefsel(&mut self, val: super::vals::Extrefsel) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Extrefsel {
    #[inline(always)]
    fn default() -> Extrefsel {
        Extrefsel(0)
    }
}
#[doc = "Compare result"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Result(pub u32);
impl Result {
    #[doc = "Result of last compare. Decision point SAMPLE task."]
    #[inline(always)]
    pub const fn result(&self) -> super::vals::Result {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Result(val as u8)
    }
    #[doc = "Result of last compare. Decision point SAMPLE task."]
    #[inline(always)]
    pub fn set_result(&mut self, val: super::vals::Result) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Result {
    #[inline(always)]
    fn default() -> Result {
        Result(0)
    }
}
