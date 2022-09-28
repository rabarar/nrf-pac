#[doc = "Device info"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Info(pub *mut u8);
unsafe impl Send for Info {}
unsafe impl Sync for Info {}
impl Info {
    #[doc = "Part code"]
    #[inline(always)]
    pub fn part(self) -> crate::common::Reg<regs::Part, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Build code (hardware version and production configuration)"]
    #[inline(always)]
    pub fn variant(self) -> crate::common::Reg<regs::Variant, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Package option"]
    #[inline(always)]
    pub fn package(self) -> crate::common::Reg<regs::Package, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "RAM variant"]
    #[inline(always)]
    pub fn ram(self) -> crate::common::Reg<regs::Ram, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Flash variant"]
    #[inline(always)]
    pub fn flash(self) -> crate::common::Reg<regs::Flash, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn unused8(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize + n * 4usize)) }
    }
}
#[doc = "Registers storing factory TEMP module linearization coefficients"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Temp(pub *mut u8);
unsafe impl Send for Temp {}
unsafe impl Sync for Temp {}
impl Temp {
    #[doc = "Slope definition A0"]
    #[inline(always)]
    pub fn a0(self) -> crate::common::Reg<regs::A0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Slope definition A1"]
    #[inline(always)]
    pub fn a1(self) -> crate::common::Reg<regs::A1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Slope definition A2"]
    #[inline(always)]
    pub fn a2(self) -> crate::common::Reg<regs::A2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Slope definition A3"]
    #[inline(always)]
    pub fn a3(self) -> crate::common::Reg<regs::A3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Slope definition A4"]
    #[inline(always)]
    pub fn a4(self) -> crate::common::Reg<regs::A4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Slope definition A5"]
    #[inline(always)]
    pub fn a5(self) -> crate::common::Reg<regs::A5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Y-intercept B0"]
    #[inline(always)]
    pub fn b0(self) -> crate::common::Reg<regs::B0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Y-intercept B1"]
    #[inline(always)]
    pub fn b1(self) -> crate::common::Reg<regs::B1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Y-intercept B2"]
    #[inline(always)]
    pub fn b2(self) -> crate::common::Reg<regs::B2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Y-intercept B3"]
    #[inline(always)]
    pub fn b3(self) -> crate::common::Reg<regs::B3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Y-intercept B4"]
    #[inline(always)]
    pub fn b4(self) -> crate::common::Reg<regs::B4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Y-intercept B5"]
    #[inline(always)]
    pub fn b5(self) -> crate::common::Reg<regs::B5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Segment end T0"]
    #[inline(always)]
    pub fn t0(self) -> crate::common::Reg<regs::T0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Segment end T1"]
    #[inline(always)]
    pub fn t1(self) -> crate::common::Reg<regs::T1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }
    #[doc = "Segment end T2"]
    #[inline(always)]
    pub fn t2(self) -> crate::common::Reg<regs::T2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "Segment end T3"]
    #[inline(always)]
    pub fn t3(self) -> crate::common::Reg<regs::T3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }
    #[doc = "Segment end T4"]
    #[inline(always)]
    pub fn t4(self) -> crate::common::Reg<regs::T4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
}
#[doc = "Factory information configuration registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ficr(pub *mut u8);
unsafe impl Send for Ficr {}
unsafe impl Sync for Ficr {}
impl Ficr {
    #[doc = "Code memory page size"]
    #[inline(always)]
    pub fn codepagesize(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Code memory size"]
    #[inline(always)]
    pub fn codesize(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Description collection\\[n\\]: Device identifier"]
    #[inline(always)]
    pub fn deviceid(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize + n * 4usize)) }
    }
    #[doc = "Description collection\\[n\\]: Encryption root, word n"]
    #[inline(always)]
    pub fn er(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize + n * 4usize)) }
    }
    #[doc = "Description collection\\[n\\]: Identity Root, word n"]
    #[inline(always)]
    pub fn ir(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize + n * 4usize)) }
    }
    #[doc = "Device address type"]
    #[inline(always)]
    pub fn deviceaddrtype(self) -> crate::common::Reg<regs::Deviceaddrtype, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }
    #[doc = "Description collection\\[n\\]: Device address n"]
    #[inline(always)]
    pub fn deviceaddr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize + n * 4usize)) }
    }
    #[doc = "Device info"]
    #[inline(always)]
    pub fn info(self) -> Info {
        unsafe { Info(self.0.add(256usize)) }
    }
    #[doc = "Description collection\\[n\\]: Production test signature n"]
    #[inline(always)]
    pub fn prodtest(self, n: usize) -> crate::common::Reg<regs::Prodtest, crate::common::R> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(848usize + n * 4usize)) }
    }
    #[doc = "Registers storing factory TEMP module linearization coefficients"]
    #[inline(always)]
    pub fn temp(self) -> Temp {
        unsafe { Temp(self.0.add(1028usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn nfc(self) -> Nfc {
        unsafe { Nfc(self.0.add(1104usize)) }
    }
    #[doc = "NIST800-90B RNG calibration data"]
    #[inline(always)]
    pub fn trng90b(self) -> Trng90b {
        unsafe { Trng90b(self.0.add(3072usize)) }
    }
}
#[doc = "NIST800-90B RNG calibration data"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trng90b(pub *mut u8);
unsafe impl Send for Trng90b {}
unsafe impl Sync for Trng90b {}
impl Trng90b {
    #[doc = "Amount of bytes for the required entropy bits"]
    #[inline(always)]
    pub fn bytes(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Repetition counter cutoff"]
    #[inline(always)]
    pub fn rccutoff(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Adaptive proportion cutoff"]
    #[inline(always)]
    pub fn apcutoff(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Amount of bytes for the startup tests"]
    #[inline(always)]
    pub fn startup(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Sample count for ring oscillator 1"]
    #[inline(always)]
    pub fn rosc1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Sample count for ring oscillator 2"]
    #[inline(always)]
    pub fn rosc2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Sample count for ring oscillator 3"]
    #[inline(always)]
    pub fn rosc3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Sample count for ring oscillator 4"]
    #[inline(always)]
    pub fn rosc4(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nfc(pub *mut u8);
unsafe impl Send for Nfc {}
unsafe impl Sync for Nfc {}
impl Nfc {
    #[doc = "Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    #[inline(always)]
    pub fn tagheader0(self) -> crate::common::Reg<regs::Tagheader0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    #[inline(always)]
    pub fn tagheader1(self) -> crate::common::Reg<regs::Tagheader1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    #[inline(always)]
    pub fn tagheader2(self) -> crate::common::Reg<regs::Tagheader2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    #[inline(always)]
    pub fn tagheader3(self) -> crate::common::Reg<regs::Tagheader3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
pub mod regs;
pub mod vals;
