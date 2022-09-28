#[doc = "Fork"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fork(pub *mut u8);
unsafe impl Send for Fork {}
unsafe impl Sync for Fork {}
impl Fork {
    #[doc = "Description cluster\\[n\\]: Channel n task end-point"]
    #[inline(always)]
    pub fn tep(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
#[doc = "PPI Channel"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch(pub *mut u8);
unsafe impl Send for Ch {}
unsafe impl Sync for Ch {}
impl Ch {
    #[doc = "Description cluster\\[n\\]: Channel n event end-point"]
    #[inline(always)]
    pub fn eep(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Channel n task end-point"]
    #[inline(always)]
    pub fn tep(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
#[doc = "Programmable Peripheral Interconnect"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppi(pub *mut u8);
unsafe impl Send for Ppi {}
unsafe impl Sync for Ppi {}
impl Ppi {
    #[doc = "Channel group tasks"]
    #[inline(always)]
    pub fn tasks_chg(self, n: usize) -> TasksChg {
        assert!(n < 6usize);
        unsafe { TasksChg(self.0.add(0usize + n * 8usize)) }
    }
    #[doc = "Channel enable register"]
    #[inline(always)]
    pub fn chen(self) -> crate::common::Reg<regs::Chen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Channel enable set register"]
    #[inline(always)]
    pub fn chenset(self) -> crate::common::Reg<regs::Chenset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "Channel enable clear register"]
    #[inline(always)]
    pub fn chenclr(self) -> crate::common::Reg<regs::Chenclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "PPI Channel"]
    #[inline(always)]
    pub fn ch(self, n: usize) -> Ch {
        assert!(n < 20usize);
        unsafe { Ch(self.0.add(1296usize + n * 8usize)) }
    }
    #[doc = "Description collection\\[n\\]: Channel group n"]
    #[inline(always)]
    pub fn chg(self, n: usize) -> crate::common::Reg<regs::Chg, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(2048usize + n * 4usize)) }
    }
    #[doc = "Fork"]
    #[inline(always)]
    pub fn fork(self, n: usize) -> Fork {
        assert!(n < 32usize);
        unsafe { Fork(self.0.add(2320usize + n * 4usize)) }
    }
}
#[doc = "Channel group tasks"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TasksChg(pub *mut u8);
unsafe impl Send for TasksChg {}
unsafe impl Sync for TasksChg {}
impl TasksChg {
    #[doc = "Description cluster\\[n\\]: Enable channel group n"]
    #[inline(always)]
    pub fn en(self) -> crate::common::Reg<regs::En, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Disable channel group n"]
    #[inline(always)]
    pub fn dis(self) -> crate::common::Reg<regs::Dis, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
pub mod regs;
