#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for IN\\[0\\] event"]
    #[inline(always)]
    pub const fn in0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for IN\\[0\\] event"]
    #[inline(always)]
    pub fn set_in0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable interrupt for IN\\[1\\] event"]
    #[inline(always)]
    pub const fn in1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for IN\\[1\\] event"]
    #[inline(always)]
    pub fn set_in1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable interrupt for IN\\[2\\] event"]
    #[inline(always)]
    pub const fn in2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for IN\\[2\\] event"]
    #[inline(always)]
    pub fn set_in2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to disable interrupt for IN\\[3\\] event"]
    #[inline(always)]
    pub const fn in3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for IN\\[3\\] event"]
    #[inline(always)]
    pub fn set_in3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to disable interrupt for IN\\[4\\] event"]
    #[inline(always)]
    pub const fn in4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for IN\\[4\\] event"]
    #[inline(always)]
    pub fn set_in4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to disable interrupt for IN\\[5\\] event"]
    #[inline(always)]
    pub const fn in5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for IN\\[5\\] event"]
    #[inline(always)]
    pub fn set_in5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write '1' to disable interrupt for IN\\[6\\] event"]
    #[inline(always)]
    pub const fn in6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for IN\\[6\\] event"]
    #[inline(always)]
    pub fn set_in6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write '1' to disable interrupt for IN\\[7\\] event"]
    #[inline(always)]
    pub const fn in7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for IN\\[7\\] event"]
    #[inline(always)]
    pub fn set_in7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write '1' to disable interrupt for PORT event"]
    #[inline(always)]
    pub const fn port(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for PORT event"]
    #[inline(always)]
    pub fn set_port(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Description collection\\[n\\]: Configuration for OUT\\[n\\], SET\\[n\\] and CLR\\[n\\] tasks and IN\\[n\\] event"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Mode"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Mode(val as u8)
    }
    #[doc = "Mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO number associated with SET\\[n\\], CLR\\[n\\] and OUT\\[n\\] tasks and IN\\[n\\] event"]
    #[inline(always)]
    pub const fn psel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "GPIO number associated with SET\\[n\\], CLR\\[n\\] and OUT\\[n\\] tasks and IN\\[n\\] event"]
    #[inline(always)]
    pub fn set_psel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub const fn port(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub fn set_port(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "When In task mode: Operation to be performed on output when OUT\\[n\\] task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\] event."]
    #[inline(always)]
    pub const fn polarity(&self) -> super::vals::Polarity {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Polarity(val as u8)
    }
    #[doc = "When In task mode: Operation to be performed on output when OUT\\[n\\] task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\] event."]
    #[inline(always)]
    pub fn set_polarity(&mut self, val: super::vals::Polarity) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.0 as u32) & 0x03) << 16usize);
    }
    #[doc = "When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
    #[inline(always)]
    pub const fn outinit(&self) -> super::vals::Outinit {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Outinit(val as u8)
    }
    #[doc = "When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
    #[inline(always)]
    pub fn set_outinit(&mut self, val: super::vals::Outinit) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.0 as u32) & 0x01) << 20usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
