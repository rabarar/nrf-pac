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
#[doc = "Enable SPI"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable SPI"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Enable(val as u8)
    }
    #[doc = "Enable or disable SPI"]
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
#[doc = "RXD register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxd(pub u32);
impl Rxd {
    #[doc = "RX data received. Double buffered"]
    #[inline(always)]
    pub const fn rxd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX data received. Double buffered"]
    #[inline(always)]
    pub fn set_rxd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rxd {
    #[inline(always)]
    fn default() -> Rxd {
        Rxd(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for READY event"]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for READY event"]
    #[inline(always)]
    pub fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frequency(pub u32);
impl Frequency {
    #[doc = "SPI master data rate"]
    #[inline(always)]
    pub const fn frequency(&self) -> super::vals::Frequency {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Frequency(val as u32)
    }
    #[doc = "SPI master data rate"]
    #[inline(always)]
    pub fn set_frequency(&mut self, val: super::vals::Frequency) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.0 as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Frequency {
    #[inline(always)]
    fn default() -> Frequency {
        Frequency(0)
    }
}
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
#[doc = "TXD register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txd(pub u32);
impl Txd {
    #[doc = "TX data to send. Double buffered"]
    #[inline(always)]
    pub const fn txd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TX data to send. Double buffered"]
    #[inline(always)]
    pub fn set_txd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txd {
    #[inline(always)]
    fn default() -> Txd {
        Txd(0)
    }
}
