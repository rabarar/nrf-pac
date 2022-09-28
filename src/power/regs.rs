#[doc = "USB supply status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbregstatus(pub u32);
impl Usbregstatus {
    #[doc = "VBUS input detection status (USBDETECTED and USBREMOVED events are derived from this information)"]
    #[inline(always)]
    pub const fn vbusdetect(&self) -> super::vals::Vbusdetect {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Vbusdetect(val as u8)
    }
    #[doc = "VBUS input detection status (USBDETECTED and USBREMOVED events are derived from this information)"]
    #[inline(always)]
    pub fn set_vbusdetect(&mut self, val: super::vals::Vbusdetect) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "USB supply output settling time elapsed"]
    #[inline(always)]
    pub const fn outputrdy(&self) -> super::vals::Outputrdy {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Outputrdy(val as u8)
    }
    #[doc = "USB supply output settling time elapsed"]
    #[inline(always)]
    pub fn set_outputrdy(&mut self, val: super::vals::Outputrdy) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
    }
}
impl Default for Usbregstatus {
    #[inline(always)]
    fn default() -> Usbregstatus {
        Usbregstatus(0)
    }
}
#[doc = "Deprecated register - RAM status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ramstatus(pub u32);
impl Ramstatus {
    #[doc = "RAM block 0 is on or off/powering up"]
    #[inline(always)]
    pub const fn ramblock0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RAM block 0 is on or off/powering up"]
    #[inline(always)]
    pub fn set_ramblock0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RAM block 1 is on or off/powering up"]
    #[inline(always)]
    pub const fn ramblock1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RAM block 1 is on or off/powering up"]
    #[inline(always)]
    pub fn set_ramblock1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RAM block 2 is on or off/powering up"]
    #[inline(always)]
    pub const fn ramblock2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RAM block 2 is on or off/powering up"]
    #[inline(always)]
    pub fn set_ramblock2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RAM block 3 is on or off/powering up"]
    #[inline(always)]
    pub const fn ramblock3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RAM block 3 is on or off/powering up"]
    #[inline(always)]
    pub fn set_ramblock3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Ramstatus {
    #[inline(always)]
    fn default() -> Ramstatus {
        Ramstatus(0)
    }
}
#[doc = "General purpose retention register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpregret(pub u32);
impl Gpregret {
    #[doc = "General purpose retention register"]
    #[inline(always)]
    pub const fn gpregret(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "General purpose retention register"]
    #[inline(always)]
    pub fn set_gpregret(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Gpregret {
    #[inline(always)]
    fn default() -> Gpregret {
        Gpregret(0)
    }
}
#[doc = "General purpose retention register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpregret2(pub u32);
impl Gpregret2 {
    #[doc = "General purpose retention register"]
    #[inline(always)]
    pub const fn gpregret(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "General purpose retention register"]
    #[inline(always)]
    pub fn set_gpregret(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Gpregret2 {
    #[inline(always)]
    fn default() -> Gpregret2 {
        Gpregret2(0)
    }
}
#[doc = "Enable DC/DC converter for REG0 stage."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdcen0(pub u32);
impl Dcdcen0 {
    #[doc = "Enable DC/DC converter for REG0 stage."]
    #[inline(always)]
    pub const fn dcdcen(&self) -> super::vals::Dcdcen0dcdcen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dcdcen0dcdcen(val as u8)
    }
    #[doc = "Enable DC/DC converter for REG0 stage."]
    #[inline(always)]
    pub fn set_dcdcen(&mut self, val: super::vals::Dcdcen0dcdcen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Dcdcen0 {
    #[inline(always)]
    fn default() -> Dcdcen0 {
        Dcdcen0(0)
    }
}
#[doc = "Power-fail comparator configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pofcon(pub u32);
impl Pofcon {
    #[doc = "Enable or disable power failure warning"]
    #[inline(always)]
    pub const fn pof(&self) -> super::vals::Pof {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pof(val as u8)
    }
    #[doc = "Enable or disable power failure warning"]
    #[inline(always)]
    pub fn set_pof(&mut self, val: super::vals::Pof) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Power-fail comparator threshold setting. This setting applies both for normal voltage mode (supply connected to both VDD and VDDH) and high voltage mode (supply connected to VDDH only). Values 0-3 set threshold below 1.7 V and should not be used as brown out detection will be activated before power failure warning on such low voltages."]
    #[inline(always)]
    pub const fn threshold(&self) -> super::vals::Threshold {
        let val = (self.0 >> 1usize) & 0x0f;
        super::vals::Threshold(val as u8)
    }
    #[doc = "Power-fail comparator threshold setting. This setting applies both for normal voltage mode (supply connected to both VDD and VDDH) and high voltage mode (supply connected to VDDH only). Values 0-3 set threshold below 1.7 V and should not be used as brown out detection will be activated before power failure warning on such low voltages."]
    #[inline(always)]
    pub fn set_threshold(&mut self, val: super::vals::Threshold) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val.0 as u32) & 0x0f) << 1usize);
    }
    #[doc = "Power-fail comparator threshold setting for high voltage mode (supply connected to VDDH only). This setting does not apply for normal voltage mode (supply connected to both VDD and VDDH)."]
    #[inline(always)]
    pub const fn thresholdvddh(&self) -> super::vals::Thresholdvddh {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Thresholdvddh(val as u8)
    }
    #[doc = "Power-fail comparator threshold setting for high voltage mode (supply connected to VDDH only). This setting does not apply for normal voltage mode (supply connected to both VDD and VDDH)."]
    #[inline(always)]
    pub fn set_thresholdvddh(&mut self, val: super::vals::Thresholdvddh) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.0 as u32) & 0x0f) << 8usize);
    }
}
impl Default for Pofcon {
    #[inline(always)]
    fn default() -> Pofcon {
        Pofcon(0)
    }
}
#[doc = "Main supply status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mainregstatus(pub u32);
impl Mainregstatus {
    #[doc = "Main supply status"]
    #[inline(always)]
    pub const fn mainregstatus(&self) -> super::vals::Mainregstatus {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mainregstatus(val as u8)
    }
    #[doc = "Main supply status"]
    #[inline(always)]
    pub fn set_mainregstatus(&mut self, val: super::vals::Mainregstatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Mainregstatus {
    #[inline(always)]
    fn default() -> Mainregstatus {
        Mainregstatus(0)
    }
}
#[doc = "Description cluster\\[n\\]: RAMn power control set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Powerset(pub u32);
impl Powerset {
    #[doc = "Keep RAM section S0 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s0power(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S0 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s0power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Keep RAM section S1 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s1power(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S1 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s1power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Keep RAM section S2 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s2power(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S2 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s2power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Keep RAM section S3 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s3power(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S3 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s3power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Keep RAM section S4 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s4power(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S4 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s4power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Keep RAM section S5 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s5power(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S5 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s5power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Keep RAM section S6 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s6power(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S6 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s6power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Keep RAM section S7 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s7power(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S7 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s7power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Keep RAM section S8 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s8power(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S8 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s8power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Keep RAM section S9 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s9power(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S9 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s9power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Keep RAM section S10 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s10power(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S10 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s10power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Keep RAM section S11 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s11power(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S11 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s11power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Keep RAM section S12 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s12power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S12 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s12power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Keep RAM section S13 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s13power(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S13 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s13power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Keep RAM section S14 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s14power(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S14 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s14power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Keep RAM section S15 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s15power(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S15 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s15power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Keep retention on RAM section S0 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s0retention(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S0 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s0retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Keep retention on RAM section S1 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s1retention(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S1 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s1retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Keep retention on RAM section S2 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s2retention(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S2 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s2retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Keep retention on RAM section S3 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s3retention(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S3 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s3retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Keep retention on RAM section S4 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s4retention(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S4 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s4retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Keep retention on RAM section S5 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s5retention(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S5 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s5retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Keep retention on RAM section S6 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s6retention(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S6 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s6retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Keep retention on RAM section S7 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s7retention(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S7 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s7retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Keep retention on RAM section S8 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s8retention(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S8 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s8retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Keep retention on RAM section S9 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s9retention(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S9 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s9retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Keep retention on RAM section S10 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s10retention(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S10 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s10retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Keep retention on RAM section S11 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s11retention(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S11 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s11retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Keep retention on RAM section S12 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s12retention(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S12 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s12retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Keep retention on RAM section S13 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s13retention(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S13 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s13retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Keep retention on RAM section S14 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s14retention(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S14 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s14retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Keep retention on RAM section S15 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s15retention(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S15 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s15retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Powerset {
    #[inline(always)]
    fn default() -> Powerset {
        Powerset(0)
    }
}
#[doc = "Description cluster\\[n\\]: RAMn power control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Power(pub u32);
impl Power {
    #[doc = "Keep RAM section S0 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s0power(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S0 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s0power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Keep RAM section S1 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s1power(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S1 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s1power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Keep RAM section S2 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s2power(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S2 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s2power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Keep RAM section S3 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s3power(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S3 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s3power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Keep RAM section S4 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s4power(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S4 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s4power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Keep RAM section S5 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s5power(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S5 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s5power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Keep RAM section S6 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s6power(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S6 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s6power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Keep RAM section S7 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s7power(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S7 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s7power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Keep RAM section S8 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s8power(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S8 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s8power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Keep RAM section S9 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s9power(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S9 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s9power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Keep RAM section S10 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s10power(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S10 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s10power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Keep RAM section S11 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s11power(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S11 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s11power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Keep RAM section S12 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s12power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S12 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s12power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Keep RAM section S13 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s13power(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S13 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s13power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Keep RAM section S14 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s14power(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S14 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s14power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Keep RAM section S15 on or off in System ON mode."]
    #[inline(always)]
    pub const fn s15power(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S15 on or off in System ON mode."]
    #[inline(always)]
    pub fn set_s15power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Keep retention on RAM section S0 when RAM section is off"]
    #[inline(always)]
    pub const fn s0retention(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S0 when RAM section is off"]
    #[inline(always)]
    pub fn set_s0retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Keep retention on RAM section S1 when RAM section is off"]
    #[inline(always)]
    pub const fn s1retention(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S1 when RAM section is off"]
    #[inline(always)]
    pub fn set_s1retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Keep retention on RAM section S2 when RAM section is off"]
    #[inline(always)]
    pub const fn s2retention(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S2 when RAM section is off"]
    #[inline(always)]
    pub fn set_s2retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Keep retention on RAM section S3 when RAM section is off"]
    #[inline(always)]
    pub const fn s3retention(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S3 when RAM section is off"]
    #[inline(always)]
    pub fn set_s3retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Keep retention on RAM section S4 when RAM section is off"]
    #[inline(always)]
    pub const fn s4retention(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S4 when RAM section is off"]
    #[inline(always)]
    pub fn set_s4retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Keep retention on RAM section S5 when RAM section is off"]
    #[inline(always)]
    pub const fn s5retention(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S5 when RAM section is off"]
    #[inline(always)]
    pub fn set_s5retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Keep retention on RAM section S6 when RAM section is off"]
    #[inline(always)]
    pub const fn s6retention(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S6 when RAM section is off"]
    #[inline(always)]
    pub fn set_s6retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Keep retention on RAM section S7 when RAM section is off"]
    #[inline(always)]
    pub const fn s7retention(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S7 when RAM section is off"]
    #[inline(always)]
    pub fn set_s7retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Keep retention on RAM section S8 when RAM section is off"]
    #[inline(always)]
    pub const fn s8retention(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S8 when RAM section is off"]
    #[inline(always)]
    pub fn set_s8retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Keep retention on RAM section S9 when RAM section is off"]
    #[inline(always)]
    pub const fn s9retention(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S9 when RAM section is off"]
    #[inline(always)]
    pub fn set_s9retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Keep retention on RAM section S10 when RAM section is off"]
    #[inline(always)]
    pub const fn s10retention(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S10 when RAM section is off"]
    #[inline(always)]
    pub fn set_s10retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Keep retention on RAM section S11 when RAM section is off"]
    #[inline(always)]
    pub const fn s11retention(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S11 when RAM section is off"]
    #[inline(always)]
    pub fn set_s11retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Keep retention on RAM section S12 when RAM section is off"]
    #[inline(always)]
    pub const fn s12retention(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S12 when RAM section is off"]
    #[inline(always)]
    pub fn set_s12retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Keep retention on RAM section S13 when RAM section is off"]
    #[inline(always)]
    pub const fn s13retention(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S13 when RAM section is off"]
    #[inline(always)]
    pub fn set_s13retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Keep retention on RAM section S14 when RAM section is off"]
    #[inline(always)]
    pub const fn s14retention(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S14 when RAM section is off"]
    #[inline(always)]
    pub fn set_s14retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Keep retention on RAM section S15 when RAM section is off"]
    #[inline(always)]
    pub const fn s15retention(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S15 when RAM section is off"]
    #[inline(always)]
    pub fn set_s15retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Power {
    #[inline(always)]
    fn default() -> Power {
        Power(0)
    }
}
#[doc = "Disable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to disable interrupt for POFWARN event"]
    #[inline(always)]
    pub const fn pofwarn(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for POFWARN event"]
    #[inline(always)]
    pub fn set_pofwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write '1' to disable interrupt for SLEEPENTER event"]
    #[inline(always)]
    pub const fn sleepenter(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for SLEEPENTER event"]
    #[inline(always)]
    pub fn set_sleepenter(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Write '1' to disable interrupt for SLEEPEXIT event"]
    #[inline(always)]
    pub const fn sleepexit(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for SLEEPEXIT event"]
    #[inline(always)]
    pub fn set_sleepexit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write '1' to disable interrupt for USBDETECTED event"]
    #[inline(always)]
    pub const fn usbdetected(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for USBDETECTED event"]
    #[inline(always)]
    pub fn set_usbdetected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write '1' to disable interrupt for USBREMOVED event"]
    #[inline(always)]
    pub const fn usbremoved(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for USBREMOVED event"]
    #[inline(always)]
    pub fn set_usbremoved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write '1' to disable interrupt for USBPWRRDY event"]
    #[inline(always)]
    pub const fn usbpwrrdy(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to disable interrupt for USBPWRRDY event"]
    #[inline(always)]
    pub fn set_usbpwrrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Description cluster\\[n\\]: RAMn power control clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Powerclr(pub u32);
impl Powerclr {
    #[doc = "Keep RAM section S0 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s0power(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S0 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s0power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Keep RAM section S1 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s1power(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S1 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s1power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Keep RAM section S2 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s2power(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S2 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s2power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Keep RAM section S3 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s3power(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S3 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s3power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Keep RAM section S4 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s4power(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S4 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s4power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Keep RAM section S5 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s5power(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S5 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s5power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Keep RAM section S6 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s6power(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S6 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s6power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Keep RAM section S7 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s7power(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S7 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s7power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Keep RAM section S8 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s8power(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S8 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s8power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Keep RAM section S9 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s9power(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S9 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s9power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Keep RAM section S10 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s10power(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S10 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s10power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Keep RAM section S11 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s11power(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S11 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s11power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Keep RAM section S12 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s12power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S12 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s12power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Keep RAM section S13 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s13power(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S13 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s13power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Keep RAM section S14 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s14power(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S14 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s14power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Keep RAM section S15 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub const fn s15power(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Keep RAM section S15 of RAMn on or off in System ON mode"]
    #[inline(always)]
    pub fn set_s15power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Keep retention on RAM section S0 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s0retention(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S0 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s0retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Keep retention on RAM section S1 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s1retention(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S1 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s1retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Keep retention on RAM section S2 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s2retention(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S2 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s2retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Keep retention on RAM section S3 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s3retention(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S3 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s3retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Keep retention on RAM section S4 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s4retention(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S4 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s4retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Keep retention on RAM section S5 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s5retention(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S5 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s5retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Keep retention on RAM section S6 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s6retention(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S6 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s6retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Keep retention on RAM section S7 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s7retention(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S7 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s7retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Keep retention on RAM section S8 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s8retention(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S8 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s8retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Keep retention on RAM section S9 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s9retention(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S9 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s9retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Keep retention on RAM section S10 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s10retention(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S10 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s10retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Keep retention on RAM section S11 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s11retention(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S11 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s11retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Keep retention on RAM section S12 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s12retention(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S12 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s12retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Keep retention on RAM section S13 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s13retention(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S13 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s13retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Keep retention on RAM section S14 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s14retention(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S14 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s14retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Keep retention on RAM section S15 when RAM section is switched off"]
    #[inline(always)]
    pub const fn s15retention(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Keep retention on RAM section S15 when RAM section is switched off"]
    #[inline(always)]
    pub fn set_s15retention(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Powerclr {
    #[inline(always)]
    fn default() -> Powerclr {
        Powerclr(0)
    }
}
#[doc = "Enable DC/DC converter for REG1 stage."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcdcen(pub u32);
impl Dcdcen {
    #[doc = "Enable DC/DC converter for REG1 stage."]
    #[inline(always)]
    pub const fn dcdcen(&self) -> super::vals::DcdcenDcdcen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DcdcenDcdcen(val as u8)
    }
    #[doc = "Enable DC/DC converter for REG1 stage."]
    #[inline(always)]
    pub fn set_dcdcen(&mut self, val: super::vals::DcdcenDcdcen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Dcdcen {
    #[inline(always)]
    fn default() -> Dcdcen {
        Dcdcen(0)
    }
}
#[doc = "Reset reason"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resetreas(pub u32);
impl Resetreas {
    #[doc = "Reset from pin-reset detected"]
    #[inline(always)]
    pub const fn resetpin(&self) -> super::vals::Resetpin {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Resetpin(val as u8)
    }
    #[doc = "Reset from pin-reset detected"]
    #[inline(always)]
    pub fn set_resetpin(&mut self, val: super::vals::Resetpin) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
    #[doc = "Reset from watchdog detected"]
    #[inline(always)]
    pub const fn dog(&self) -> super::vals::Dog {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dog(val as u8)
    }
    #[doc = "Reset from watchdog detected"]
    #[inline(always)]
    pub fn set_dog(&mut self, val: super::vals::Dog) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
    }
    #[doc = "Reset from soft reset detected"]
    #[inline(always)]
    pub const fn sreq(&self) -> super::vals::Sreq {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sreq(val as u8)
    }
    #[doc = "Reset from soft reset detected"]
    #[inline(always)]
    pub fn set_sreq(&mut self, val: super::vals::Sreq) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
    }
    #[doc = "Reset from CPU lock-up detected"]
    #[inline(always)]
    pub const fn lockup(&self) -> super::vals::Lockup {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lockup(val as u8)
    }
    #[doc = "Reset from CPU lock-up detected"]
    #[inline(always)]
    pub fn set_lockup(&mut self, val: super::vals::Lockup) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.0 as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
    #[inline(always)]
    pub const fn off(&self) -> super::vals::Off {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Off(val as u8)
    }
    #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
    #[inline(always)]
    pub fn set_off(&mut self, val: super::vals::Off) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.0 as u32) & 0x01) << 16usize);
    }
    #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub const fn lpcomp(&self) -> super::vals::Lpcomp {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Lpcomp(val as u8)
    }
    #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub fn set_lpcomp(&mut self, val: super::vals::Lpcomp) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.0 as u32) & 0x01) << 17usize);
    }
    #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
    #[inline(always)]
    pub const fn dif(&self) -> super::vals::Dif {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dif(val as u8)
    }
    #[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
    #[inline(always)]
    pub fn set_dif(&mut self, val: super::vals::Dif) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.0 as u32) & 0x01) << 18usize);
    }
    #[doc = "Reset due to wake up from System OFF mode by NFC field detect"]
    #[inline(always)]
    pub const fn nfc(&self) -> super::vals::Nfc {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Nfc(val as u8)
    }
    #[doc = "Reset due to wake up from System OFF mode by NFC field detect"]
    #[inline(always)]
    pub fn set_nfc(&mut self, val: super::vals::Nfc) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.0 as u32) & 0x01) << 19usize);
    }
    #[doc = "Reset due to wake up from System OFF mode by VBUS rising into valid range"]
    #[inline(always)]
    pub const fn vbus(&self) -> super::vals::Vbus {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Vbus(val as u8)
    }
    #[doc = "Reset due to wake up from System OFF mode by VBUS rising into valid range"]
    #[inline(always)]
    pub fn set_vbus(&mut self, val: super::vals::Vbus) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.0 as u32) & 0x01) << 20usize);
    }
}
impl Default for Resetreas {
    #[inline(always)]
    fn default() -> Resetreas {
        Resetreas(0)
    }
}
#[doc = "System OFF register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systemoff(pub u32);
impl Systemoff {
    #[doc = "Enable System OFF mode"]
    #[inline(always)]
    pub const fn systemoff(&self) -> super::vals::Systemoff {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Systemoff(val as u8)
    }
    #[doc = "Enable System OFF mode"]
    #[inline(always)]
    pub fn set_systemoff(&mut self, val: super::vals::Systemoff) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Systemoff {
    #[inline(always)]
    fn default() -> Systemoff {
        Systemoff(0)
    }
}
