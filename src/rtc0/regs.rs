#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for TICK event"]
    #[inline(always)]
    pub const fn tick(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for TICK event"]
    #[inline(always)]
    pub fn set_tick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable interrupt for OVRFLW event"]
    #[inline(always)]
    pub const fn ovrflw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for OVRFLW event"]
    #[inline(always)]
    pub fn set_ovrflw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable interrupt for COMPARE\\[0\\] event"]
    #[inline(always)]
    pub const fn compare0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for COMPARE\\[0\\] event"]
    #[inline(always)]
    pub fn set_compare0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Write '1' to disable interrupt for COMPARE\\[1\\] event"]
    #[inline(always)]
    pub const fn compare1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for COMPARE\\[1\\] event"]
    #[inline(always)]
    pub fn set_compare1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Write '1' to disable interrupt for COMPARE\\[2\\] event"]
    #[inline(always)]
    pub const fn compare2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for COMPARE\\[2\\] event"]
    #[inline(always)]
    pub fn set_compare2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to disable interrupt for COMPARE\\[3\\] event"]
    #[inline(always)]
    pub const fn compare3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for COMPARE\\[3\\] event"]
    #[inline(always)]
    pub fn set_compare3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Disable event routing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evtenclr(pub u32);
impl Evtenclr {
    #[doc = "Write '1' to disable event routing for TICK event"]
    #[inline(always)]
    pub const fn tick(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable event routing for TICK event"]
    #[inline(always)]
    pub fn set_tick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable event routing for OVRFLW event"]
    #[inline(always)]
    pub const fn ovrflw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable event routing for OVRFLW event"]
    #[inline(always)]
    pub fn set_ovrflw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable event routing for COMPARE\\[0\\] event"]
    #[inline(always)]
    pub const fn compare0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable event routing for COMPARE\\[0\\] event"]
    #[inline(always)]
    pub fn set_compare0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Write '1' to disable event routing for COMPARE\\[1\\] event"]
    #[inline(always)]
    pub const fn compare1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable event routing for COMPARE\\[1\\] event"]
    #[inline(always)]
    pub fn set_compare1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Write '1' to disable event routing for COMPARE\\[2\\] event"]
    #[inline(always)]
    pub const fn compare2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable event routing for COMPARE\\[2\\] event"]
    #[inline(always)]
    pub fn set_compare2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to disable event routing for COMPARE\\[3\\] event"]
    #[inline(always)]
    pub const fn compare3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable event routing for COMPARE\\[3\\] event"]
    #[inline(always)]
    pub fn set_compare3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Evtenclr {
    #[inline(always)]
    fn default() -> Evtenclr {
        Evtenclr(0)
    }
}
#[doc = "Description collection\\[n\\]: Compare register n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cc(pub u32);
impl Cc {
    #[doc = "Compare value"]
    #[inline(always)]
    pub const fn compare(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Compare value"]
    #[inline(always)]
    pub fn set_compare(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Cc {
    #[inline(always)]
    fn default() -> Cc {
        Cc(0)
    }
}
#[doc = "Current COUNTER value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Counter(pub u32);
impl Counter {
    #[doc = "Counter value"]
    #[inline(always)]
    pub const fn counter(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Counter value"]
    #[inline(always)]
    pub fn set_counter(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Counter {
    #[inline(always)]
    fn default() -> Counter {
        Counter(0)
    }
}
#[doc = "Enable event routing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evtenset(pub u32);
impl Evtenset {
    #[doc = "Write '1' to enable event routing for TICK event"]
    #[inline(always)]
    pub const fn tick(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable event routing for TICK event"]
    #[inline(always)]
    pub fn set_tick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to enable event routing for OVRFLW event"]
    #[inline(always)]
    pub const fn ovrflw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable event routing for OVRFLW event"]
    #[inline(always)]
    pub fn set_ovrflw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to enable event routing for COMPARE\\[0\\] event"]
    #[inline(always)]
    pub const fn compare0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable event routing for COMPARE\\[0\\] event"]
    #[inline(always)]
    pub fn set_compare0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Write '1' to enable event routing for COMPARE\\[1\\] event"]
    #[inline(always)]
    pub const fn compare1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable event routing for COMPARE\\[1\\] event"]
    #[inline(always)]
    pub fn set_compare1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Write '1' to enable event routing for COMPARE\\[2\\] event"]
    #[inline(always)]
    pub const fn compare2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable event routing for COMPARE\\[2\\] event"]
    #[inline(always)]
    pub fn set_compare2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to enable event routing for COMPARE\\[3\\] event"]
    #[inline(always)]
    pub const fn compare3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable event routing for COMPARE\\[3\\] event"]
    #[inline(always)]
    pub fn set_compare3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Evtenset {
    #[inline(always)]
    fn default() -> Evtenset {
        Evtenset(0)
    }
}
#[doc = "Enable or disable event routing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evten(pub u32);
impl Evten {
    #[doc = "Enable or disable event routing for TICK event"]
    #[inline(always)]
    pub const fn tick(&self) -> super::vals::Tick {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tick(val as u8)
    }
    #[doc = "Enable or disable event routing for TICK event"]
    #[inline(always)]
    pub fn set_tick(&mut self, val: super::vals::Tick) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable or disable event routing for OVRFLW event"]
    #[inline(always)]
    pub const fn ovrflw(&self) -> super::vals::Ovrflw {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ovrflw(val as u8)
    }
    #[doc = "Enable or disable event routing for OVRFLW event"]
    #[inline(always)]
    pub fn set_ovrflw(&mut self, val: super::vals::Ovrflw) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable or disable event routing for COMPARE\\[0\\] event"]
    #[inline(always)]
    pub const fn compare0(&self) -> super::vals::Compare0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Compare0(val as u8)
    }
    #[doc = "Enable or disable event routing for COMPARE\\[0\\] event"]
    #[inline(always)]
    pub fn set_compare0(&mut self, val: super::vals::Compare0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
    }
    #[doc = "Enable or disable event routing for COMPARE\\[1\\] event"]
    #[inline(always)]
    pub const fn compare1(&self) -> super::vals::Compare1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Compare1(val as u8)
    }
    #[doc = "Enable or disable event routing for COMPARE\\[1\\] event"]
    #[inline(always)]
    pub fn set_compare1(&mut self, val: super::vals::Compare1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.0 as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable or disable event routing for COMPARE\\[2\\] event"]
    #[inline(always)]
    pub const fn compare2(&self) -> super::vals::Compare2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Compare2(val as u8)
    }
    #[doc = "Enable or disable event routing for COMPARE\\[2\\] event"]
    #[inline(always)]
    pub fn set_compare2(&mut self, val: super::vals::Compare2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.0 as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable or disable event routing for COMPARE\\[3\\] event"]
    #[inline(always)]
    pub const fn compare3(&self) -> super::vals::Compare3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Compare3(val as u8)
    }
    #[doc = "Enable or disable event routing for COMPARE\\[3\\] event"]
    #[inline(always)]
    pub fn set_compare3(&mut self, val: super::vals::Compare3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.0 as u32) & 0x01) << 19usize);
    }
}
impl Default for Evten {
    #[inline(always)]
    fn default() -> Evten {
        Evten(0)
    }
}
#[doc = "12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prescaler(pub u32);
impl Prescaler {
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub const fn prescaler(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub fn set_prescaler(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Prescaler {
    #[inline(always)]
    fn default() -> Prescaler {
        Prescaler(0)
    }
}
