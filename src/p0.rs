#[doc = "GPIO Port 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0(pub *mut u8);
unsafe impl Send for P0 {}
unsafe impl Sync for P0 {}
impl P0 {
    #[doc = "Write GPIO port"]
    #[inline(always)]
    pub fn out(self) -> crate::common::Reg<regs::Out, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "Set individual bits in GPIO port"]
    #[inline(always)]
    pub fn outset(self) -> crate::common::Reg<regs::Outset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "Clear individual bits in GPIO port"]
    #[inline(always)]
    pub fn outclr(self) -> crate::common::Reg<regs::Outclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1292usize)) }
    }
    #[doc = "Read GPIO port"]
    #[inline(always)]
    pub fn in_(self) -> crate::common::Reg<regs::In, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize)) }
    }
    #[doc = "Direction of GPIO pins"]
    #[inline(always)]
    pub fn dir(self) -> crate::common::Reg<regs::Dir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1300usize)) }
    }
    #[doc = "DIR set register"]
    #[inline(always)]
    pub fn dirset(self) -> crate::common::Reg<regs::Dirset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1304usize)) }
    }
    #[doc = "DIR clear register"]
    #[inline(always)]
    pub fn dirclr(self) -> crate::common::Reg<regs::Dirclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1308usize)) }
    }
    #[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
    #[inline(always)]
    pub fn latch(self) -> crate::common::Reg<regs::Latch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1312usize)) }
    }
    #[doc = "Select between default DETECT signal behaviour and LDETECT mode"]
    #[inline(always)]
    pub fn detectmode(self) -> crate::common::Reg<regs::Detectmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1316usize)) }
    }
    #[doc = "Description collection\\[n\\]: Configuration of GPIO pins"]
    #[inline(always)]
    pub fn pin_cnf(self, n: usize) -> crate::common::Reg<regs::PinCnf, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(1792usize + n * 4usize)) }
    }
}
pub mod regs;
pub mod vals;
