#[doc = "Pin select for SDA"]
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
    #[doc = "NACK received after sending the address (write '1' to clear)"]
    #[inline(always)]
    pub const fn anack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NACK received after sending the address (write '1' to clear)"]
    #[inline(always)]
    pub fn set_anack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NACK received after sending a data byte (write '1' to clear)"]
    #[inline(always)]
    pub const fn dnack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NACK received after sending a data byte (write '1' to clear)"]
    #[inline(always)]
    pub fn set_dnack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Errorsrc {
    #[inline(always)]
    fn default() -> Errorsrc {
        Errorsrc(0)
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
#[doc = "Shortcut register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between BB event and SUSPEND task"]
    #[inline(always)]
    pub const fn bb_suspend(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between BB event and SUSPEND task"]
    #[inline(always)]
    pub fn set_bb_suspend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Shortcut between BB event and STOP task"]
    #[inline(always)]
    pub const fn bb_stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between BB event and STOP task"]
    #[inline(always)]
    pub fn set_bb_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "Pin select for SCL"]
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
#[doc = "TXD register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txd(pub u32);
impl Txd {
    #[doc = "TXD register"]
    #[inline(always)]
    pub const fn txd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TXD register"]
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
#[doc = "Enable TWI"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable TWI"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Enable(val as u8)
    }
    #[doc = "Enable or disable TWI"]
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
    #[doc = "Write '1' to enable interrupt for RXDREADY event"]
    #[inline(always)]
    pub const fn rxdready(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for RXDREADY event"]
    #[inline(always)]
    pub fn set_rxdready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to enable interrupt for TXDSENT event"]
    #[inline(always)]
    pub const fn txdsent(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for TXDSENT event"]
    #[inline(always)]
    pub fn set_txdsent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    #[doc = "Write '1' to enable interrupt for BB event"]
    #[inline(always)]
    pub const fn bb(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for BB event"]
    #[inline(always)]
    pub fn set_bb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Write '1' to enable interrupt for SUSPENDED event"]
    #[inline(always)]
    pub const fn suspended(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for SUSPENDED event"]
    #[inline(always)]
    pub fn set_suspended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "RXD register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxd(pub u32);
impl Rxd {
    #[doc = "RXD register"]
    #[inline(always)]
    pub const fn rxd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RXD register"]
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
