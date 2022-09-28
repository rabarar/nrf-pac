#[doc = "GPIO reference voltage / external output supply voltage in high voltage mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Regout0(pub u32);
impl Regout0 {
    #[doc = "Output voltage from of REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - VEXDIF."]
    #[inline(always)]
    pub const fn vout(&self) -> super::vals::Vout {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Vout(val as u8)
    }
    #[doc = "Output voltage from of REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - VEXDIF."]
    #[inline(always)]
    pub fn set_vout(&mut self, val: super::vals::Vout) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.0 as u32) & 0x07) << 0usize);
    }
}
impl Default for Regout0 {
    #[inline(always)]
    fn default() -> Regout0 {
        Regout0(0)
    }
}
#[doc = "Access port protection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Approtect(pub u32);
impl Approtect {
    #[doc = "Enable or disable access port protection."]
    #[inline(always)]
    pub const fn pall(&self) -> super::vals::Pall {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Pall(val as u8)
    }
    #[doc = "Enable or disable access port protection."]
    #[inline(always)]
    pub fn set_pall(&mut self, val: super::vals::Pall) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.0 as u32) & 0xff) << 0usize);
    }
}
impl Default for Approtect {
    #[inline(always)]
    fn default() -> Approtect {
        Approtect(0)
    }
}
#[doc = "Description collection\\[n\\]: Mapping of the nRESET function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pselreset(pub u32);
impl Pselreset {
    #[doc = "Pin number of PORT onto which nRESET is exposed"]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Pin number of PORT onto which nRESET is exposed"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port number onto which nRESET is exposed"]
    #[inline(always)]
    pub const fn port(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port number onto which nRESET is exposed"]
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
impl Default for Pselreset {
    #[inline(always)]
    fn default() -> Pselreset {
        Pselreset(0)
    }
}
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nfcpins(pub u32);
impl Nfcpins {
    #[doc = "Setting of pins dedicated to NFC functionality"]
    #[inline(always)]
    pub const fn protect(&self) -> super::vals::Protect {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Protect(val as u8)
    }
    #[doc = "Setting of pins dedicated to NFC functionality"]
    #[inline(always)]
    pub fn set_protect(&mut self, val: super::vals::Protect) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Nfcpins {
    #[inline(always)]
    fn default() -> Nfcpins {
        Nfcpins(0)
    }
}
#[doc = "Processor debug control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debugctrl(pub u32);
impl Debugctrl {
    #[doc = "Configure CPU non-intrusive debug features"]
    #[inline(always)]
    pub const fn cpuniden(&self) -> super::vals::Cpuniden {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Cpuniden(val as u8)
    }
    #[doc = "Configure CPU non-intrusive debug features"]
    #[inline(always)]
    pub fn set_cpuniden(&mut self, val: super::vals::Cpuniden) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.0 as u32) & 0xff) << 0usize);
    }
    #[doc = "Configure CPU flash patch and breakpoint (FPB) unit behavior"]
    #[inline(always)]
    pub const fn cpufpben(&self) -> super::vals::Cpufpben {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Cpufpben(val as u8)
    }
    #[doc = "Configure CPU flash patch and breakpoint (FPB) unit behavior"]
    #[inline(always)]
    pub fn set_cpufpben(&mut self, val: super::vals::Cpufpben) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.0 as u32) & 0xff) << 8usize);
    }
}
impl Default for Debugctrl {
    #[inline(always)]
    fn default() -> Debugctrl {
        Debugctrl(0)
    }
}
