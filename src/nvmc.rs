#[doc = "Non Volatile Memory Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvmc(pub *mut u8);
unsafe impl Send for Nvmc {}
unsafe impl Sync for Nvmc {}
impl Nvmc {
    #[doc = "Ready flag"]
    #[inline(always)]
    pub fn ready(self) -> crate::common::Reg<regs::Ready, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1024usize)) }
    }
    #[doc = "Ready flag"]
    #[inline(always)]
    pub fn readynext(self) -> crate::common::Reg<regs::Readynext, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1032usize)) }
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "Register for erasing a page in code area"]
    #[inline(always)]
    pub fn erasepage(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
    #[inline(always)]
    pub fn erasepcr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "Register for erasing all non-volatile user memory"]
    #[inline(always)]
    pub fn eraseall(self) -> crate::common::Reg<regs::Eraseall, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1292usize)) }
    }
    #[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
    #[inline(always)]
    pub fn erasepcr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize)) }
    }
    #[doc = "Register for erasing user information configuration registers"]
    #[inline(always)]
    pub fn eraseuicr(self) -> crate::common::Reg<regs::Eraseuicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1300usize)) }
    }
    #[doc = "Register for partial erase of a page in code area"]
    #[inline(always)]
    pub fn erasepagepartial(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1304usize)) }
    }
    #[doc = "Register for partial erase configuration"]
    #[inline(always)]
    pub fn erasepagepartialcfg(
        self,
    ) -> crate::common::Reg<regs::Erasepagepartialcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1308usize)) }
    }
    #[doc = "I-code cache configuration register."]
    #[inline(always)]
    pub fn icachecnf(self) -> crate::common::Reg<regs::Icachecnf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1344usize)) }
    }
    #[doc = "I-code cache hit counter."]
    #[inline(always)]
    pub fn ihit(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1352usize)) }
    }
    #[doc = "I-code cache miss counter."]
    #[inline(always)]
    pub fn imiss(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1356usize)) }
    }
}
pub mod regs;
pub mod vals;
