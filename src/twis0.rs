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
#[doc = "TXD EasyDMA channel"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txd(pub *mut u8);
unsafe impl Send for Txd {}
unsafe impl Sync for Txd {}
impl Txd {
    #[doc = "TXD Data pointer"]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Maximum number of bytes in TXD buffer"]
    #[inline(always)]
    pub fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Number of bytes transferred in the last TXD transaction"]
    #[inline(always)]
    pub fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[doc = "RXD EasyDMA channel"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxd(pub *mut u8);
unsafe impl Send for Rxd {}
unsafe impl Sync for Rxd {}
impl Rxd {
    #[doc = "RXD Data pointer"]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Maximum number of bytes in RXD buffer"]
    #[inline(always)]
    pub fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Number of bytes transferred in the last RXD transaction"]
    #[inline(always)]
    pub fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Twis0(pub *mut u8);
unsafe impl Send for Twis0 {}
unsafe impl Sync for Twis0 {}
impl Twis0 {
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
    #[doc = "Prepare the TWI slave to respond to a write command"]
    #[inline(always)]
    pub fn tasks_preparerx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Prepare the TWI slave to respond to a read command"]
    #[inline(always)]
    pub fn tasks_preparetx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
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
    #[doc = "Write command received"]
    #[inline(always)]
    pub fn events_write(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(356usize)) }
    }
    #[doc = "Read command received"]
    #[inline(always)]
    pub fn events_read(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(360usize)) }
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
        unsafe { crate::common::Reg::from_ptr(self.0.add(1232usize)) }
    }
    #[doc = "Status register indicating which address had a match"]
    #[inline(always)]
    pub fn match_(self) -> crate::common::Reg<regs::Match, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1236usize)) }
    }
    #[doc = "Enable TWIS"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn psel(self) -> Psel {
        unsafe { Psel(self.0.add(1288usize)) }
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
    #[doc = "Description collection\\[n\\]: TWI slave address n"]
    #[inline(always)]
    pub fn address(self, n: usize) -> crate::common::Reg<regs::Address, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(1416usize + n * 4usize)) }
    }
    #[doc = "Configuration register for the address match mechanism"]
    #[inline(always)]
    pub fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1428usize)) }
    }
    #[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
    #[inline(always)]
    pub fn orc(self) -> crate::common::Reg<regs::Orc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1472usize)) }
    }
}
pub mod regs;
pub mod vals;
