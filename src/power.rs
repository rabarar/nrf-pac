#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ram(pub *mut u8);
unsafe impl Send for Ram {}
unsafe impl Sync for Ram {}
impl Ram {
    #[doc = "Description cluster\\[n\\]: RAMn power control register"]
    #[inline(always)]
    pub fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Description cluster\\[n\\]: RAMn power control set register"]
    #[inline(always)]
    pub fn powerset(self) -> crate::common::Reg<regs::Powerset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Description cluster\\[n\\]: RAMn power control clear register"]
    #[inline(always)]
    pub fn powerclr(self) -> crate::common::Reg<regs::Powerclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[doc = "Power control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Power(pub *mut u8);
unsafe impl Send for Power {}
unsafe impl Sync for Power {}
impl Power {
    #[doc = "Enable constant latency mode"]
    #[inline(always)]
    pub fn tasks_constlat(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }
    #[doc = "Enable low power mode (variable latency)"]
    #[inline(always)]
    pub fn tasks_lowpwr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }
    #[doc = "Power failure warning"]
    #[inline(always)]
    pub fn events_pofwarn(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }
    #[doc = "CPU entered WFI/WFE sleep"]
    #[inline(always)]
    pub fn events_sleepenter(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(276usize)) }
    }
    #[doc = "CPU exited WFI/WFE sleep"]
    #[inline(always)]
    pub fn events_sleepexit(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(280usize)) }
    }
    #[doc = "Voltage supply detected on VBUS"]
    #[inline(always)]
    pub fn events_usbdetected(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(284usize)) }
    }
    #[doc = "Voltage supply removed from VBUS"]
    #[inline(always)]
    pub fn events_usbremoved(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(288usize)) }
    }
    #[doc = "USB 3.3 V supply ready"]
    #[inline(always)]
    pub fn events_usbpwrrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(292usize)) }
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(772usize)) }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(776usize)) }
    }
    #[doc = "Reset reason"]
    #[inline(always)]
    pub fn resetreas(self) -> crate::common::Reg<regs::Resetreas, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1024usize)) }
    }
    #[doc = "Deprecated register - RAM status register"]
    #[inline(always)]
    pub fn ramstatus(self) -> crate::common::Reg<regs::Ramstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1064usize)) }
    }
    #[doc = "USB supply status"]
    #[inline(always)]
    pub fn usbregstatus(self) -> crate::common::Reg<regs::Usbregstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1080usize)) }
    }
    #[doc = "System OFF register"]
    #[inline(always)]
    pub fn systemoff(self) -> crate::common::Reg<regs::Systemoff, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Power-fail comparator configuration"]
    #[inline(always)]
    pub fn pofcon(self) -> crate::common::Reg<regs::Pofcon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize)) }
    }
    #[doc = "General purpose retention register"]
    #[inline(always)]
    pub fn gpregret(self) -> crate::common::Reg<regs::Gpregret, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1308usize)) }
    }
    #[doc = "General purpose retention register"]
    #[inline(always)]
    pub fn gpregret2(self) -> crate::common::Reg<regs::Gpregret2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1312usize)) }
    }
    #[doc = "Enable DC/DC converter for REG1 stage."]
    #[inline(always)]
    pub fn dcdcen(self) -> crate::common::Reg<regs::Dcdcen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1400usize)) }
    }
    #[doc = "Enable DC/DC converter for REG0 stage."]
    #[inline(always)]
    pub fn dcdcen0(self) -> crate::common::Reg<regs::Dcdcen0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1408usize)) }
    }
    #[doc = "Main supply status"]
    #[inline(always)]
    pub fn mainregstatus(self) -> crate::common::Reg<regs::Mainregstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1600usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn ram(self, n: usize) -> Ram {
        assert!(n < 9usize);
        unsafe { Ram(self.0.add(2304usize + n * 16usize)) }
    }
}
pub mod regs;
pub mod vals;
