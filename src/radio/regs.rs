#[doc = "Current radio state"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State(pub u32);
impl State {
    #[doc = "Current radio state"]
    #[inline(always)]
    pub const fn state(&self) -> super::vals::State {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::State(val as u8)
    }
    #[doc = "Current radio state"]
    #[inline(always)]
    pub fn set_state(&mut self, val: super::vals::State) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.0 as u32) & 0x0f) << 0usize);
    }
}
impl Default for State {
    #[inline(always)]
    fn default() -> State {
        State(0)
    }
}
#[doc = "CRC configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crccnf(pub u32);
impl Crccnf {
    #[doc = "CRC length in number of bytes."]
    #[inline(always)]
    pub const fn len(&self) -> super::vals::Len {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Len(val as u8)
    }
    #[doc = "CRC length in number of bytes."]
    #[inline(always)]
    pub fn set_len(&mut self, val: super::vals::Len) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.0 as u32) & 0x03) << 0usize);
    }
    #[doc = "Include or exclude packet address field out of CRC calculation."]
    #[inline(always)]
    pub const fn skipaddr(&self) -> super::vals::Skipaddr {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Skipaddr(val as u8)
    }
    #[doc = "Include or exclude packet address field out of CRC calculation."]
    #[inline(always)]
    pub fn set_skipaddr(&mut self, val: super::vals::Skipaddr) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.0 as u32) & 0x03) << 8usize);
    }
}
impl Default for Crccnf {
    #[inline(always)]
    fn default() -> Crccnf {
        Crccnf(0)
    }
}
#[doc = "CRC polynomial"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcpoly(pub u32);
impl Crcpoly {
    #[doc = "CRC polynomial"]
    #[inline(always)]
    pub const fn crcpoly(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "CRC polynomial"]
    #[inline(always)]
    pub fn set_crcpoly(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crcpoly {
    #[inline(always)]
    fn default() -> Crcpoly {
        Crcpoly(0)
    }
}
#[doc = "Shortcut register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shorts(pub u32);
impl Shorts {
    #[doc = "Shortcut between READY event and START task"]
    #[inline(always)]
    pub const fn ready_start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between READY event and START task"]
    #[inline(always)]
    pub fn set_ready_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Shortcut between END event and DISABLE task"]
    #[inline(always)]
    pub const fn end_disable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between END event and DISABLE task"]
    #[inline(always)]
    pub fn set_end_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Shortcut between DISABLED event and TXEN task"]
    #[inline(always)]
    pub const fn disabled_txen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between DISABLED event and TXEN task"]
    #[inline(always)]
    pub fn set_disabled_txen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Shortcut between DISABLED event and RXEN task"]
    #[inline(always)]
    pub const fn disabled_rxen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between DISABLED event and RXEN task"]
    #[inline(always)]
    pub fn set_disabled_rxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Shortcut between ADDRESS event and RSSISTART task"]
    #[inline(always)]
    pub const fn address_rssistart(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between ADDRESS event and RSSISTART task"]
    #[inline(always)]
    pub fn set_address_rssistart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Shortcut between END event and START task"]
    #[inline(always)]
    pub const fn end_start(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between END event and START task"]
    #[inline(always)]
    pub fn set_end_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Shortcut between ADDRESS event and BCSTART task"]
    #[inline(always)]
    pub const fn address_bcstart(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between ADDRESS event and BCSTART task"]
    #[inline(always)]
    pub fn set_address_bcstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Shortcut between DISABLED event and RSSISTOP task"]
    #[inline(always)]
    pub const fn disabled_rssistop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between DISABLED event and RSSISTOP task"]
    #[inline(always)]
    pub fn set_disabled_rssistop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Shortcut between RXREADY event and CCASTART task"]
    #[inline(always)]
    pub const fn rxready_ccastart(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between RXREADY event and CCASTART task"]
    #[inline(always)]
    pub fn set_rxready_ccastart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Shortcut between CCAIDLE event and TXEN task"]
    #[inline(always)]
    pub const fn ccaidle_txen(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between CCAIDLE event and TXEN task"]
    #[inline(always)]
    pub fn set_ccaidle_txen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Shortcut between CCABUSY event and DISABLE task"]
    #[inline(always)]
    pub const fn ccabusy_disable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between CCABUSY event and DISABLE task"]
    #[inline(always)]
    pub fn set_ccabusy_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Shortcut between FRAMESTART event and BCSTART task"]
    #[inline(always)]
    pub const fn framestart_bcstart(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between FRAMESTART event and BCSTART task"]
    #[inline(always)]
    pub fn set_framestart_bcstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Shortcut between READY event and EDSTART task"]
    #[inline(always)]
    pub const fn ready_edstart(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between READY event and EDSTART task"]
    #[inline(always)]
    pub fn set_ready_edstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Shortcut between EDEND event and DISABLE task"]
    #[inline(always)]
    pub const fn edend_disable(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between EDEND event and DISABLE task"]
    #[inline(always)]
    pub fn set_edend_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Shortcut between CCAIDLE event and STOP task"]
    #[inline(always)]
    pub const fn ccaidle_stop(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between CCAIDLE event and STOP task"]
    #[inline(always)]
    pub fn set_ccaidle_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Shortcut between TXREADY event and START task"]
    #[inline(always)]
    pub const fn txready_start(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between TXREADY event and START task"]
    #[inline(always)]
    pub fn set_txready_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Shortcut between RXREADY event and START task"]
    #[inline(always)]
    pub const fn rxready_start(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between RXREADY event and START task"]
    #[inline(always)]
    pub fn set_rxready_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Shortcut between PHYEND event and DISABLE task"]
    #[inline(always)]
    pub const fn phyend_disable(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between PHYEND event and DISABLE task"]
    #[inline(always)]
    pub fn set_phyend_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Shortcut between PHYEND event and START task"]
    #[inline(always)]
    pub const fn phyend_start(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Shortcut between PHYEND event and START task"]
    #[inline(always)]
    pub fn set_phyend_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Shorts {
    #[inline(always)]
    fn default() -> Shorts {
        Shorts(0)
    }
}
#[doc = "Frequency"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frequency(pub u32);
impl Frequency {
    #[doc = "Radio channel frequency"]
    #[inline(always)]
    pub const fn frequency(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Radio channel frequency"]
    #[inline(always)]
    pub fn set_frequency(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Channel map selection."]
    #[inline(always)]
    pub const fn map(&self) -> super::vals::Map {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Map(val as u8)
    }
    #[doc = "Channel map selection."]
    #[inline(always)]
    pub fn set_map(&mut self, val: super::vals::Map) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.0 as u32) & 0x01) << 8usize);
    }
}
impl Default for Frequency {
    #[inline(always)]
    fn default() -> Frequency {
        Frequency(0)
    }
}
#[doc = "CRC field of previously received packet"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxcrc(pub u32);
impl Rxcrc {
    #[doc = "CRC field of previously received packet"]
    #[inline(always)]
    pub const fn rxcrc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "CRC field of previously received packet"]
    #[inline(always)]
    pub fn set_rxcrc(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Rxcrc {
    #[inline(always)]
    fn default() -> Rxcrc {
        Rxcrc(0)
    }
}
#[doc = "IEEE 802.15.4 energy detect loop count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edcnt(pub u32);
impl Edcnt {
    #[doc = "IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    pub const fn edcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    pub fn set_edcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
    }
}
impl Default for Edcnt {
    #[inline(always)]
    fn default() -> Edcnt {
        Edcnt(0)
    }
}
#[doc = "IEEE 802.15.4 energy detect level"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edsample(pub u32);
impl Edsample {
    #[doc = "IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub const fn edlvl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub fn set_edlvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Edsample {
    #[inline(always)]
    fn default() -> Edsample {
        Edsample(0)
    }
}
#[doc = "Packet configuration register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcnf1(pub u32);
impl Pcnf1 {
    #[doc = "Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
    #[inline(always)]
    pub const fn maxlen(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
    #[inline(always)]
    pub fn set_maxlen(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Static length in number of bytes"]
    #[inline(always)]
    pub const fn statlen(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Static length in number of bytes"]
    #[inline(always)]
    pub fn set_statlen(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Base address length in number of bytes"]
    #[inline(always)]
    pub const fn balen(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Base address length in number of bytes"]
    #[inline(always)]
    pub fn set_balen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields."]
    #[inline(always)]
    pub const fn endian(&self) -> super::vals::Endian {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Endian(val as u8)
    }
    #[doc = "On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields."]
    #[inline(always)]
    pub fn set_endian(&mut self, val: super::vals::Endian) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.0 as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable or disable packet whitening"]
    #[inline(always)]
    pub const fn whiteen(&self) -> super::vals::Whiteen {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Whiteen(val as u8)
    }
    #[doc = "Enable or disable packet whitening"]
    #[inline(always)]
    pub fn set_whiteen(&mut self, val: super::vals::Whiteen) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.0 as u32) & 0x01) << 25usize);
    }
}
impl Default for Pcnf1 {
    #[inline(always)]
    fn default() -> Pcnf1 {
        Pcnf1(0)
    }
}
#[doc = "RSSI sample"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rssisample(pub u32);
impl Rssisample {
    #[doc = "RSSI sample"]
    #[inline(always)]
    pub const fn rssisample(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "RSSI sample"]
    #[inline(always)]
    pub fn set_rssisample(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Rssisample {
    #[inline(always)]
    fn default() -> Rssisample {
        Rssisample(0)
    }
}
#[doc = "Data rate and modulation"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Mode(val as u8)
    }
    #[doc = "Radio data rate and modulation setting. The radio supports frequency-shift keying (FSK) modulation."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.0 as u32) & 0x0f) << 0usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
#[doc = "Device address match index"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dai(pub u32);
impl Dai {
    #[doc = "Device address match index"]
    #[inline(always)]
    pub const fn dai(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Device address match index"]
    #[inline(always)]
    pub fn set_dai(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Dai {
    #[inline(always)]
    fn default() -> Dai {
        Dai(0)
    }
}
#[doc = "CRC status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcstatus(pub u32);
impl Crcstatus {
    #[doc = "CRC status of packet received"]
    #[inline(always)]
    pub const fn crcstatus(&self) -> super::vals::Crcstatus {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Crcstatus(val as u8)
    }
    #[doc = "CRC status of packet received"]
    #[inline(always)]
    pub fn set_crcstatus(&mut self, val: super::vals::Crcstatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Crcstatus {
    #[inline(always)]
    fn default() -> Crcstatus {
        Crcstatus(0)
    }
}
#[doc = "Prefixes bytes for logical addresses 0-3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prefix0(pub u32);
impl Prefix0 {
    #[doc = "Address prefix 0."]
    #[inline(always)]
    pub const fn ap0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 0."]
    #[inline(always)]
    pub fn set_ap0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address prefix 1."]
    #[inline(always)]
    pub const fn ap1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 1."]
    #[inline(always)]
    pub fn set_ap1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address prefix 2."]
    #[inline(always)]
    pub const fn ap2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 2."]
    #[inline(always)]
    pub fn set_ap2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address prefix 3."]
    #[inline(always)]
    pub const fn ap3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 3."]
    #[inline(always)]
    pub fn set_ap3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Prefix0 {
    #[inline(always)]
    fn default() -> Prefix0 {
        Prefix0(0)
    }
}
#[doc = "Radio mode configuration register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modecnf0(pub u32);
impl Modecnf0 {
    #[doc = "Radio ramp-up time"]
    #[inline(always)]
    pub const fn ru(&self) -> super::vals::Ru {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ru(val as u8)
    }
    #[doc = "Radio ramp-up time"]
    #[inline(always)]
    pub fn set_ru(&mut self, val: super::vals::Ru) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Default TX value"]
    #[inline(always)]
    pub const fn dtx(&self) -> super::vals::Dtx {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Dtx(val as u8)
    }
    #[doc = "Default TX value"]
    #[inline(always)]
    pub fn set_dtx(&mut self, val: super::vals::Dtx) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.0 as u32) & 0x03) << 8usize);
    }
}
impl Default for Modecnf0 {
    #[inline(always)]
    fn default() -> Modecnf0 {
        Modecnf0(0)
    }
}
#[doc = "Transmit address select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txaddress(pub u32);
impl Txaddress {
    #[doc = "Transmit address select"]
    #[inline(always)]
    pub const fn txaddress(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit address select"]
    #[inline(always)]
    pub fn set_txaddress(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Txaddress {
    #[inline(always)]
    fn default() -> Txaddress {
        Txaddress(0)
    }
}
#[doc = "Interframe spacing in us"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tifs(pub u32);
impl Tifs {
    #[doc = "Interframe spacing in us"]
    #[inline(always)]
    pub const fn tifs(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Interframe spacing in us"]
    #[inline(always)]
    pub fn set_tifs(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Tifs {
    #[inline(always)]
    fn default() -> Tifs {
        Tifs(0)
    }
}
#[doc = "Data whitening initial value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Datawhiteiv(pub u32);
impl Datawhiteiv {
    #[doc = "Data whitening initial value. Bit 6 is hard-wired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
    #[inline(always)]
    pub const fn datawhiteiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Data whitening initial value. Bit 6 is hard-wired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
    #[inline(always)]
    pub fn set_datawhiteiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Datawhiteiv {
    #[inline(always)]
    fn default() -> Datawhiteiv {
        Datawhiteiv(0)
    }
}
#[doc = "Received address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxmatch(pub u32);
impl Rxmatch {
    #[doc = "Received address"]
    #[inline(always)]
    pub const fn rxmatch(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Received address"]
    #[inline(always)]
    pub fn set_rxmatch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Rxmatch {
    #[inline(always)]
    fn default() -> Rxmatch {
        Rxmatch(0)
    }
}
#[doc = "Prefixes bytes for logical addresses 4-7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prefix1(pub u32);
impl Prefix1 {
    #[doc = "Address prefix 4."]
    #[inline(always)]
    pub const fn ap4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 4."]
    #[inline(always)]
    pub fn set_ap4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address prefix 5."]
    #[inline(always)]
    pub const fn ap5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 5."]
    #[inline(always)]
    pub fn set_ap5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address prefix 6."]
    #[inline(always)]
    pub const fn ap6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 6."]
    #[inline(always)]
    pub fn set_ap6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address prefix 7."]
    #[inline(always)]
    pub const fn ap7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address prefix 7."]
    #[inline(always)]
    pub fn set_ap7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Prefix1 {
    #[inline(always)]
    fn default() -> Prefix1 {
        Prefix1(0)
    }
}
#[doc = "Packet configuration register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcnf0(pub u32);
impl Pcnf0 {
    #[doc = "Length on air of LENGTH field in number of bits."]
    #[inline(always)]
    pub const fn lflen(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Length on air of LENGTH field in number of bits."]
    #[inline(always)]
    pub fn set_lflen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Length on air of S0 field in number of bytes."]
    #[inline(always)]
    pub const fn s0len(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Length on air of S0 field in number of bytes."]
    #[inline(always)]
    pub fn set_s0len(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Length on air of S1 field in number of bits."]
    #[inline(always)]
    pub const fn s1len(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Length on air of S1 field in number of bits."]
    #[inline(always)]
    pub fn set_s1len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Include or exclude S1 field in RAM"]
    #[inline(always)]
    pub const fn s1incl(&self) -> super::vals::S1incl {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::S1incl(val as u8)
    }
    #[doc = "Include or exclude S1 field in RAM"]
    #[inline(always)]
    pub fn set_s1incl(&mut self, val: super::vals::S1incl) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.0 as u32) & 0x01) << 20usize);
    }
    #[doc = "Length of code indicator - long range"]
    #[inline(always)]
    pub const fn cilen(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Length of code indicator - long range"]
    #[inline(always)]
    pub fn set_cilen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Length of preamble on air. Decision point: TASKS_START task"]
    #[inline(always)]
    pub const fn plen(&self) -> super::vals::Plen {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Plen(val as u8)
    }
    #[doc = "Length of preamble on air. Decision point: TASKS_START task"]
    #[inline(always)]
    pub fn set_plen(&mut self, val: super::vals::Plen) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.0 as u32) & 0x03) << 24usize);
    }
    #[doc = "Indicates if LENGTH field contains CRC or not"]
    #[inline(always)]
    pub const fn crcinc(&self) -> super::vals::Crcinc {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Crcinc(val as u8)
    }
    #[doc = "Indicates if LENGTH field contains CRC or not"]
    #[inline(always)]
    pub fn set_crcinc(&mut self, val: super::vals::Crcinc) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.0 as u32) & 0x01) << 26usize);
    }
    #[doc = "Length of TERM field in Long Range operation"]
    #[inline(always)]
    pub const fn termlen(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "Length of TERM field in Long Range operation"]
    #[inline(always)]
    pub fn set_termlen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for Pcnf0 {
    #[inline(always)]
    fn default() -> Pcnf0 {
        Pcnf0(0)
    }
}
#[doc = "Receive address select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxaddresses(pub u32);
impl Rxaddresses {
    #[doc = "Enable or disable reception on logical address 0."]
    #[inline(always)]
    pub const fn addr0(&self) -> super::vals::Addr0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Addr0(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 0."]
    #[inline(always)]
    pub fn set_addr0(&mut self, val: super::vals::Addr0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable or disable reception on logical address 1."]
    #[inline(always)]
    pub const fn addr1(&self) -> super::vals::Addr1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Addr1(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 1."]
    #[inline(always)]
    pub fn set_addr1(&mut self, val: super::vals::Addr1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable or disable reception on logical address 2."]
    #[inline(always)]
    pub const fn addr2(&self) -> super::vals::Addr2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Addr2(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 2."]
    #[inline(always)]
    pub fn set_addr2(&mut self, val: super::vals::Addr2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable or disable reception on logical address 3."]
    #[inline(always)]
    pub const fn addr3(&self) -> super::vals::Addr3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Addr3(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 3."]
    #[inline(always)]
    pub fn set_addr3(&mut self, val: super::vals::Addr3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable or disable reception on logical address 4."]
    #[inline(always)]
    pub const fn addr4(&self) -> super::vals::Addr4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Addr4(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 4."]
    #[inline(always)]
    pub fn set_addr4(&mut self, val: super::vals::Addr4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable or disable reception on logical address 5."]
    #[inline(always)]
    pub const fn addr5(&self) -> super::vals::Addr5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Addr5(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 5."]
    #[inline(always)]
    pub fn set_addr5(&mut self, val: super::vals::Addr5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable or disable reception on logical address 6."]
    #[inline(always)]
    pub const fn addr6(&self) -> super::vals::Addr6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Addr6(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 6."]
    #[inline(always)]
    pub fn set_addr6(&mut self, val: super::vals::Addr6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.0 as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable or disable reception on logical address 7."]
    #[inline(always)]
    pub const fn addr7(&self) -> super::vals::Addr7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Addr7(val as u8)
    }
    #[doc = "Enable or disable reception on logical address 7."]
    #[inline(always)]
    pub fn set_addr7(&mut self, val: super::vals::Addr7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
    }
}
impl Default for Rxaddresses {
    #[inline(always)]
    fn default() -> Rxaddresses {
        Rxaddresses(0)
    }
}
#[doc = "CRC initial value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcinit(pub u32);
impl Crcinit {
    #[doc = "CRC initial value"]
    #[inline(always)]
    pub const fn crcinit(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "CRC initial value"]
    #[inline(always)]
    pub fn set_crcinit(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crcinit {
    #[inline(always)]
    fn default() -> Crcinit {
        Crcinit(0)
    }
}
#[doc = "Description collection\\[n\\]: Device address prefix n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dap(pub u32);
impl Dap {
    #[doc = "Device address prefix n"]
    #[inline(always)]
    pub const fn dap(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Device address prefix n"]
    #[inline(always)]
    pub fn set_dap(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dap {
    #[inline(always)]
    fn default() -> Dap {
        Dap(0)
    }
}
#[doc = "IEEE 802.15.4 clear channel assessment control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccactrl(pub u32);
impl Ccactrl {
    #[doc = "CCA mode of operation"]
    #[inline(always)]
    pub const fn ccamode(&self) -> super::vals::Ccamode {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ccamode(val as u8)
    }
    #[doc = "CCA mode of operation"]
    #[inline(always)]
    pub fn set_ccamode(&mut self, val: super::vals::Ccamode) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.0 as u32) & 0x07) << 0usize);
    }
    #[doc = "CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
    #[inline(always)]
    pub const fn ccaedthres(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "CCA energy busy threshold. Used in all the CCA modes except CarrierMode."]
    #[inline(always)]
    pub fn set_ccaedthres(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode and CarrierOrEdMode."]
    #[inline(always)]
    pub const fn ccacorrthres(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "CCA correlator busy threshold. Only relevant to CarrierMode, CarrierAndEdMode and CarrierOrEdMode."]
    #[inline(always)]
    pub fn set_ccacorrthres(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
    #[inline(always)]
    pub const fn ccacorrcnt(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Limit for occurances above CCACORRTHRES. When not equal to zero the corrolator based signal detect is enabled."]
    #[inline(always)]
    pub fn set_ccacorrcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ccactrl {
    #[inline(always)]
    fn default() -> Ccactrl {
        Ccactrl(0)
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
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for READY event"]
    #[inline(always)]
    pub fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write '1' to disable interrupt for ADDRESS event"]
    #[inline(always)]
    pub const fn address(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for ADDRESS event"]
    #[inline(always)]
    pub fn set_address(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write '1' to disable interrupt for PAYLOAD event"]
    #[inline(always)]
    pub const fn payload(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for PAYLOAD event"]
    #[inline(always)]
    pub fn set_payload(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to disable interrupt for END event"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for END event"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write '1' to disable interrupt for DISABLED event"]
    #[inline(always)]
    pub const fn disabled(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for DISABLED event"]
    #[inline(always)]
    pub fn set_disabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write '1' to disable interrupt for DEVMATCH event"]
    #[inline(always)]
    pub const fn devmatch(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for DEVMATCH event"]
    #[inline(always)]
    pub fn set_devmatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write '1' to disable interrupt for DEVMISS event"]
    #[inline(always)]
    pub const fn devmiss(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for DEVMISS event"]
    #[inline(always)]
    pub fn set_devmiss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write '1' to disable interrupt for RSSIEND event"]
    #[inline(always)]
    pub const fn rssiend(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for RSSIEND event"]
    #[inline(always)]
    pub fn set_rssiend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write '1' to disable interrupt for BCMATCH event"]
    #[inline(always)]
    pub const fn bcmatch(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for BCMATCH event"]
    #[inline(always)]
    pub fn set_bcmatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write '1' to disable interrupt for CRCOK event"]
    #[inline(always)]
    pub const fn crcok(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for CRCOK event"]
    #[inline(always)]
    pub fn set_crcok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Write '1' to disable interrupt for CRCERROR event"]
    #[inline(always)]
    pub const fn crcerror(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for CRCERROR event"]
    #[inline(always)]
    pub fn set_crcerror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Write '1' to disable interrupt for FRAMESTART event"]
    #[inline(always)]
    pub const fn framestart(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for FRAMESTART event"]
    #[inline(always)]
    pub fn set_framestart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Write '1' to disable interrupt for EDEND event"]
    #[inline(always)]
    pub const fn edend(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for EDEND event"]
    #[inline(always)]
    pub fn set_edend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Write '1' to disable interrupt for EDSTOPPED event"]
    #[inline(always)]
    pub const fn edstopped(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for EDSTOPPED event"]
    #[inline(always)]
    pub fn set_edstopped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Write '1' to disable interrupt for CCAIDLE event"]
    #[inline(always)]
    pub const fn ccaidle(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for CCAIDLE event"]
    #[inline(always)]
    pub fn set_ccaidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Write '1' to disable interrupt for CCABUSY event"]
    #[inline(always)]
    pub const fn ccabusy(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for CCABUSY event"]
    #[inline(always)]
    pub fn set_ccabusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write '1' to disable interrupt for CCASTOPPED event"]
    #[inline(always)]
    pub const fn ccastopped(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for CCASTOPPED event"]
    #[inline(always)]
    pub fn set_ccastopped(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Write '1' to disable interrupt for RATEBOOST event"]
    #[inline(always)]
    pub const fn rateboost(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for RATEBOOST event"]
    #[inline(always)]
    pub fn set_rateboost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Write '1' to disable interrupt for TXREADY event"]
    #[inline(always)]
    pub const fn txready(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for TXREADY event"]
    #[inline(always)]
    pub fn set_txready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Write '1' to disable interrupt for RXREADY event"]
    #[inline(always)]
    pub const fn rxready(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for RXREADY event"]
    #[inline(always)]
    pub fn set_rxready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Write '1' to disable interrupt for MHRMATCH event"]
    #[inline(always)]
    pub const fn mhrmatch(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for MHRMATCH event"]
    #[inline(always)]
    pub fn set_mhrmatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Write '1' to disable interrupt for PHYEND event"]
    #[inline(always)]
    pub const fn phyend(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for PHYEND event"]
    #[inline(always)]
    pub fn set_phyend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Peripheral power control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Power(pub u32);
impl Power {
    #[doc = "Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
    #[inline(always)]
    pub const fn power(&self) -> super::vals::Power {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Power(val as u8)
    }
    #[doc = "Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
    #[inline(always)]
    pub fn set_power(&mut self, val: super::vals::Power) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Power {
    #[inline(always)]
    fn default() -> Power {
        Power(0)
    }
}
#[doc = "Output power"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txpower(pub u32);
impl Txpower {
    #[doc = "RADIO output power"]
    #[inline(always)]
    pub const fn txpower(&self) -> super::vals::Txpower {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Txpower(val as u8)
    }
    #[doc = "RADIO output power"]
    #[inline(always)]
    pub fn set_txpower(&mut self, val: super::vals::Txpower) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.0 as u32) & 0xff) << 0usize);
    }
}
impl Default for Txpower {
    #[inline(always)]
    fn default() -> Txpower {
        Txpower(0)
    }
}
#[doc = "Payload status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdustat(pub u32);
impl Pdustat {
    #[doc = "Status on payload length vs. PCNF1.MAXLEN"]
    #[inline(always)]
    pub const fn pdustat(&self) -> super::vals::Pdustat {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdustat(val as u8)
    }
    #[doc = "Status on payload length vs. PCNF1.MAXLEN"]
    #[inline(always)]
    pub fn set_pdustat(&mut self, val: super::vals::Pdustat) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Status on what rate packet is received with in Long Range"]
    #[inline(always)]
    pub const fn cistat(&self) -> super::vals::Cistat {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Cistat(val as u8)
    }
    #[doc = "Status on what rate packet is received with in Long Range"]
    #[inline(always)]
    pub fn set_cistat(&mut self, val: super::vals::Cistat) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.0 as u32) & 0x03) << 1usize);
    }
}
impl Default for Pdustat {
    #[inline(always)]
    fn default() -> Pdustat {
        Pdustat(0)
    }
}
#[doc = "Device address match configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacnf(pub u32);
impl Dacnf {
    #[doc = "Enable or disable device address matching using device address 0"]
    #[inline(always)]
    pub const fn ena0(&self) -> super::vals::Ena0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ena0(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 0"]
    #[inline(always)]
    pub fn set_ena0(&mut self, val: super::vals::Ena0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable or disable device address matching using device address 1"]
    #[inline(always)]
    pub const fn ena1(&self) -> super::vals::Ena1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ena1(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 1"]
    #[inline(always)]
    pub fn set_ena1(&mut self, val: super::vals::Ena1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable or disable device address matching using device address 2"]
    #[inline(always)]
    pub const fn ena2(&self) -> super::vals::Ena2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ena2(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 2"]
    #[inline(always)]
    pub fn set_ena2(&mut self, val: super::vals::Ena2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable or disable device address matching using device address 3"]
    #[inline(always)]
    pub const fn ena3(&self) -> super::vals::Ena3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ena3(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 3"]
    #[inline(always)]
    pub fn set_ena3(&mut self, val: super::vals::Ena3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable or disable device address matching using device address 4"]
    #[inline(always)]
    pub const fn ena4(&self) -> super::vals::Ena4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ena4(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 4"]
    #[inline(always)]
    pub fn set_ena4(&mut self, val: super::vals::Ena4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.0 as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable or disable device address matching using device address 5"]
    #[inline(always)]
    pub const fn ena5(&self) -> super::vals::Ena5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ena5(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 5"]
    #[inline(always)]
    pub fn set_ena5(&mut self, val: super::vals::Ena5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.0 as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable or disable device address matching using device address 6"]
    #[inline(always)]
    pub const fn ena6(&self) -> super::vals::Ena6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ena6(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 6"]
    #[inline(always)]
    pub fn set_ena6(&mut self, val: super::vals::Ena6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.0 as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable or disable device address matching using device address 7"]
    #[inline(always)]
    pub const fn ena7(&self) -> super::vals::Ena7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ena7(val as u8)
    }
    #[doc = "Enable or disable device address matching using device address 7"]
    #[inline(always)]
    pub fn set_ena7(&mut self, val: super::vals::Ena7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.0 as u32) & 0x01) << 7usize);
    }
    #[doc = "TxAdd for device address 0"]
    #[inline(always)]
    pub const fn txadd0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 0"]
    #[inline(always)]
    pub fn set_txadd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "TxAdd for device address 1"]
    #[inline(always)]
    pub const fn txadd1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 1"]
    #[inline(always)]
    pub fn set_txadd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "TxAdd for device address 2"]
    #[inline(always)]
    pub const fn txadd2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 2"]
    #[inline(always)]
    pub fn set_txadd2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "TxAdd for device address 3"]
    #[inline(always)]
    pub const fn txadd3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 3"]
    #[inline(always)]
    pub fn set_txadd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TxAdd for device address 4"]
    #[inline(always)]
    pub const fn txadd4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 4"]
    #[inline(always)]
    pub fn set_txadd4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TxAdd for device address 5"]
    #[inline(always)]
    pub const fn txadd5(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 5"]
    #[inline(always)]
    pub fn set_txadd5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "TxAdd for device address 6"]
    #[inline(always)]
    pub const fn txadd6(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 6"]
    #[inline(always)]
    pub fn set_txadd6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "TxAdd for device address 7"]
    #[inline(always)]
    pub const fn txadd7(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "TxAdd for device address 7"]
    #[inline(always)]
    pub fn set_txadd7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Dacnf {
    #[inline(always)]
    fn default() -> Dacnf {
        Dacnf(0)
    }
}
#[doc = "IEEE 802.15.4 start of frame delimiter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfd(pub u32);
impl Sfd {
    #[doc = "IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub const fn sfd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub fn set_sfd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Sfd {
    #[inline(always)]
    fn default() -> Sfd {
        Sfd(0)
    }
}
