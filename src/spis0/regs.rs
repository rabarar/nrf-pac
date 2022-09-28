#[doc = "Pin select for MISO signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Miso(pub u32);
impl Miso {
    #[doc = "Pin number"]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Pin number"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub const fn port(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub fn set_port(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::MisoConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MisoConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::MisoConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Miso {
    #[inline(always)]
    fn default() -> Miso {
        Miso(0)
    }
}
#[doc = "Pin select for CSN signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csn(pub u32);
impl Csn {
    #[doc = "Pin number"]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Pin number"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub const fn port(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub fn set_port(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::CsnConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CsnConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::CsnConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Csn {
    #[inline(always)]
    fn default() -> Csn {
        Csn(0)
    }
}
#[doc = "Pin select for MOSI signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mosi(pub u32);
impl Mosi {
    #[doc = "Pin number"]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Pin number"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub const fn port(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub fn set_port(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::MosiConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MosiConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::MosiConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Mosi {
    #[inline(always)]
    fn default() -> Mosi {
        Mosi(0)
    }
}
#[doc = "Enable SPI slave"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable SPI slave"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Enable(val as u8)
    }
    #[doc = "Enable or disable SPI slave"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.0 as u32) & 0x0f) << 0usize);
    }
}
impl Default for Enable {
    #[inline(always)]
    fn default() -> Enable {
        Enable(0)
    }
}
#[doc = "Maximum number of bytes in receive buffer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxdMaxcnt(pub u32);
impl RxdMaxcnt {
    #[doc = "Maximum number of bytes in receive buffer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Maximum number of bytes in receive buffer"]
    #[inline(always)]
    pub fn set_maxcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RxdMaxcnt {
    #[inline(always)]
    fn default() -> RxdMaxcnt {
        RxdMaxcnt(0)
    }
}
#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Bit order"]
    #[inline(always)]
    pub const fn order(&self) -> super::vals::Order {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Order(val as u8)
    }
    #[doc = "Bit order"]
    #[inline(always)]
    pub fn set_order(&mut self, val: super::vals::Order) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Serial clock (SCK) phase"]
    #[inline(always)]
    pub const fn cpha(&self) -> super::vals::Cpha {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cpha(val as u8)
    }
    #[doc = "Serial clock (SCK) phase"]
    #[inline(always)]
    pub fn set_cpha(&mut self, val: super::vals::Cpha) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
    }
    #[doc = "Serial clock (SCK) polarity"]
    #[inline(always)]
    pub const fn cpol(&self) -> super::vals::Cpol {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Cpol(val as u8)
    }
    #[doc = "Serial clock (SCK) polarity"]
    #[inline(always)]
    pub fn set_cpol(&mut self, val: super::vals::Cpol) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
#[doc = "Shortcut register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between END event and ACQUIRE task"]
    #[inline(always)]
    pub const fn end_acquire(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between END event and ACQUIRE task"]
    #[inline(always)]
    pub fn set_end_acquire(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "Over-read character"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Orc(pub u32);
impl Orc {
    #[doc = "Over-read character. Character clocked out after an over-read of the transmit buffer."]
    #[inline(always)]
    pub const fn orc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Over-read character. Character clocked out after an over-read of the transmit buffer."]
    #[inline(always)]
    pub fn set_orc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Orc {
    #[inline(always)]
    fn default() -> Orc {
        Orc(0)
    }
}
#[doc = "Enable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to enable interrupt for END event"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for END event"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to enable interrupt for ENDRX event"]
    #[inline(always)]
    pub const fn endrx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for ENDRX event"]
    #[inline(always)]
    pub fn set_endrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to enable interrupt for ACQUIRED event"]
    #[inline(always)]
    pub const fn acquired(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for ACQUIRED event"]
    #[inline(always)]
    pub fn set_acquired(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Status from last transaction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub const fn overread(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn set_overread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "Number of bytes received in last granted transaction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxdAmount(pub u32);
impl RxdAmount {
    #[doc = "Number of bytes received in the last granted transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of bytes received in the last granted transaction"]
    #[inline(always)]
    pub fn set_amount(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RxdAmount {
    #[inline(always)]
    fn default() -> RxdAmount {
        RxdAmount(0)
    }
}
#[doc = "Semaphore status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Semstat(pub u32);
impl Semstat {
    #[doc = "Semaphore status"]
    #[inline(always)]
    pub const fn semstat(&self) -> super::vals::Semstat {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Semstat(val as u8)
    }
    #[doc = "Semaphore status"]
    #[inline(always)]
    pub fn set_semstat(&mut self, val: super::vals::Semstat) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
}
impl Default for Semstat {
    #[inline(always)]
    fn default() -> Semstat {
        Semstat(0)
    }
}
#[doc = "Default character. Character clocked out in case of an ignored transaction."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Def(pub u32);
impl Def {
    #[doc = "Default character. Character clocked out in case of an ignored transaction."]
    #[inline(always)]
    pub const fn def(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Default character. Character clocked out in case of an ignored transaction."]
    #[inline(always)]
    pub fn set_def(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Def {
    #[inline(always)]
    fn default() -> Def {
        Def(0)
    }
}
#[doc = "Pin select for SCK"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sck(pub u32);
impl Sck {
    #[doc = "Pin number"]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Pin number"]
    #[inline(always)]
    pub fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub const fn port(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Port number"]
    #[inline(always)]
    pub fn set_port(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub const fn connect(&self) -> super::vals::SckConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SckConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::SckConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Sck {
    #[inline(always)]
    fn default() -> Sck {
        Sck(0)
    }
}
#[doc = "Maximum number of bytes in transmit buffer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxdMaxcnt(pub u32);
impl TxdMaxcnt {
    #[doc = "Maximum number of bytes in transmit buffer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Maximum number of bytes in transmit buffer"]
    #[inline(always)]
    pub fn set_maxcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for TxdMaxcnt {
    #[inline(always)]
    fn default() -> TxdMaxcnt {
        TxdMaxcnt(0)
    }
}
#[doc = "Number of bytes transmitted in last granted transaction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxdAmount(pub u32);
impl TxdAmount {
    #[doc = "Number of bytes transmitted in last granted transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of bytes transmitted in last granted transaction"]
    #[inline(always)]
    pub fn set_amount(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for TxdAmount {
    #[inline(always)]
    fn default() -> TxdAmount {
        TxdAmount(0)
    }
}
