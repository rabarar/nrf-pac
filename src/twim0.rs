#[doc = "TXD EasyDMA channel"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txd(pub *mut u8);
unsafe impl Send for Txd {}
unsafe impl Sync for Txd {}
impl Txd {
    #[doc = "Data pointer"]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Maximum number of bytes in transmit buffer"]
    #[inline(always)]
    pub fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "EasyDMA list type"]
    #[inline(always)]
    pub fn list(self) -> crate::common::Reg<regs::TxdList, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psel(pub *mut u8);
unsafe impl Send for Psel {}
unsafe impl Sync for Psel {}
impl Psel {
    #[doc = "Pin select for SCL signal"]
    #[inline(always)]
    pub fn scl(self) -> crate::common::Reg<regs::Scl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Pin select for SDA signal"]
    #[inline(always)]
    pub fn sda(self) -> crate::common::Reg<regs::Sda, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Twim0(pub *mut u8);
unsafe impl Send for Twim0 {}
unsafe impl Sync for Twim0 {}
impl Twim0 {
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
    #[doc = "Stop TWI transaction. Must be issued while the TWI master is not suspended."]
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
    #[doc = "TWI error"]
    #[inline(always)]
    pub fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(292usize)) }
    }
    #[doc = "Last byte has been sent out after the SUSPEND task has been issued, TWI traffic is now suspended."]
    #[inline(always)]
    pub fn events_suspended(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(328usize)) }
    }
    #[doc = "Receive sequence started"]
    #[inline(always)]
    pub fn events_rxstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(332usize)) }
    }
    #[doc = "Transmit sequence started"]
    #[inline(always)]
    pub fn events_txstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(336usize)) }
    }
    #[doc = "Byte boundary, starting to receive the last byte"]
    #[inline(always)]
    pub fn events_lastrx(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(348usize)) }
    }
    #[doc = "Byte boundary, starting to transmit the last byte"]
    #[inline(always)]
    pub fn events_lasttx(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(352usize)) }
    }
    #[doc = "Shortcut register"]
    #[inline(always)]
    pub fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(512usize)) }
    }
    #[doc = "Enable or disable interrupt"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(768usize)) }
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
    #[doc = "Enable TWIM"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn psel(self) -> Psel {
        unsafe { Psel(self.0.add(1288usize)) }
    }
    #[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
    #[inline(always)]
    pub fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1316usize)) }
    }
    #[doc = "RXD EasyDMA channel"]
    #[inline(always)]
    pub fn rxd(self) -> Rxd {
        unsafe { Rxd(self.0.add(1332usize)) }
    }
    #[doc = "TXD EasyDMA channel"]
    #[inline(always)]
    pub fn txd(self) -> Txd {
        unsafe { Txd(self.0.add(1348usize)) }
    }
    #[doc = "Address used in the TWI transfer"]
    #[inline(always)]
    pub fn address(self) -> crate::common::Reg<regs::Address, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1416usize)) }
    }
}
#[doc = "RXD EasyDMA channel"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxd(pub *mut u8);
unsafe impl Send for Rxd {}
unsafe impl Sync for Rxd {}
impl Rxd {
    #[doc = "Data pointer"]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Maximum number of bytes in receive buffer"]
    #[inline(always)]
    pub fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "EasyDMA list type"]
    #[inline(always)]
    pub fn list(self) -> crate::common::Reg<regs::RxdList, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
pub mod regs;
pub mod vals;
