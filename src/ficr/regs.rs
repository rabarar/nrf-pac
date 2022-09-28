#[doc = "Y-intercept B5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B5(pub u32);
impl B5 {
    #[doc = "B (y-intercept)"]
    #[inline(always)]
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "B (y-intercept)"]
    #[inline(always)]
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for B5 {
    #[inline(always)]
    fn default() -> B5 {
        B5(0)
    }
}
#[doc = "Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tagheader1(pub u32);
impl Tagheader1 {
    #[doc = "Unique identifier byte 4"]
    #[inline(always)]
    pub const fn ud4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 4"]
    #[inline(always)]
    pub fn set_ud4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Unique identifier byte 5"]
    #[inline(always)]
    pub const fn ud5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 5"]
    #[inline(always)]
    pub fn set_ud5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Unique identifier byte 6"]
    #[inline(always)]
    pub const fn ud6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 6"]
    #[inline(always)]
    pub fn set_ud6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Unique identifier byte 7"]
    #[inline(always)]
    pub const fn ud7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 7"]
    #[inline(always)]
    pub fn set_ud7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Tagheader1 {
    #[inline(always)]
    fn default() -> Tagheader1 {
        Tagheader1(0)
    }
}
#[doc = "Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tagheader0(pub u32);
impl Tagheader0 {
    #[doc = "Default Manufacturer ID: Nordic Semiconductor ASA has ICM 0x5F"]
    #[inline(always)]
    pub const fn mfgid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Default Manufacturer ID: Nordic Semiconductor ASA has ICM 0x5F"]
    #[inline(always)]
    pub fn set_mfgid(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Unique identifier byte 1"]
    #[inline(always)]
    pub const fn ud1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 1"]
    #[inline(always)]
    pub fn set_ud1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Unique identifier byte 2"]
    #[inline(always)]
    pub const fn ud2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 2"]
    #[inline(always)]
    pub fn set_ud2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Unique identifier byte 3"]
    #[inline(always)]
    pub const fn ud3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 3"]
    #[inline(always)]
    pub fn set_ud3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Tagheader0 {
    #[inline(always)]
    fn default() -> Tagheader0 {
        Tagheader0(0)
    }
}
#[doc = "Y-intercept B2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B2(pub u32);
impl B2 {
    #[doc = "B (y-intercept)"]
    #[inline(always)]
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "B (y-intercept)"]
    #[inline(always)]
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for B2 {
    #[inline(always)]
    fn default() -> B2 {
        B2(0)
    }
}
#[doc = "Y-intercept B1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B1(pub u32);
impl B1 {
    #[doc = "B (y-intercept)"]
    #[inline(always)]
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "B (y-intercept)"]
    #[inline(always)]
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for B1 {
    #[inline(always)]
    fn default() -> B1 {
        B1(0)
    }
}
#[doc = "Slope definition A2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A2(pub u32);
impl A2 {
    #[doc = "A (slope definition) register."]
    #[inline(always)]
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "A (slope definition) register."]
    #[inline(always)]
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A2 {
    #[inline(always)]
    fn default() -> A2 {
        A2(0)
    }
}
#[doc = "Slope definition A1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A1(pub u32);
impl A1 {
    #[doc = "A (slope definition) register."]
    #[inline(always)]
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "A (slope definition) register."]
    #[inline(always)]
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A1 {
    #[inline(always)]
    fn default() -> A1 {
        A1(0)
    }
}
#[doc = "Slope definition A0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A0(pub u32);
impl A0 {
    #[doc = "A (slope definition) register."]
    #[inline(always)]
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "A (slope definition) register."]
    #[inline(always)]
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A0 {
    #[inline(always)]
    fn default() -> A0 {
        A0(0)
    }
}
#[doc = "Segment end T1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T1(pub u32);
impl T1 {
    #[doc = "T (segment end) register"]
    #[inline(always)]
    pub const fn t(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "T (segment end) register"]
    #[inline(always)]
    pub fn set_t(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T1 {
    #[inline(always)]
    fn default() -> T1 {
        T1(0)
    }
}
#[doc = "Y-intercept B4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B4(pub u32);
impl B4 {
    #[doc = "B (y-intercept)"]
    #[inline(always)]
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "B (y-intercept)"]
    #[inline(always)]
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for B4 {
    #[inline(always)]
    fn default() -> B4 {
        B4(0)
    }
}
#[doc = "Package option"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Package(pub u32);
impl Package {
    #[doc = "Package option"]
    #[inline(always)]
    pub const fn package(&self) -> super::vals::Package {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Package(val as u32)
    }
    #[doc = "Package option"]
    #[inline(always)]
    pub fn set_package(&mut self, val: super::vals::Package) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.0 as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Package {
    #[inline(always)]
    fn default() -> Package {
        Package(0)
    }
}
#[doc = "Description collection\\[n\\]: Production test signature n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prodtest(pub u32);
impl Prodtest {
    #[doc = "Production test signature n"]
    #[inline(always)]
    pub const fn prodtest(&self) -> super::vals::Prodtest {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Prodtest(val as u32)
    }
    #[doc = "Production test signature n"]
    #[inline(always)]
    pub fn set_prodtest(&mut self, val: super::vals::Prodtest) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.0 as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Prodtest {
    #[inline(always)]
    fn default() -> Prodtest {
        Prodtest(0)
    }
}
#[doc = "Slope definition A3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A3(pub u32);
impl A3 {
    #[doc = "A (slope definition) register."]
    #[inline(always)]
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "A (slope definition) register."]
    #[inline(always)]
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A3 {
    #[inline(always)]
    fn default() -> A3 {
        A3(0)
    }
}
#[doc = "Y-intercept B0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0(pub u32);
impl B0 {
    #[doc = "B (y-intercept)"]
    #[inline(always)]
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "B (y-intercept)"]
    #[inline(always)]
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for B0 {
    #[inline(always)]
    fn default() -> B0 {
        B0(0)
    }
}
#[doc = "Slope definition A4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A4(pub u32);
impl A4 {
    #[doc = "A (slope definition) register."]
    #[inline(always)]
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "A (slope definition) register."]
    #[inline(always)]
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A4 {
    #[inline(always)]
    fn default() -> A4 {
        A4(0)
    }
}
#[doc = "Build code (hardware version and production configuration)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Variant(pub u32);
impl Variant {
    #[doc = "Build code (hardware version and production configuration). Encoded as ASCII."]
    #[inline(always)]
    pub const fn variant(&self) -> super::vals::Variant {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Variant(val as u32)
    }
    #[doc = "Build code (hardware version and production configuration). Encoded as ASCII."]
    #[inline(always)]
    pub fn set_variant(&mut self, val: super::vals::Variant) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.0 as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Variant {
    #[inline(always)]
    fn default() -> Variant {
        Variant(0)
    }
}
#[doc = "Segment end T3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T3(pub u32);
impl T3 {
    #[doc = "T (segment end) register"]
    #[inline(always)]
    pub const fn t(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "T (segment end) register"]
    #[inline(always)]
    pub fn set_t(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T3 {
    #[inline(always)]
    fn default() -> T3 {
        T3(0)
    }
}
#[doc = "Y-intercept B3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B3(pub u32);
impl B3 {
    #[doc = "B (y-intercept)"]
    #[inline(always)]
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "B (y-intercept)"]
    #[inline(always)]
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for B3 {
    #[inline(always)]
    fn default() -> B3 {
        B3(0)
    }
}
#[doc = "Part code"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Part(pub u32);
impl Part {
    #[doc = "Part code"]
    #[inline(always)]
    pub const fn part(&self) -> super::vals::Part {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Part(val as u32)
    }
    #[doc = "Part code"]
    #[inline(always)]
    pub fn set_part(&mut self, val: super::vals::Part) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.0 as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Part {
    #[inline(always)]
    fn default() -> Part {
        Part(0)
    }
}
#[doc = "Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tagheader3(pub u32);
impl Tagheader3 {
    #[doc = "Unique identifier byte 12"]
    #[inline(always)]
    pub const fn ud12(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 12"]
    #[inline(always)]
    pub fn set_ud12(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Unique identifier byte 13"]
    #[inline(always)]
    pub const fn ud13(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 13"]
    #[inline(always)]
    pub fn set_ud13(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Unique identifier byte 14"]
    #[inline(always)]
    pub const fn ud14(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 14"]
    #[inline(always)]
    pub fn set_ud14(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Unique identifier byte 15"]
    #[inline(always)]
    pub const fn ud15(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 15"]
    #[inline(always)]
    pub fn set_ud15(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Tagheader3 {
    #[inline(always)]
    fn default() -> Tagheader3 {
        Tagheader3(0)
    }
}
#[doc = "Segment end T2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T2(pub u32);
impl T2 {
    #[doc = "T (segment end) register"]
    #[inline(always)]
    pub const fn t(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "T (segment end) register"]
    #[inline(always)]
    pub fn set_t(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T2 {
    #[inline(always)]
    fn default() -> T2 {
        T2(0)
    }
}
#[doc = "Segment end T4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T4(pub u32);
impl T4 {
    #[doc = "T (segment end) register"]
    #[inline(always)]
    pub const fn t(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "T (segment end) register"]
    #[inline(always)]
    pub fn set_t(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T4 {
    #[inline(always)]
    fn default() -> T4 {
        T4(0)
    }
}
#[doc = "Segment end T0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T0(pub u32);
impl T0 {
    #[doc = "T (segment end) register"]
    #[inline(always)]
    pub const fn t(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "T (segment end) register"]
    #[inline(always)]
    pub fn set_t(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T0 {
    #[inline(always)]
    fn default() -> T0 {
        T0(0)
    }
}
#[doc = "RAM variant"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram(pub u32);
impl Ram {
    #[doc = "RAM variant"]
    #[inline(always)]
    pub const fn ram(&self) -> super::vals::Ram {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Ram(val as u32)
    }
    #[doc = "RAM variant"]
    #[inline(always)]
    pub fn set_ram(&mut self, val: super::vals::Ram) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.0 as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ram {
    #[inline(always)]
    fn default() -> Ram {
        Ram(0)
    }
}
#[doc = "Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tagheader2(pub u32);
impl Tagheader2 {
    #[doc = "Unique identifier byte 8"]
    #[inline(always)]
    pub const fn ud8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 8"]
    #[inline(always)]
    pub fn set_ud8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Unique identifier byte 9"]
    #[inline(always)]
    pub const fn ud9(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 9"]
    #[inline(always)]
    pub fn set_ud9(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Unique identifier byte 10"]
    #[inline(always)]
    pub const fn ud10(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 10"]
    #[inline(always)]
    pub fn set_ud10(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Unique identifier byte 11"]
    #[inline(always)]
    pub const fn ud11(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Unique identifier byte 11"]
    #[inline(always)]
    pub fn set_ud11(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Tagheader2 {
    #[inline(always)]
    fn default() -> Tagheader2 {
        Tagheader2(0)
    }
}
#[doc = "Device address type"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Deviceaddrtype(pub u32);
impl Deviceaddrtype {
    #[doc = "Device address type"]
    #[inline(always)]
    pub const fn deviceaddrtype(&self) -> super::vals::Deviceaddrtype {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Deviceaddrtype(val as u8)
    }
    #[doc = "Device address type"]
    #[inline(always)]
    pub fn set_deviceaddrtype(&mut self, val: super::vals::Deviceaddrtype) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.0 as u32) & 0x01) << 0usize);
    }
}
impl Default for Deviceaddrtype {
    #[inline(always)]
    fn default() -> Deviceaddrtype {
        Deviceaddrtype(0)
    }
}
#[doc = "Flash variant"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash(pub u32);
impl Flash {
    #[doc = "Flash variant"]
    #[inline(always)]
    pub const fn flash(&self) -> super::vals::Flash {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Flash(val as u32)
    }
    #[doc = "Flash variant"]
    #[inline(always)]
    pub fn set_flash(&mut self, val: super::vals::Flash) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.0 as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Flash {
    #[inline(always)]
    fn default() -> Flash {
        Flash(0)
    }
}
#[doc = "Slope definition A5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A5(pub u32);
impl A5 {
    #[doc = "A (slope definition) register."]
    #[inline(always)]
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "A (slope definition) register."]
    #[inline(always)]
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A5 {
    #[inline(always)]
    fn default() -> A5 {
        A5(0)
    }
}
