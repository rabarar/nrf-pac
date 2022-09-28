#[doc = "Configuration of parity and hardware flow control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Hardware flow control"]
    #[inline(always)]
    pub const fn hwfc(&self) -> super::vals::Hwfc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hwfc(val as u8)
    }
    #[doc = "Hardware flow control"]
    #[inline(always)]
    pub fn set_hwfc(&mut self, val: super::vals::Hwfc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Parity"]
    #[inline(always)]
    pub const fn parity(&self) -> super::vals::ConfigParity {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::ConfigParity(val as u8)
    }
    #[doc = "Parity"]
    #[inline(always)]
    pub fn set_parity(&mut self, val: super::vals::ConfigParity) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.0 as u32) & 0x07) << 1usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
#[doc = "Pin select for RTS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rts(pub u32);
impl Rts {
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
    pub const fn connect(&self) -> super::vals::RtsConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::RtsConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::RtsConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Rts {
    #[inline(always)]
    fn default() -> Rts {
        Rts(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for CTS event"]
    #[inline(always)]
    pub const fn cts(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for CTS event"]
    #[inline(always)]
    pub fn set_cts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable interrupt for NCTS event"]
    #[inline(always)]
    pub const fn ncts(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for NCTS event"]
    #[inline(always)]
    pub fn set_ncts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable interrupt for RXDRDY event"]
    #[inline(always)]
    pub const fn rxdrdy(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for RXDRDY event"]
    #[inline(always)]
    pub fn set_rxdrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to disable interrupt for TXDRDY event"]
    #[inline(always)]
    pub const fn txdrdy(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for TXDRDY event"]
    #[inline(always)]
    pub fn set_txdrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write '1' to disable interrupt for ERROR event"]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for ERROR event"]
    #[inline(always)]
    pub fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Write '1' to disable interrupt for RXTO event"]
    #[inline(always)]
    pub const fn rxto(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for RXTO event"]
    #[inline(always)]
    pub fn set_rxto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Error source"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errorsrc(pub u32);
impl Errorsrc {
    #[doc = "Overrun error"]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error"]
    #[inline(always)]
    pub fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Parity error"]
    #[inline(always)]
    pub const fn parity(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error"]
    #[inline(always)]
    pub fn set_parity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Framing error occurred"]
    #[inline(always)]
    pub const fn framing(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error occurred"]
    #[inline(always)]
    pub fn set_framing(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Break condition"]
    #[inline(always)]
    pub const fn break_(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Break condition"]
    #[inline(always)]
    pub fn set_break_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Errorsrc {
    #[inline(always)]
    fn default() -> Errorsrc {
        Errorsrc(0)
    }
}
#[doc = "Enable UART"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable UART"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Enable(val as u8)
    }
    #[doc = "Enable or disable UART"]
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
#[doc = "Shortcut register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between CTS event and STARTRX task"]
    #[inline(always)]
    pub const fn cts_startrx(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between CTS event and STARTRX task"]
    #[inline(always)]
    pub fn set_cts_startrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Shortcut between NCTS event and STOPRX task"]
    #[inline(always)]
    pub const fn ncts_stoprx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between NCTS event and STOPRX task"]
    #[inline(always)]
    pub fn set_ncts_stoprx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Baudrate(pub u32);
impl Baudrate {
    #[doc = "Baud rate"]
    #[inline(always)]
    pub const fn baudrate(&self) -> super::vals::Baudrate {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Baudrate(val as u32)
    }
    #[doc = "Baud rate"]
    #[inline(always)]
    pub fn set_baudrate(&mut self, val: super::vals::Baudrate) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.0 as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Baudrate {
    #[inline(always)]
    fn default() -> Baudrate {
        Baudrate(0)
    }
}
#[doc = "RXD register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart0rxd(pub u32);
impl Uart0rxd {
    #[doc = "RX data received in previous transfers, double buffered"]
    #[inline(always)]
    pub const fn rxd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX data received in previous transfers, double buffered"]
    #[inline(always)]
    pub fn set_rxd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Uart0rxd {
    #[inline(always)]
    fn default() -> Uart0rxd {
        Uart0rxd(0)
    }
}
#[doc = "Pin select for CTS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cts(pub u32);
impl Cts {
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
    pub const fn connect(&self) -> super::vals::CtsConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CtsConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::CtsConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Cts {
    #[inline(always)]
    fn default() -> Cts {
        Cts(0)
    }
}
#[doc = "Pin select for RXD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PselRxd(pub u32);
impl PselRxd {
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
    pub const fn connect(&self) -> super::vals::RxdConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::RxdConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::RxdConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for PselRxd {
    #[inline(always)]
    fn default() -> PselRxd {
        PselRxd(0)
    }
}
#[doc = "Pin select for TXD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PselTxd(pub u32);
impl PselTxd {
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
    pub const fn connect(&self) -> super::vals::TxdConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::TxdConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::TxdConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for PselTxd {
    #[inline(always)]
    fn default() -> PselTxd {
        PselTxd(0)
    }
}
#[doc = "TXD register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart0txd(pub u32);
impl Uart0txd {
    #[doc = "TX data to be transferred"]
    #[inline(always)]
    pub const fn txd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TX data to be transferred"]
    #[inline(always)]
    pub fn set_txd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Uart0txd {
    #[inline(always)]
    fn default() -> Uart0txd {
        Uart0txd(0)
    }
}
