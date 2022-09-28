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
#[doc = "DCX configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcxcnt(pub u32);
impl Dcxcnt {
    #[doc = "This register specifies the number of command bytes preceding the data bytes. The PSEL.DCX line will be low during transmission of command bytes and high during transmission of data bytes. Value 0xF indicates that all bytes are command bytes."]
    #[inline(always)]
    pub const fn dcxcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "This register specifies the number of command bytes preceding the data bytes. The PSEL.DCX line will be low during transmission of command bytes and high during transmission of data bytes. Value 0xF indicates that all bytes are command bytes."]
    #[inline(always)]
    pub fn set_dcxcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Dcxcnt {
    #[inline(always)]
    fn default() -> Dcxcnt {
        Dcxcnt(0)
    }
}
#[doc = "Stall status for EasyDMA RAM accesses. The fields in this register is set to STALL by hardware whenever a stall occurres and can be cleared (set to NOSTALL) by the CPU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stallstat(pub u32);
impl Stallstat {
    #[doc = "Stall status for EasyDMA RAM reads"]
    #[inline(always)]
    pub const fn tx(&self) -> super::vals::Tx {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tx(val as u8)
    }
    #[doc = "Stall status for EasyDMA RAM reads"]
    #[inline(always)]
    pub fn set_tx(&mut self, val: super::vals::Tx) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Stall status for EasyDMA RAM writes"]
    #[inline(always)]
    pub const fn rx(&self) -> super::vals::Rx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rx(val as u8)
    }
    #[doc = "Stall status for EasyDMA RAM writes"]
    #[inline(always)]
    pub fn set_rx(&mut self, val: super::vals::Rx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
    }
}
impl Default for Stallstat {
    #[inline(always)]
    fn default() -> Stallstat {
        Stallstat(0)
    }
}
#[doc = "Polarity of CSN output"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csnpol(pub u32);
impl Csnpol {
    #[doc = "Polarity of CSN output"]
    #[inline(always)]
    pub const fn csnpol(&self) -> super::vals::Csnpol {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Csnpol(val as u8)
    }
    #[doc = "Polarity of CSN output"]
    #[inline(always)]
    pub fn set_csnpol(&mut self, val: super::vals::Csnpol) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Csnpol {
    #[inline(always)]
    fn default() -> Csnpol {
        Csnpol(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for STOPPED event"]
    #[inline(always)]
    pub const fn stopped(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn set_stopped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable interrupt for ENDRX event"]
    #[inline(always)]
    pub const fn endrx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for ENDRX event"]
    #[inline(always)]
    pub fn set_endrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to disable interrupt for END event"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for END event"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write '1' to disable interrupt for ENDTX event"]
    #[inline(always)]
    pub const fn endtx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for ENDTX event"]
    #[inline(always)]
    pub fn set_endtx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write '1' to disable interrupt for STARTED event"]
    #[inline(always)]
    pub const fn started(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for STARTED event"]
    #[inline(always)]
    pub fn set_started(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Number of bytes in transmit buffer"]
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
#[doc = "EasyDMA list type"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxdList(pub u32);
impl TxdList {
    #[doc = "List type"]
    #[inline(always)]
    pub const fn list(&self) -> super::vals::TxdListList {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::TxdListList(val as u8)
    }
    #[doc = "List type"]
    #[inline(always)]
    pub fn set_list(&mut self, val: super::vals::TxdListList) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
}
impl Default for TxdList {
    #[inline(always)]
    fn default() -> TxdList {
        TxdList(0)
    }
}
#[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Orc(pub u32);
impl Orc {
    #[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT."]
    #[inline(always)]
    pub const fn orc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT."]
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
#[doc = "EasyDMA list type"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxdList(pub u32);
impl RxdList {
    #[doc = "List type"]
    #[inline(always)]
    pub const fn list(&self) -> super::vals::RxdListList {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RxdListList(val as u8)
    }
    #[doc = "List type"]
    #[inline(always)]
    pub fn set_list(&mut self, val: super::vals::RxdListList) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
}
impl Default for RxdList {
    #[inline(always)]
    fn default() -> RxdList {
        RxdList(0)
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
#[doc = "Shortcut register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between END event and START task"]
    #[inline(always)]
    pub const fn end_start(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between END event and START task"]
    #[inline(always)]
    pub fn set_end_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
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
#[doc = "Pin select for CSN"]
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
#[doc = "Enable SPIM"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable SPIM"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Enable(val as u8)
    }
    #[doc = "Enable or disable SPIM"]
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
#[doc = "Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csndur(pub u32);
impl Csndur {
    #[doc = "Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions. The value is specified in number of 64 MHz clock cycles (15.625 ns)."]
    #[inline(always)]
    pub const fn csndur(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions. The value is specified in number of 64 MHz clock cycles (15.625 ns)."]
    #[inline(always)]
    pub fn set_csndur(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Csndur {
    #[inline(always)]
    fn default() -> Csndur {
        Csndur(0)
    }
}
#[doc = "Number of bytes transferred in the last transaction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxdAmount(pub u32);
impl RxdAmount {
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of bytes transferred in the last transaction"]
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
#[doc = "Sample delay for input serial data on MISO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxdelay(pub u32);
impl Rxdelay {
    #[doc = "Sample delay for input serial data on MISO. The value specifies the number of 64 MHz clock cycles (15.625 ns) delay from the the sampling edge of SCK (leading edge for CONFIG.CPHA = 0, trailing edge for CONFIG.CPHA = 1) until the input serial data is sampled. As en example, if RXDELAY = 0 and CONFIG.CPHA = 0, the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub const fn rxdelay(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Sample delay for input serial data on MISO. The value specifies the number of 64 MHz clock cycles (15.625 ns) delay from the the sampling edge of SCK (leading edge for CONFIG.CPHA = 0, trailing edge for CONFIG.CPHA = 1) until the input serial data is sampled. As en example, if RXDELAY = 0 and CONFIG.CPHA = 0, the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn set_rxdelay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Rxdelay {
    #[inline(always)]
    fn default() -> Rxdelay {
        Rxdelay(0)
    }
}
#[doc = "Number of bytes transferred in the last transaction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxdAmount(pub u32);
impl TxdAmount {
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of bytes transferred in the last transaction"]
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
#[doc = "Pin select for DCX signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pseldcx(pub u32);
impl Pseldcx {
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
    pub const fn connect(&self) -> super::vals::PseldcxConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PseldcxConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::PseldcxConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Pseldcx {
    #[inline(always)]
    fn default() -> Pseldcx {
        Pseldcx(0)
    }
}
