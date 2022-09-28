#[doc = "Select between default DETECT signal behaviour and LDETECT mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Detectmode(pub u32);
impl Detectmode {
    #[doc = "Select between default DETECT signal behaviour and LDETECT mode"]
    #[inline(always)]
    pub const fn detectmode(&self) -> super::vals::Detectmode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Detectmode(val as u8)
    }
    #[doc = "Select between default DETECT signal behaviour and LDETECT mode"]
    #[inline(always)]
    pub fn set_detectmode(&mut self, val: super::vals::Detectmode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Detectmode {
    #[inline(always)]
    fn default() -> Detectmode {
        Detectmode(0)
    }
}
#[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Latch(pub u32);
impl Latch {
    #[doc = "Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn set_pin(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Latch {
    #[inline(always)]
    fn default() -> Latch {
        Latch(0)
    }
}
#[doc = "Direction of GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dir(pub u32);
impl Dir {
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn pin(&self, n: usize) -> super::vals::Dir {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        super::vals::Dir(val as u8)
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn set_pin(&mut self, n: usize, val: super::vals::Dir) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.0 as u32) & 0x01) << offs);
    }
}
impl Default for Dir {
    #[inline(always)]
    fn default() -> Dir {
        Dir(0)
    }
}
#[doc = "DIR set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dirset(pub u32);
impl Dirset {
    #[doc = "Set as output pin 0"]
    #[inline(always)]
    pub fn pin(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Set as output pin 0"]
    #[inline(always)]
    pub fn set_pin(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Dirset {
    #[inline(always)]
    fn default() -> Dirset {
        Dirset(0)
    }
}
#[doc = "Set individual bits in GPIO port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outset(pub u32);
impl Outset {
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn pin(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn set_pin(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Outset {
    #[inline(always)]
    fn default() -> Outset {
        Outset(0)
    }
}
#[doc = "Write GPIO port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Out(pub u32);
impl Out {
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn pin(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn set_pin(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Out {
    #[inline(always)]
    fn default() -> Out {
        Out(0)
    }
}
#[doc = "Description collection\\[n\\]: Configuration of GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PinCnf(pub u32);
impl PinCnf {
    #[doc = "Pin direction. Same physical register as DIR register"]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::Dir {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dir(val as u8)
    }
    #[doc = "Pin direction. Same physical register as DIR register"]
    #[inline(always)]
    pub fn set_dir(&mut self, val: super::vals::Dir) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Connect or disconnect input buffer"]
    #[inline(always)]
    pub const fn input(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Connect or disconnect input buffer"]
    #[inline(always)]
    pub fn set_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull configuration"]
    #[inline(always)]
    pub const fn pull(&self) -> super::vals::Pull {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pull(val as u8)
    }
    #[doc = "Pull configuration"]
    #[inline(always)]
    pub fn set_pull(&mut self, val: super::vals::Pull) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.0 as u32) & 0x03) << 2usize);
    }
    #[doc = "Drive configuration"]
    #[inline(always)]
    pub const fn drive(&self) -> super::vals::Drive {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Drive(val as u8)
    }
    #[doc = "Drive configuration"]
    #[inline(always)]
    pub fn set_drive(&mut self, val: super::vals::Drive) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.0 as u32) & 0x07) << 8usize);
    }
    #[doc = "Pin sensing mechanism"]
    #[inline(always)]
    pub const fn sense(&self) -> super::vals::Sense {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Sense(val as u8)
    }
    #[doc = "Pin sensing mechanism"]
    #[inline(always)]
    pub fn set_sense(&mut self, val: super::vals::Sense) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.0 as u32) & 0x03) << 16usize);
    }
}
impl Default for PinCnf {
    #[inline(always)]
    fn default() -> PinCnf {
        PinCnf(0)
    }
}
#[doc = "Read GPIO port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct In(pub u32);
impl In {
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn pin(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn set_pin(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for In {
    #[inline(always)]
    fn default() -> In {
        In(0)
    }
}
#[doc = "DIR clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dirclr(pub u32);
impl Dirclr {
    #[doc = "Set as input pin 0"]
    #[inline(always)]
    pub fn pin(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Set as input pin 0"]
    #[inline(always)]
    pub fn set_pin(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Dirclr {
    #[inline(always)]
    fn default() -> Dirclr {
        Dirclr(0)
    }
}
#[doc = "Clear individual bits in GPIO port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outclr(pub u32);
impl Outclr {
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn pin(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn set_pin(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Outclr {
    #[inline(always)]
    fn default() -> Outclr {
        Outclr(0)
    }
}
