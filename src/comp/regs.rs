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
#[doc = "External reference select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extrefsel(pub u32);
impl Extrefsel {
    #[doc = "External analog reference select"]
    #[inline(always)]
    pub const fn extrefsel(&self) -> super::vals::Extrefsel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Extrefsel(val as u8)
    }
    #[doc = "External analog reference select"]
    #[inline(always)]
    pub fn set_extrefsel(&mut self, val: super::vals::Extrefsel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.0 as u32) & 0x07) << 0usize);
    }
}
impl Default for Extrefsel {
    #[inline(always)]
    fn default() -> Extrefsel {
        Extrefsel(0)
    }
}
#[doc = "Comparator hysteresis enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hyst(pub u32);
impl Hyst {
    #[doc = "Comparator hysteresis"]
    #[inline(always)]
    pub const fn hyst(&self) -> super::vals::Hyst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hyst(val as u8)
    }
    #[doc = "Comparator hysteresis"]
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
#[doc = "COMP enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable COMP"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Enable(val as u8)
    }
    #[doc = "Enable or disable COMP"]
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
#[doc = "Pin select"]
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
#[doc = "Mode configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "Speed and power modes"]
    #[inline(always)]
    pub const fn sp(&self) -> super::vals::Sp {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sp(val as u8)
    }
    #[doc = "Speed and power modes"]
    #[inline(always)]
    pub fn set_sp(&mut self, val: super::vals::Sp) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
    #[doc = "Main operation modes"]
    #[inline(always)]
    pub const fn main(&self) -> super::vals::Main {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Main(val as u8)
    }
    #[doc = "Main operation modes"]
    #[inline(always)]
    pub fn set_main(&mut self, val: super::vals::Main) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.0 as u32) & 0x01) << 8usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
#[doc = "Reference source select for single-ended mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Refsel(pub u32);
impl Refsel {
    #[doc = "Reference select"]
    #[inline(always)]
    pub const fn refsel(&self) -> super::vals::Refsel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Refsel(val as u8)
    }
    #[doc = "Reference select"]
    #[inline(always)]
    pub fn set_refsel(&mut self, val: super::vals::Refsel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.0 as u32) & 0x07) << 0usize);
    }
}
impl Default for Refsel {
    #[inline(always)]
    fn default() -> Refsel {
        Refsel(0)
    }
}
#[doc = "Threshold configuration for hysteresis unit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Th(pub u32);
impl Th {
    #[doc = "VDOWN = (THDOWN+1)/64*VREF"]
    #[inline(always)]
    pub const fn thdown(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "VDOWN = (THDOWN+1)/64*VREF"]
    #[inline(always)]
    pub fn set_thdown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "VUP = (THUP+1)/64*VREF"]
    #[inline(always)]
    pub const fn thup(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "VUP = (THUP+1)/64*VREF"]
    #[inline(always)]
    pub fn set_thup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Th {
    #[inline(always)]
    fn default() -> Th {
        Th(0)
    }
}
