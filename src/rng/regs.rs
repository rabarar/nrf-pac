#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Bias correction"]
    #[inline(always)]
    pub const fn dercen(&self) -> super::vals::Dercen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dercen(val as u8)
    }
    #[doc = "Bias correction"]
    #[inline(always)]
    pub fn set_dercen(&mut self, val: super::vals::Dercen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
#[doc = "Output random number"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Value(pub u32);
impl Value {
    #[doc = "Generated random number"]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Generated random number"]
    #[inline(always)]
    pub fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Value {
    #[inline(always)]
    fn default() -> Value {
        Value(0)
    }
}
#[doc = "Shortcut register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between VALRDY event and STOP task"]
    #[inline(always)]
    pub const fn valrdy_stop(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between VALRDY event and STOP task"]
    #[inline(always)]
    pub fn set_valrdy_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for VALRDY event"]
    #[inline(always)]
    pub const fn valrdy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for VALRDY event"]
    #[inline(always)]
    pub fn set_valrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
