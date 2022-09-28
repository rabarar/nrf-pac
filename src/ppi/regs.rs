#[doc = "Channel enable clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chenclr(pub u32);
impl Chenclr {
    #[doc = "Channel 0 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn set_ch(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Chenclr {
    #[inline(always)]
    fn default() -> Chenclr {
        Chenclr(0)
    }
}
#[doc = "Channel enable set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chenset(pub u32);
impl Chenset {
    #[doc = "Channel 0 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 enable set register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn set_ch(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Chenset {
    #[inline(always)]
    fn default() -> Chenset {
        Chenset(0)
    }
}
#[doc = "Description cluster\\[n\\]: Enable channel group n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct En(pub u32);
impl En {
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for En {
    #[inline(always)]
    fn default() -> En {
        En(0)
    }
}
#[doc = "Description cluster\\[n\\]: Disable channel group n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dis(pub u32);
impl Dis {
    #[inline(always)]
    pub const fn dis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Dis {
    #[inline(always)]
    fn default() -> Dis {
        Dis(0)
    }
}
#[doc = "Description collection\\[n\\]: Channel group n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chg(pub u32);
impl Chg {
    #[doc = "Include or exclude channel 0"]
    #[inline(always)]
    pub fn ch(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Include or exclude channel 0"]
    #[inline(always)]
    pub fn set_ch(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Chg {
    #[inline(always)]
    fn default() -> Chg {
        Chg(0)
    }
}
#[doc = "Channel enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chen(pub u32);
impl Chen {
    #[doc = "Enable or disable channel 0"]
    #[inline(always)]
    pub fn ch(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable channel 0"]
    #[inline(always)]
    pub fn set_ch(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Chen {
    #[inline(always)]
    fn default() -> Chen {
        Chen(0)
    }
}
