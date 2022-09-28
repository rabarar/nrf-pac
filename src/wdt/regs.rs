#[doc = "Run status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Runstatus(pub u32);
impl Runstatus {
    #[doc = "Indicates whether or not the watchdog is running"]
    #[inline(always)]
    pub const fn runstatus(&self) -> super::vals::Runstatus {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Runstatus(val as u8)
    }
    #[doc = "Indicates whether or not the watchdog is running"]
    #[inline(always)]
    pub fn set_runstatus(&mut self, val: super::vals::Runstatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Runstatus {
    #[inline(always)]
    fn default() -> Runstatus {
        Runstatus(0)
    }
}
#[doc = "Enable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to enable interrupt for TIMEOUT event"]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for TIMEOUT event"]
    #[inline(always)]
    pub fn set_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Enable register for reload request registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rren(pub u32);
impl Rren {
    #[doc = "Enable or disable RR\\[0\\] register"]
    #[inline(always)]
    pub const fn rr0(&self) -> super::vals::RrenRr0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RrenRr0(val as u8)
    }
    #[doc = "Enable or disable RR\\[0\\] register"]
    #[inline(always)]
    pub fn set_rr0(&mut self, val: super::vals::RrenRr0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable or disable RR\\[1\\] register"]
    #[inline(always)]
    pub const fn rr1(&self) -> super::vals::RrenRr1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RrenRr1(val as u8)
    }
    #[doc = "Enable or disable RR\\[1\\] register"]
    #[inline(always)]
    pub fn set_rr1(&mut self, val: super::vals::RrenRr1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable or disable RR\\[2\\] register"]
    #[inline(always)]
    pub const fn rr2(&self) -> super::vals::RrenRr2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RrenRr2(val as u8)
    }
    #[doc = "Enable or disable RR\\[2\\] register"]
    #[inline(always)]
    pub fn set_rr2(&mut self, val: super::vals::RrenRr2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable or disable RR\\[3\\] register"]
    #[inline(always)]
    pub const fn rr3(&self) -> super::vals::RrenRr3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::RrenRr3(val as u8)
    }
    #[doc = "Enable or disable RR\\[3\\] register"]
    #[inline(always)]
    pub fn set_rr3(&mut self, val: super::vals::RrenRr3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable or disable RR\\[4\\] register"]
    #[inline(always)]
    pub const fn rr4(&self) -> super::vals::RrenRr4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::RrenRr4(val as u8)
    }
    #[doc = "Enable or disable RR\\[4\\] register"]
    #[inline(always)]
    pub fn set_rr4(&mut self, val: super::vals::RrenRr4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable or disable RR\\[5\\] register"]
    #[inline(always)]
    pub const fn rr5(&self) -> super::vals::RrenRr5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::RrenRr5(val as u8)
    }
    #[doc = "Enable or disable RR\\[5\\] register"]
    #[inline(always)]
    pub fn set_rr5(&mut self, val: super::vals::RrenRr5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable or disable RR\\[6\\] register"]
    #[inline(always)]
    pub const fn rr6(&self) -> super::vals::RrenRr6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::RrenRr6(val as u8)
    }
    #[doc = "Enable or disable RR\\[6\\] register"]
    #[inline(always)]
    pub fn set_rr6(&mut self, val: super::vals::RrenRr6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.0 as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable or disable RR\\[7\\] register"]
    #[inline(always)]
    pub const fn rr7(&self) -> super::vals::RrenRr7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::RrenRr7(val as u8)
    }
    #[doc = "Enable or disable RR\\[7\\] register"]
    #[inline(always)]
    pub fn set_rr7(&mut self, val: super::vals::RrenRr7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
    }
}
impl Default for Rren {
    #[inline(always)]
    fn default() -> Rren {
        Rren(0)
    }
}
#[doc = "Description collection\\[n\\]: Reload request n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rr(pub u32);
impl Rr {
    #[doc = "Reload request register"]
    #[inline(always)]
    pub const fn rr(&self) -> super::vals::Rr {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Rr(val as u32)
    }
    #[doc = "Reload request register"]
    #[inline(always)]
    pub fn set_rr(&mut self, val: super::vals::Rr) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.0 as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rr {
    #[inline(always)]
    fn default() -> Rr {
        Rr(0)
    }
}
#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is sleeping"]
    #[inline(always)]
    pub const fn sleep(&self) -> super::vals::Sleep {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sleep(val as u8)
    }
    #[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is sleeping"]
    #[inline(always)]
    pub fn set_sleep(&mut self, val: super::vals::Sleep) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger"]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Halt {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Halt(val as u8)
    }
    #[doc = "Configure the watchdog to either be paused, or kept running, while the CPU is halted by the debugger"]
    #[inline(always)]
    pub fn set_halt(&mut self, val: super::vals::Halt) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
#[doc = "Request status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reqstatus(pub u32);
impl Reqstatus {
    #[doc = "Request status for RR\\[0\\] register"]
    #[inline(always)]
    pub const fn rr0(&self) -> super::vals::ReqstatusRr0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ReqstatusRr0(val as u8)
    }
    #[doc = "Request status for RR\\[0\\] register"]
    #[inline(always)]
    pub fn set_rr0(&mut self, val: super::vals::ReqstatusRr0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Request status for RR\\[1\\] register"]
    #[inline(always)]
    pub const fn rr1(&self) -> super::vals::ReqstatusRr1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ReqstatusRr1(val as u8)
    }
    #[doc = "Request status for RR\\[1\\] register"]
    #[inline(always)]
    pub fn set_rr1(&mut self, val: super::vals::ReqstatusRr1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
    }
    #[doc = "Request status for RR\\[2\\] register"]
    #[inline(always)]
    pub const fn rr2(&self) -> super::vals::ReqstatusRr2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ReqstatusRr2(val as u8)
    }
    #[doc = "Request status for RR\\[2\\] register"]
    #[inline(always)]
    pub fn set_rr2(&mut self, val: super::vals::ReqstatusRr2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
    }
    #[doc = "Request status for RR\\[3\\] register"]
    #[inline(always)]
    pub const fn rr3(&self) -> super::vals::ReqstatusRr3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ReqstatusRr3(val as u8)
    }
    #[doc = "Request status for RR\\[3\\] register"]
    #[inline(always)]
    pub fn set_rr3(&mut self, val: super::vals::ReqstatusRr3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
    }
    #[doc = "Request status for RR\\[4\\] register"]
    #[inline(always)]
    pub const fn rr4(&self) -> super::vals::ReqstatusRr4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::ReqstatusRr4(val as u8)
    }
    #[doc = "Request status for RR\\[4\\] register"]
    #[inline(always)]
    pub fn set_rr4(&mut self, val: super::vals::ReqstatusRr4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
    }
    #[doc = "Request status for RR\\[5\\] register"]
    #[inline(always)]
    pub const fn rr5(&self) -> super::vals::ReqstatusRr5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ReqstatusRr5(val as u8)
    }
    #[doc = "Request status for RR\\[5\\] register"]
    #[inline(always)]
    pub fn set_rr5(&mut self, val: super::vals::ReqstatusRr5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
    }
    #[doc = "Request status for RR\\[6\\] register"]
    #[inline(always)]
    pub const fn rr6(&self) -> super::vals::ReqstatusRr6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::ReqstatusRr6(val as u8)
    }
    #[doc = "Request status for RR\\[6\\] register"]
    #[inline(always)]
    pub fn set_rr6(&mut self, val: super::vals::ReqstatusRr6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.0 as u32) & 0x01) << 6usize);
    }
    #[doc = "Request status for RR\\[7\\] register"]
    #[inline(always)]
    pub const fn rr7(&self) -> super::vals::ReqstatusRr7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::ReqstatusRr7(val as u8)
    }
    #[doc = "Request status for RR\\[7\\] register"]
    #[inline(always)]
    pub fn set_rr7(&mut self, val: super::vals::ReqstatusRr7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
    }
}
impl Default for Reqstatus {
    #[inline(always)]
    fn default() -> Reqstatus {
        Reqstatus(0)
    }
}
