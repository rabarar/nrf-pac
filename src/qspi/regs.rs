#[doc = "Pin select for serial data MISO/IO1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Io1(pub u32);
impl Io1 {
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
    pub const fn connect(&self) -> super::vals::Io1connect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Io1connect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::Io1connect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Io1 {
    #[inline(always)]
    fn default() -> Io1 {
        Io1(0)
    }
}
#[doc = "Size of block to be erased."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Len(pub u32);
impl Len {
    #[doc = "LEN"]
    #[inline(always)]
    pub const fn len(&self) -> super::vals::Len {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Len(val as u8)
    }
    #[doc = "LEN"]
    #[inline(always)]
    pub fn set_len(&mut self, val: super::vals::Len) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
}
impl Default for Len {
    #[inline(always)]
    fn default() -> Len {
        Len(0)
    }
}
#[doc = "Custom instruction data register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cinstrdat1(pub u32);
impl Cinstrdat1 {
    #[doc = "Data byte 4"]
    #[inline(always)]
    pub const fn byte4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 4"]
    #[inline(always)]
    pub fn set_byte4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 5"]
    #[inline(always)]
    pub const fn byte5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 5"]
    #[inline(always)]
    pub fn set_byte5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 6"]
    #[inline(always)]
    pub const fn byte6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 6"]
    #[inline(always)]
    pub fn set_byte6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 7"]
    #[inline(always)]
    pub const fn byte7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 7"]
    #[inline(always)]
    pub fn set_byte7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cinstrdat1 {
    #[inline(always)]
    fn default() -> Cinstrdat1 {
        Cinstrdat1(0)
    }
}
#[doc = "Enable QSPI peripheral and acquire the pins selected in PSELn registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enable(pub u32);
impl Enable {
    #[doc = "Enable or disable QSPI"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable(val as u8)
    }
    #[doc = "Enable or disable QSPI"]
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
#[doc = "Set the duration required to enter/exit deep power-down mode (DPM)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpmdur(pub u32);
impl Dpmdur {
    #[doc = "Duration needed by external flash to enter DPM. Duration is given as ENTER * 256 * 62.5 ns."]
    #[inline(always)]
    pub const fn enter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Duration needed by external flash to enter DPM. Duration is given as ENTER * 256 * 62.5 ns."]
    #[inline(always)]
    pub fn set_enter(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Duration needed by external flash to exit DPM. Duration is given as EXIT * 256 * 62.5 ns."]
    #[inline(always)]
    pub const fn exit(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Duration needed by external flash to exit DPM. Duration is given as EXIT * 256 * 62.5 ns."]
    #[inline(always)]
    pub fn set_exit(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Dpmdur {
    #[inline(always)]
    fn default() -> Dpmdur {
        Dpmdur(0)
    }
}
#[doc = "SPI interface timing."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iftiming(pub u32);
impl Iftiming {
    #[doc = "Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of 64 MHz cycles (15.625 ns) delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. As en example, if set to 0 the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub const fn rxdelay(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Timing related to sampling of the input serial data. The value of RXDELAY specifies the number of 64 MHz cycles (15.625 ns) delay from the the rising edge of the SPI Clock (SCK) until the input serial data is sampled. As en example, if set to 0 the input serial data is sampled on the rising edge of SCK."]
    #[inline(always)]
    pub fn set_rxdelay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for Iftiming {
    #[inline(always)]
    fn default() -> Iftiming {
        Iftiming(0)
    }
}
#[doc = "Write transfer length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WriteCnt(pub u32);
impl WriteCnt {
    #[doc = "Write transfer length in number of bytes. The length must be a multiple of 4 bytes."]
    #[inline(always)]
    pub const fn cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "Write transfer length in number of bytes. The length must be a multiple of 4 bytes."]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
    }
}
impl Default for WriteCnt {
    #[inline(always)]
    fn default() -> WriteCnt {
        WriteCnt(0)
    }
}
#[doc = "Custom instruction configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cinstrconf(pub u32);
impl Cinstrconf {
    #[doc = "Opcode of Custom instruction."]
    #[inline(always)]
    pub const fn opcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Opcode of Custom instruction."]
    #[inline(always)]
    pub fn set_opcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Length of custom instruction in number of bytes."]
    #[inline(always)]
    pub const fn length(&self) -> super::vals::Length {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Length(val as u8)
    }
    #[doc = "Length of custom instruction in number of bytes."]
    #[inline(always)]
    pub fn set_length(&mut self, val: super::vals::Length) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.0 as u32) & 0x0f) << 8usize);
    }
    #[doc = "Level of the IO2 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    pub const fn lio2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Level of the IO2 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    pub fn set_lio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Level of the IO3 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    pub const fn lio3(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Level of the IO3 pin (if connected) during transmission of custom instruction."]
    #[inline(always)]
    pub fn set_lio3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Wait for write complete before sending command."]
    #[inline(always)]
    pub const fn wipwait(&self) -> super::vals::CinstrconfWipwait {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::CinstrconfWipwait(val as u8)
    }
    #[doc = "Wait for write complete before sending command."]
    #[inline(always)]
    pub fn set_wipwait(&mut self, val: super::vals::CinstrconfWipwait) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.0 as u32) & 0x01) << 14usize);
    }
    #[doc = "Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub const fn wren(&self) -> super::vals::CinstrconfWren {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::CinstrconfWren(val as u8)
    }
    #[doc = "Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn set_wren(&mut self, val: super::vals::CinstrconfWren) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.0 as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field."]
    #[inline(always)]
    pub const fn lfen(&self) -> super::vals::Lfen {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Lfen(val as u8)
    }
    #[doc = "Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field."]
    #[inline(always)]
    pub fn set_lfen(&mut self, val: super::vals::Lfen) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
    }
    #[doc = "Stop (finalize) long frame transaction"]
    #[inline(always)]
    pub const fn lfstop(&self) -> super::vals::Lfstop {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Lfstop(val as u8)
    }
    #[doc = "Stop (finalize) long frame transaction"]
    #[inline(always)]
    pub fn set_lfstop(&mut self, val: super::vals::Lfstop) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.0 as u32) & 0x01) << 17usize);
    }
}
impl Default for Cinstrconf {
    #[inline(always)]
    fn default() -> Cinstrconf {
        Cinstrconf(0)
    }
}
#[doc = "Pin select for serial data MOSI/IO0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Io0(pub u32);
impl Io0 {
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
    pub const fn connect(&self) -> super::vals::Io0connect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Io0connect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::Io0connect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Io0 {
    #[inline(always)]
    fn default() -> Io0 {
        Io0(0)
    }
}
#[doc = "Pin select for serial data IO3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Io3(pub u32);
impl Io3 {
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
    pub const fn connect(&self) -> super::vals::Io3connect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Io3connect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::Io3connect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Io3 {
    #[inline(always)]
    fn default() -> Io3 {
        Io3(0)
    }
}
#[doc = "Enable or disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Enable or disable interrupt for READY event"]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable or disable interrupt for READY event"]
    #[inline(always)]
    pub fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Deep power-down mode (DPM) status of external flash."]
    #[inline(always)]
    pub const fn dpm(&self) -> super::vals::Dpm {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dpm(val as u8)
    }
    #[doc = "Deep power-down mode (DPM) status of external flash."]
    #[inline(always)]
    pub fn set_dpm(&mut self, val: super::vals::Dpm) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
    }
    #[doc = "Ready status."]
    #[inline(always)]
    pub const fn ready(&self) -> super::vals::StatusReady {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::StatusReady(val as u8)
    }
    #[doc = "Ready status."]
    #[inline(always)]
    pub fn set_ready(&mut self, val: super::vals::StatusReady) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
    }
    #[doc = "Value of external flash device Status Register. When the external flash has two bytes status register this field includes the value of the low byte."]
    #[inline(always)]
    pub const fn sreg(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Value of external flash device Status Register. When the external flash has two bytes status register this field includes the value of the low byte."]
    #[inline(always)]
    pub fn set_sreg(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "Read transfer length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReadCnt(pub u32);
impl ReadCnt {
    #[doc = "Read transfer length in number of bytes. The length must be a multiple of 4 bytes."]
    #[inline(always)]
    pub const fn cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "Read transfer length in number of bytes. The length must be a multiple of 4 bytes."]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
    }
}
impl Default for ReadCnt {
    #[inline(always)]
    fn default() -> ReadCnt {
        ReadCnt(0)
    }
}
#[doc = "Pin select for serial data IO2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Io2(pub u32);
impl Io2 {
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
    pub const fn connect(&self) -> super::vals::Io2connect {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Io2connect(val as u8)
    }
    #[doc = "Connection"]
    #[inline(always)]
    pub fn set_connect(&mut self, val: super::vals::Io2connect) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.0 as u32) & 0x01) << 31usize);
    }
}
impl Default for Io2 {
    #[inline(always)]
    fn default() -> Io2 {
        Io2(0)
    }
}
#[doc = "Pin select for chip select signal CSN."]
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
#[doc = "Custom instruction data register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cinstrdat0(pub u32);
impl Cinstrdat0 {
    #[doc = "Data byte 0"]
    #[inline(always)]
    pub const fn byte0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0"]
    #[inline(always)]
    pub fn set_byte0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1"]
    #[inline(always)]
    pub const fn byte1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1"]
    #[inline(always)]
    pub fn set_byte1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 2"]
    #[inline(always)]
    pub const fn byte2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2"]
    #[inline(always)]
    pub fn set_byte2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 3"]
    #[inline(always)]
    pub const fn byte3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3"]
    #[inline(always)]
    pub fn set_byte3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cinstrdat0 {
    #[inline(always)]
    fn default() -> Cinstrdat0 {
        Cinstrdat0(0)
    }
}
#[doc = "Interface configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifconfig0(pub u32);
impl Ifconfig0 {
    #[doc = "Configure number of data lines and opcode used for reading."]
    #[inline(always)]
    pub const fn readoc(&self) -> super::vals::Readoc {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Readoc(val as u8)
    }
    #[doc = "Configure number of data lines and opcode used for reading."]
    #[inline(always)]
    pub fn set_readoc(&mut self, val: super::vals::Readoc) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.0 as u32) & 0x07) << 0usize);
    }
    #[doc = "Configure number of data lines and opcode used for writing."]
    #[inline(always)]
    pub const fn writeoc(&self) -> super::vals::Writeoc {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Writeoc(val as u8)
    }
    #[doc = "Configure number of data lines and opcode used for writing."]
    #[inline(always)]
    pub fn set_writeoc(&mut self, val: super::vals::Writeoc) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.0 as u32) & 0x07) << 3usize);
    }
    #[doc = "Addressing mode."]
    #[inline(always)]
    pub const fn addrmode(&self) -> super::vals::Addrmode {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Addrmode(val as u8)
    }
    #[doc = "Addressing mode."]
    #[inline(always)]
    pub fn set_addrmode(&mut self, val: super::vals::Addrmode) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.0 as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable deep power-down mode (DPM) feature."]
    #[inline(always)]
    pub const fn dpmenable(&self) -> super::vals::Dpmenable {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dpmenable(val as u8)
    }
    #[doc = "Enable deep power-down mode (DPM) feature."]
    #[inline(always)]
    pub fn set_dpmenable(&mut self, val: super::vals::Dpmenable) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
    }
    #[doc = "Page size for commands PP, PP2O, PP4O and PP4IO."]
    #[inline(always)]
    pub const fn ppsize(&self) -> super::vals::Ppsize {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Ppsize(val as u8)
    }
    #[doc = "Page size for commands PP, PP2O, PP4O and PP4IO."]
    #[inline(always)]
    pub fn set_ppsize(&mut self, val: super::vals::Ppsize) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.0 as u32) & 0x01) << 12usize);
    }
}
impl Default for Ifconfig0 {
    #[inline(always)]
    fn default() -> Ifconfig0 {
        Ifconfig0(0)
    }
}
#[doc = "Interface configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifconfig1(pub u32);
impl Ifconfig1 {
    #[doc = "Minimum amount of time that the CSN pin must stay high before it can go low again. Value is specified in number of 16 MHz periods (62.5 ns)."]
    #[inline(always)]
    pub const fn sckdelay(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Minimum amount of time that the CSN pin must stay high before it can go low again. Value is specified in number of 16 MHz periods (62.5 ns)."]
    #[inline(always)]
    pub fn set_sckdelay(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Enter/exit deep power-down mode (DPM) for external flash memory."]
    #[inline(always)]
    pub const fn dpmen(&self) -> super::vals::Dpmen {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dpmen(val as u8)
    }
    #[doc = "Enter/exit deep power-down mode (DPM) for external flash memory."]
    #[inline(always)]
    pub fn set_dpmen(&mut self, val: super::vals::Dpmen) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.0 as u32) & 0x01) << 24usize);
    }
    #[doc = "Select SPI mode."]
    #[inline(always)]
    pub const fn spimode(&self) -> super::vals::Spimode {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Spimode(val as u8)
    }
    #[doc = "Select SPI mode."]
    #[inline(always)]
    pub fn set_spimode(&mut self, val: super::vals::Spimode) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.0 as u32) & 0x01) << 25usize);
    }
    #[doc = "SCK frequency is given as 32 MHz / (SCKFREQ + 1)."]
    #[inline(always)]
    pub const fn sckfreq(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "SCK frequency is given as 32 MHz / (SCKFREQ + 1)."]
    #[inline(always)]
    pub fn set_sckfreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ifconfig1 {
    #[inline(always)]
    fn default() -> Ifconfig1 {
        Ifconfig1(0)
    }
}
#[doc = "Extended address configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addrconf(pub u32);
impl Addrconf {
    #[doc = "Opcode that enters the 32-bit addressing mode."]
    #[inline(always)]
    pub const fn opcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Opcode that enters the 32-bit addressing mode."]
    #[inline(always)]
    pub fn set_opcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Byte 0 following opcode."]
    #[inline(always)]
    pub const fn byte0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Byte 0 following opcode."]
    #[inline(always)]
    pub fn set_byte0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Byte 1 following byte 0."]
    #[inline(always)]
    pub const fn byte1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Byte 1 following byte 0."]
    #[inline(always)]
    pub fn set_byte1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Extended addressing mode."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Mode(val as u8)
    }
    #[doc = "Extended addressing mode."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.0 as u32) & 0x03) << 24usize);
    }
    #[doc = "Wait for write complete before sending command."]
    #[inline(always)]
    pub const fn wipwait(&self) -> super::vals::AddrconfWipwait {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::AddrconfWipwait(val as u8)
    }
    #[doc = "Wait for write complete before sending command."]
    #[inline(always)]
    pub fn set_wipwait(&mut self, val: super::vals::AddrconfWipwait) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.0 as u32) & 0x01) << 26usize);
    }
    #[doc = "Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub const fn wren(&self) -> super::vals::AddrconfWren {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::AddrconfWren(val as u8)
    }
    #[doc = "Send WREN (write enable opcode 0x06) before instruction."]
    #[inline(always)]
    pub fn set_wren(&mut self, val: super::vals::AddrconfWren) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.0 as u32) & 0x01) << 27usize);
    }
}
impl Default for Addrconf {
    #[inline(always)]
    fn default() -> Addrconf {
        Addrconf(0)
    }
}
#[doc = "Pin select for serial clock SCK"]
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
