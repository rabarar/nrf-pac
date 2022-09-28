#[doc = "I2C compatible Two-Wire Interface 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Twi0(pub *mut u8);
unsafe impl Send for Twi0 {}
unsafe impl Sync for Twi0 {}
impl Twi0 {
    #[doc = "Start TWI receive sequence"]
    #[inline(always)]
    pub fn tasks_startrx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Start TWI transmit sequence"]
    #[inline(always)]
    pub fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Stop TWI transaction"]
    #[inline(always)]
    pub fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Suspend TWI transaction"]
    #[inline(always)]
    pub fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Resume TWI transaction"]
    #[inline(always)]
    pub fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "TWI stopped"]
    #[inline(always)]
    pub fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "TWI RXD byte received"]
    #[inline(always)]
    pub fn events_rxdready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }
    #[doc = "TWI TXD byte sent"]
    #[inline(always)]
    pub fn events_txdsent(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(284usize)) }
    }
    #[doc = "TWI error"]
    #[inline(always)]
    pub fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(292usize)) }
    }
    #[doc = "TWI byte boundary, generated before each byte that is sent or received"]
    #[inline(always)]
    pub fn events_bb(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(312usize)) }
    }
    #[doc = "TWI entered the suspended state"]
    #[inline(always)]
    pub fn events_suspended(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(328usize)) }
    }
    #[doc = "Shortcut register"]
    #[inline(always)]
    pub fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(512usize)) }
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
    #[doc = "Error source"]
    #[inline(always)]
    pub fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1220usize)) }
    }
    #[doc = "Enable TWI"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn psel(self) -> Psel {
        unsafe { Psel(self.0.add(1288usize)) }
    }
    #[doc = "RXD register"]
    #[inline(always)]
    pub fn rxd(self) -> crate::common::Reg<regs::Rxd, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1304usize)) }
    }
    #[doc = "TXD register"]
    #[inline(always)]
    pub fn txd(self) -> crate::common::Reg<regs::Txd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1308usize)) }
    }
    #[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
    #[inline(always)]
    pub fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1316usize)) }
    }
    #[doc = "Address used in the TWI transfer"]
    #[inline(always)]
    pub fn address(self) -> crate::common::Reg<regs::Address, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1416usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psel(pub *mut u8);
unsafe impl Send for Psel {}
unsafe impl Sync for Psel {}
impl Psel {
    #[doc = "Pin select for SCL"]
    #[inline(always)]
    pub fn scl(self) -> crate::common::Reg<regs::Scl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Pin select for SDA"]
    #[inline(always)]
    pub fn sda(self) -> crate::common::Reg<regs::Sda, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
pub mod regs;
pub mod vals;
