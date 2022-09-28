#[doc = "EasyDMA list type"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxdList(pub u32);
impl TxdList {
    #[doc = "List type"]
    #[inline(always)]
    pub const fn list(&self) -> super::vals::TxdListList {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::TxdListList(val as u8)
    }
    #[doc = "List type"]
    #[inline(always)]
    pub fn set_list(&mut self, val: super::vals::TxdListList) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.0 as u32) & 0x07) << 0usize);
    }
}
impl Default for TxdList {
    #[inline(always)]
    fn default() -> TxdList {
        TxdList(0)
    }
}
#[doc = "Shortcut register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between LASTTX event and STARTRX task"]
    #[inline(always)]
    pub const fn lasttx_startrx(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between LASTTX event and STARTRX task"]
    #[inline(always)]
    pub fn set_lasttx_startrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Shortcut between LASTTX event and SUSPEND task"]
    #[inline(always)]
    pub const fn lasttx_suspend(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between LASTTX event and SUSPEND task"]
    #[inline(always)]
    pub fn set_lasttx_suspend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Shortcut between LASTTX event and STOP task"]
    #[inline(always)]
    pub const fn lasttx_stop(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between LASTTX event and STOP task"]
    #[inline(always)]
    pub fn set_lasttx_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Shortcut between LASTRX event and STARTTX task"]
    #[inline(always)]
    pub const fn lastrx_starttx(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between LASTRX event and STARTTX task"]
    #[inline(always)]
    pub fn set_lastrx_starttx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Shortcut between LASTRX event and SUSPEND task"]
    #[inline(always)]
    pub const fn lastrx_suspend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between LASTRX event and SUSPEND task"]
    #[inline(always)]
    pub fn set_lastrx_suspend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Shortcut between LASTRX event and STOP task"]
    #[inline(always)]
    pub const fn lastrx_stop(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between LASTRX event and STOP task"]
    #[inline(always)]
    pub fn set_lastrx_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frequency(pub u32);
impl Frequency {
    #[doc = "TWI master clock frequency"]
    #[inline(always)]
    pub const fn frequency(&self) -> super::vals::Frequency {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Frequency(val as u32)
    }
    #[doc = "TWI master clock frequency"]
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
#[doc = "EasyDMA list type"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxdList(pub u32);
impl RxdList {
    #[doc = "List type"]
    #[inline(always)]
    pub const fn list(&self) -> super::vals::RxdListList {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::RxdListList(val as u8)
    }
    #[doc = "List type"]
    #[inline(always)]
    pub fn set_list(&mut self, val: super::vals::RxdListList) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.0 as u32) & 0x07) << 0usize);
    }
}
impl Default for RxdList {
    #[inline(always)]
    fn default() -> RxdList {
        RxdList(0)
    }
}
#[doc = "Number of bytes transferred in the last transaction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxdAmount(pub u32);
impl RxdAmount {
    #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
    #[inline(always)]
    pub const fn amount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
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
#[doc = "Number of bytes transferred in the last transaction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxdAmount(pub u32);
impl TxdAmount {
    #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
    #[inline(always)]
    pub const fn amount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
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
#[doc = "Enable TWIM"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable TWIM"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Enable(val as u8)
    }
    #[doc = "Enable or disable TWIM"]
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
    #[doc = "Write '1' to disable interrupt for SUSPENDED event"]
    #[inline(always)]
    pub const fn suspended(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for SUSPENDED event"]
    #[inline(always)]
    pub fn set_suspended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to disable interrupt for RXSTARTED event"]
    #[inline(always)]
    pub const fn rxstarted(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for RXSTARTED event"]
    #[inline(always)]
    pub fn set_rxstarted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Write '1' to disable interrupt for TXSTARTED event"]
    #[inline(always)]
    pub const fn txstarted(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for TXSTARTED event"]
    #[inline(always)]
    pub fn set_txstarted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Write '1' to disable interrupt for LASTRX event"]
    #[inline(always)]
    pub const fn lastrx(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for LASTRX event"]
    #[inline(always)]
    pub fn set_lastrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Write '1' to disable interrupt for LASTTX event"]
    #[inline(always)]
    pub const fn lasttx(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for LASTTX event"]
    #[inline(always)]
    pub fn set_lasttx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
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
#[doc = "Address used in the TWI transfer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Address(pub u32);
impl Address {
    #[doc = "Address used in the TWI transfer"]
    #[inline(always)]
    pub const fn address(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Address used in the TWI transfer"]
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
#[doc = "Error source"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errorsrc(pub u32);
impl Errorsrc {
    #[doc = "Overrun error"]
    #[inline(always)]
    pub const fn overrun(&self) -> super::vals::Overrun {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Overrun(val as u8)
    }
    #[doc = "Overrun error"]
    #[inline(always)]
    pub fn set_overrun(&mut self, val: super::vals::Overrun) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "NACK received after sending the address (write '1' to clear)"]
    #[inline(always)]
    pub const fn anack(&self) -> super::vals::Anack {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Anack(val as u8)
    }
    #[doc = "NACK received after sending the address (write '1' to clear)"]
    #[inline(always)]
    pub fn set_anack(&mut self, val: super::vals::Anack) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
    }
    #[doc = "NACK received after sending a data byte (write '1' to clear)"]
    #[inline(always)]
    pub const fn dnack(&self) -> super::vals::Dnack {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dnack(val as u8)
    }
    #[doc = "NACK received after sending a data byte (write '1' to clear)"]
    #[inline(always)]
    pub fn set_dnack(&mut self, val: super::vals::Dnack) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
    }
}
impl Default for Errorsrc {
    #[inline(always)]
    fn default() -> Errorsrc {
        Errorsrc(0)
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
