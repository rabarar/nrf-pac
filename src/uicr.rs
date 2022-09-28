#[doc = "User information configuration registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uicr(pub *mut u8);
unsafe impl Send for Uicr {}
unsafe impl Sync for Uicr {}
impl Uicr {
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn unused0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn unused1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn unused2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn unused3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Description collection\\[n\\]: Reserved for Nordic firmware design"]
    #[inline(always)]
    pub fn nrffw(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 15usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize + n * 4usize)) }
    }
    #[doc = "Description collection\\[n\\]: Reserved for Nordic hardware design"]
    #[inline(always)]
    pub fn nrfhw(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize + n * 4usize)) }
    }
    #[doc = "Description collection\\[n\\]: Reserved for customer"]
    #[inline(always)]
    pub fn customer(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize + n * 4usize)) }
    }
    #[doc = "Description collection\\[n\\]: Mapping of the nRESET function"]
    #[inline(always)]
    pub fn pselreset(self, n: usize) -> crate::common::Reg<regs::Pselreset, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(512usize + n * 4usize)) }
    }
    #[doc = "Access port protection"]
    #[inline(always)]
    pub fn approtect(self) -> crate::common::Reg<regs::Approtect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(520usize)) }
    }
    #[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
    #[inline(always)]
    pub fn nfcpins(self) -> crate::common::Reg<regs::Nfcpins, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(524usize)) }
    }
    #[doc = "Processor debug control"]
    #[inline(always)]
    pub fn debugctrl(self) -> crate::common::Reg<regs::Debugctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(528usize)) }
    }
    #[doc = "GPIO reference voltage / external output supply voltage in high voltage mode"]
    #[inline(always)]
    pub fn regout0(self) -> crate::common::Reg<regs::Regout0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(772usize)) }
    }
}
pub mod regs;
pub mod vals;
