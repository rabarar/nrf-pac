#[doc = "Reception (RX) enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxen(pub u32);
impl Rxen {
    #[doc = "Reception (RX) enable."]
    #[inline(always)]
    pub const fn rxen(&self) -> super::vals::Rxen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rxen(val as u8)
    }
    #[doc = "Reception (RX) enable."]
    #[inline(always)]
    pub fn set_rxen(&mut self, val: super::vals::Rxen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Rxen {
    #[inline(always)]
    fn default() -> Rxen {
        Rxen(0)
    }
}
#[doc = "Frame format."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Format(pub u32);
impl Format {
    #[doc = "Frame format."]
    #[inline(always)]
    pub const fn format(&self) -> super::vals::Format {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Format(val as u8)
    }
    #[doc = "Frame format."]
    #[inline(always)]
    pub fn set_format(&mut self, val: super::vals::Format) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Format {
    #[inline(always)]
    fn default() -> Format {
        Format(0)
    }
}
#[doc = "MCK / LRCK ratio."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ratio(pub u32);
impl Ratio {
    #[doc = "MCK / LRCK ratio."]
    #[inline(always)]
    pub const fn ratio(&self) -> super::vals::Ratio {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Ratio(val as u8)
    }
    #[doc = "MCK / LRCK ratio."]
    #[inline(always)]
    pub fn set_ratio(&mut self, val: super::vals::Ratio) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.0 as u32) & 0x0f) << 0usize);
    }
}
impl Default for Ratio {
    #[inline(always)]
    fn default() -> Ratio {
        Ratio(0)
    }
}
#[doc = "Enable channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channels(pub u32);
impl Channels {
    #[doc = "Enable channels."]
    #[inline(always)]
    pub const fn channels(&self) -> super::vals::Channels {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Channels(val as u8)
    }
    #[doc = "Enable channels."]
    #[inline(always)]
    pub fn set_channels(&mut self, val: super::vals::Channels) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
}
impl Default for Channels {
    #[inline(always)]
    fn default() -> Channels {
        Channels(0)
    }
}
#[doc = "Pin select for MCK signal."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mck(pub u32);
impl Mck {
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
    pub const fn connect(&self) -> super::vals::MckConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MckConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::MckConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Mck {
    #[inline(always)]
    fn default() -> Mck {
        Mck(0)
    }
}
#[doc = "Pin select for SDOUT signal."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdout(pub u32);
impl Sdout {
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
    pub const fn connect(&self) -> super::vals::SdoutConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SdoutConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::SdoutConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Sdout {
    #[inline(always)]
    fn default() -> Sdout {
        Sdout(0)
    }
}
#[doc = "Pin select for SDIN signal."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdin(pub u32);
impl Sdin {
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
    pub const fn connect(&self) -> super::vals::SdinConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SdinConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::SdinConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Sdin {
    #[inline(always)]
    fn default() -> Sdin {
        Sdin(0)
    }
}
#[doc = "Enable I2S module."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable I2S module."]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable(val as u8)
    }
    #[doc = "Enable I2S module."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Enable {
    #[inline(always)]
    fn default() -> Enable {
        Enable(0)
    }
}
#[doc = "I2S mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "I2S mode."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mode(val as u8)
    }
    #[doc = "I2S mode."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
#[doc = "Size of RXD and TXD buffers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Maxcnt(pub u32);
impl Maxcnt {
    #[doc = "Size of RXD and TXD buffers in number of 32 bit words."]
    #[inline(always)]
    pub const fn maxcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Size of RXD and TXD buffers in number of 32 bit words."]
    #[inline(always)]
    pub fn set_maxcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Maxcnt {
    #[inline(always)]
    fn default() -> Maxcnt {
        Maxcnt(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for RXPTRUPD event"]
    #[inline(always)]
    pub const fn rxptrupd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for RXPTRUPD event"]
    #[inline(always)]
    pub fn set_rxptrupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable interrupt for STOPPED event"]
    #[inline(always)]
    pub const fn stopped(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn set_stopped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to disable interrupt for TXPTRUPD event"]
    #[inline(always)]
    pub const fn txptrupd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for TXPTRUPD event"]
    #[inline(always)]
    pub fn set_txptrupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Transmission (TX) enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txen(pub u32);
impl Txen {
    #[doc = "Transmission (TX) enable."]
    #[inline(always)]
    pub const fn txen(&self) -> super::vals::Txen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Txen(val as u8)
    }
    #[doc = "Transmission (TX) enable."]
    #[inline(always)]
    pub fn set_txen(&mut self, val: super::vals::Txen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Txen {
    #[inline(always)]
    fn default() -> Txen {
        Txen(0)
    }
}
#[doc = "Master clock generator enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcken(pub u32);
impl Mcken {
    #[doc = "Master clock generator enable."]
    #[inline(always)]
    pub const fn mcken(&self) -> super::vals::Mcken {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mcken(val as u8)
    }
    #[doc = "Master clock generator enable."]
    #[inline(always)]
    pub fn set_mcken(&mut self, val: super::vals::Mcken) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Mcken {
    #[inline(always)]
    fn default() -> Mcken {
        Mcken(0)
    }
}
#[doc = "Pin select for SCK signal."]
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
#[doc = "Alignment of sample within a frame."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Align(pub u32);
impl Align {
    #[doc = "Alignment of sample within a frame."]
    #[inline(always)]
    pub const fn align(&self) -> super::vals::Align {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Align(val as u8)
    }
    #[doc = "Alignment of sample within a frame."]
    #[inline(always)]
    pub fn set_align(&mut self, val: super::vals::Align) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Align {
    #[inline(always)]
    fn default() -> Align {
        Align(0)
    }
}
#[doc = "Sample width."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swidth(pub u32);
impl Swidth {
    #[doc = "Sample width."]
    #[inline(always)]
    pub const fn swidth(&self) -> super::vals::Swidth {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Swidth(val as u8)
    }
    #[doc = "Sample width."]
    #[inline(always)]
    pub fn set_swidth(&mut self, val: super::vals::Swidth) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
}
impl Default for Swidth {
    #[inline(always)]
    fn default() -> Swidth {
        Swidth(0)
    }
}
#[doc = "Pin select for LRCK signal."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lrck(pub u32);
impl Lrck {
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
    pub const fn connect(&self) -> super::vals::LrckConnect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::LrckConnect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::LrckConnect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Lrck {
    #[inline(always)]
    fn default() -> Lrck {
        Lrck(0)
    }
}
#[doc = "Master clock generator frequency."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mckfreq(pub u32);
impl Mckfreq {
    #[doc = "Master clock generator frequency."]
    #[inline(always)]
    pub const fn mckfreq(&self) -> super::vals::Mckfreq {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Mckfreq(val as u32)
    }
    #[doc = "Master clock generator frequency."]
    #[inline(always)]
    pub fn set_mckfreq(&mut self, val: super::vals::Mckfreq) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.0 as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mckfreq {
    #[inline(always)]
    fn default() -> Mckfreq {
        Mckfreq(0)
    }
}
