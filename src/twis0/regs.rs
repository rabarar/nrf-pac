#[doc = "Enable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to enable interrupt for STOPPED event"]
    #[inline(always)]
    pub const fn stopped(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn set_stopped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to enable interrupt for ERROR event"]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for ERROR event"]
    #[inline(always)]
    pub fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Write '1' to enable interrupt for RXSTARTED event"]
    #[inline(always)]
    pub const fn rxstarted(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for RXSTARTED event"]
    #[inline(always)]
    pub fn set_rxstarted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Write '1' to enable interrupt for TXSTARTED event"]
    #[inline(always)]
    pub const fn txstarted(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for TXSTARTED event"]
    #[inline(always)]
    pub fn set_txstarted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Write '1' to enable interrupt for WRITE event"]
    #[inline(always)]
    pub const fn write(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for WRITE event"]
    #[inline(always)]
    pub fn set_write(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Write '1' to enable interrupt for READ event"]
    #[inline(always)]
    pub const fn read(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for READ event"]
    #[inline(always)]
    pub fn set_read(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Maximum number of bytes in TXD buffer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxdMaxcnt(pub u32);
impl TxdMaxcnt {
    #[doc = "Maximum number of bytes in TXD buffer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Maximum number of bytes in TXD buffer"]
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
#[doc = "Enable TWIS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable TWIS"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Enable(val as u8)
    }
    #[doc = "Enable or disable TWIS"]
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
#[doc = "Status register indicating which address had a match"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match(pub u32);
impl Match {
    #[doc = "Which of the addresses in {ADDRESS} matched the incoming address"]
    #[inline(always)]
    pub const fn match_(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Which of the addresses in {ADDRESS} matched the incoming address"]
    #[inline(always)]
    pub fn set_match_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Match {
    #[inline(always)]
    fn default() -> Match {
        Match(0)
    }
}
#[doc = "Pin select for SCL signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scl(pub u32);
impl Scl {
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
    pub const fn connect(&self) -> super::vals::SclConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SclConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::SclConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Scl {
    #[inline(always)]
    fn default() -> Scl {
        Scl(0)
    }
}
#[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Orc(pub u32);
impl Orc {
    #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
    #[inline(always)]
    pub const fn orc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
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
#[doc = "Configuration register for the address match mechanism"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Enable or disable address matching on ADDRESS\\[0\\]"]
    #[inline(always)]
    pub const fn address0(&self) -> super::vals::Address0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Address0(val as u8)
    }
    #[doc = "Enable or disable address matching on ADDRESS\\[0\\]"]
    #[inline(always)]
    pub fn set_address0(&mut self, val: super::vals::Address0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable or disable address matching on ADDRESS\\[1\\]"]
    #[inline(always)]
    pub const fn address1(&self) -> super::vals::Address1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Address1(val as u8)
    }
    #[doc = "Enable or disable address matching on ADDRESS\\[1\\]"]
    #[inline(always)]
    pub fn set_address1(&mut self, val: super::vals::Address1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
#[doc = "Error source"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errorsrc(pub u32);
impl Errorsrc {
    #[doc = "RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub const fn overflow(&self) -> super::vals::Overflow {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Overflow(val as u8)
    }
    #[doc = "RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn set_overflow(&mut self, val: super::vals::Overflow) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "NACK sent after receiving a data byte"]
    #[inline(always)]
    pub const fn dnack(&self) -> super::vals::Dnack {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dnack(val as u8)
    }
    #[doc = "NACK sent after receiving a data byte"]
    #[inline(always)]
    pub fn set_dnack(&mut self, val: super::vals::Dnack) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
    }
    #[doc = "TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub const fn overread(&self) -> super::vals::Overread {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Overread(val as u8)
    }
    #[doc = "TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn set_overread(&mut self, val: super::vals::Overread) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
    }
}
impl Default for Errorsrc {
    #[inline(always)]
    fn default() -> Errorsrc {
        Errorsrc(0)
    }
}
#[doc = "Number of bytes transferred in the last RXD transaction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxdAmount(pub u32);
impl RxdAmount {
    #[doc = "Number of bytes transferred in the last RXD transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of bytes transferred in the last RXD transaction"]
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
#[doc = "Shortcut register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between WRITE event and SUSPEND task"]
    #[inline(always)]
    pub const fn write_suspend(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between WRITE event and SUSPEND task"]
    #[inline(always)]
    pub fn set_write_suspend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Shortcut between READ event and SUSPEND task"]
    #[inline(always)]
    pub const fn read_suspend(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between READ event and SUSPEND task"]
    #[inline(always)]
    pub fn set_read_suspend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "Maximum number of bytes in RXD buffer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxdMaxcnt(pub u32);
impl RxdMaxcnt {
    #[doc = "Maximum number of bytes in RXD buffer"]
    #[inline(always)]
    pub const fn maxcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Maximum number of bytes in RXD buffer"]
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
#[doc = "Number of bytes transferred in the last TXD transaction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxdAmount(pub u32);
impl TxdAmount {
    #[doc = "Number of bytes transferred in the last TXD transaction"]
    #[inline(always)]
    pub const fn amount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of bytes transferred in the last TXD transaction"]
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
#[doc = "Pin select for SDA signal"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sda(pub u32);
impl Sda {
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
    pub const fn connect(&self) -> super::vals::SdaConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SdaConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::SdaConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Sda {
    #[inline(always)]
    fn default() -> Sda {
        Sda(0)
    }
}
#[doc = "Description collection\\[n\\]: TWI slave address n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Address(pub u32);
impl Address {
    #[doc = "TWI slave address"]
    #[inline(always)]
    pub const fn address(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "TWI slave address"]
    #[inline(always)]
    pub fn set_address(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Address {
    #[inline(always)]
    fn default() -> Address {
        Address(0)
    }
}
