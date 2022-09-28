#[doc = "Enable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to enable interrupt for COMPARE\\[0\\] event"]
    #[inline(always)]
    pub const fn compare0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for COMPARE\\[0\\] event"]
    #[inline(always)]
    pub fn set_compare0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Write '1' to enable interrupt for COMPARE\\[1\\] event"]
    #[inline(always)]
    pub const fn compare1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for COMPARE\\[1\\] event"]
    #[inline(always)]
    pub fn set_compare1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Write '1' to enable interrupt for COMPARE\\[2\\] event"]
    #[inline(always)]
    pub const fn compare2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for COMPARE\\[2\\] event"]
    #[inline(always)]
    pub fn set_compare2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to enable interrupt for COMPARE\\[3\\] event"]
    #[inline(always)]
    pub const fn compare3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for COMPARE\\[3\\] event"]
    #[inline(always)]
    pub fn set_compare3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Write '1' to enable interrupt for COMPARE\\[4\\] event"]
    #[inline(always)]
    pub const fn compare4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for COMPARE\\[4\\] event"]
    #[inline(always)]
    pub fn set_compare4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Write '1' to enable interrupt for COMPARE\\[5\\] event"]
    #[inline(always)]
    pub const fn compare5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for COMPARE\\[5\\] event"]
    #[inline(always)]
    pub fn set_compare5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Timer mode selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "Timer mode"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Mode(val as u8)
    }
    #[doc = "Timer mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
#[doc = "Configure the number of bits used by the TIMER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bitmode(pub u32);
impl Bitmode {
    #[doc = "Timer bit width"]
    #[inline(always)]
    pub const fn bitmode(&self) -> super::vals::Bitmode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Bitmode(val as u8)
    }
    #[doc = "Timer bit width"]
    #[inline(always)]
    pub fn set_bitmode(&mut self, val: super::vals::Bitmode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
}
impl Default for Bitmode {
    #[inline(always)]
    fn default() -> Bitmode {
        Bitmode(0)
    }
}
#[doc = "Shortcut register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between COMPARE\\[0\\] event and CLEAR task"]
    #[inline(always)]
    pub const fn compare0_clear(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between COMPARE\\[0\\] event and CLEAR task"]
    #[inline(always)]
    pub fn set_compare0_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Shortcut between COMPARE\\[1\\] event and CLEAR task"]
    #[inline(always)]
    pub const fn compare1_clear(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between COMPARE\\[1\\] event and CLEAR task"]
    #[inline(always)]
    pub fn set_compare1_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Shortcut between COMPARE\\[2\\] event and CLEAR task"]
    #[inline(always)]
    pub const fn compare2_clear(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between COMPARE\\[2\\] event and CLEAR task"]
    #[inline(always)]
    pub fn set_compare2_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Shortcut between COMPARE\\[3\\] event and CLEAR task"]
    #[inline(always)]
    pub const fn compare3_clear(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between COMPARE\\[3\\] event and CLEAR task"]
    #[inline(always)]
    pub fn set_compare3_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Shortcut between COMPARE\\[4\\] event and CLEAR task"]
    #[inline(always)]
    pub const fn compare4_clear(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between COMPARE\\[4\\] event and CLEAR task"]
    #[inline(always)]
    pub fn set_compare4_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Shortcut between COMPARE\\[5\\] event and CLEAR task"]
    #[inline(always)]
    pub const fn compare5_clear(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between COMPARE\\[5\\] event and CLEAR task"]
    #[inline(always)]
    pub fn set_compare5_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Shortcut between COMPARE\\[0\\] event and STOP task"]
    #[inline(always)]
    pub const fn compare0_stop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between COMPARE\\[0\\] event and STOP task"]
    #[inline(always)]
    pub fn set_compare0_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Shortcut between COMPARE\\[1\\] event and STOP task"]
    #[inline(always)]
    pub const fn compare1_stop(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between COMPARE\\[1\\] event and STOP task"]
    #[inline(always)]
    pub fn set_compare1_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Shortcut between COMPARE\\[2\\] event and STOP task"]
    #[inline(always)]
    pub const fn compare2_stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between COMPARE\\[2\\] event and STOP task"]
    #[inline(always)]
    pub fn set_compare2_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Shortcut between COMPARE\\[3\\] event and STOP task"]
    #[inline(always)]
    pub const fn compare3_stop(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between COMPARE\\[3\\] event and STOP task"]
    #[inline(always)]
    pub fn set_compare3_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Shortcut between COMPARE\\[4\\] event and STOP task"]
    #[inline(always)]
    pub const fn compare4_stop(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between COMPARE\\[4\\] event and STOP task"]
    #[inline(always)]
    pub fn set_compare4_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Shortcut between COMPARE\\[5\\] event and STOP task"]
    #[inline(always)]
    pub const fn compare5_stop(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between COMPARE\\[5\\] event and STOP task"]
    #[inline(always)]
    pub fn set_compare5_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "Timer prescaler register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prescaler(pub u32);
impl Prescaler {
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub const fn prescaler(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub fn set_prescaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Prescaler {
    #[inline(always)]
    fn default() -> Prescaler {
        Prescaler(0)
    }
}
