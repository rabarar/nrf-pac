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
    #[doc = "Number of bytes in transmit buffer"]
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
pub struct Iftiming(pub *mut u8);
unsafe impl Send for Iftiming {}
unsafe impl Sync for Iftiming {}
impl Iftiming {
    #[doc = "Sample delay for input serial data on MISO"]
    #[inline(always)]
    pub fn rxdelay(self) -> crate::common::Reg<regs::Rxdelay, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions"]
    #[inline(always)]
    pub fn csndur(self) -> crate::common::Reg<regs::Csndur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
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
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psel(pub *mut u8);
unsafe impl Send for Psel {}
unsafe impl Sync for Psel {}
impl Psel {
    #[doc = "Pin select for SCK"]
    #[inline(always)]
    pub fn sck(self) -> crate::common::Reg<regs::Sck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Pin select for MOSI signal"]
    #[inline(always)]
    pub fn mosi(self) -> crate::common::Reg<regs::Mosi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Pin select for MISO signal"]
    #[inline(always)]
    pub fn miso(self) -> crate::common::Reg<regs::Miso, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Pin select for CSN"]
    #[inline(always)]
    pub fn csn(self) -> crate::common::Reg<regs::Csn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spim0(pub *mut u8);
unsafe impl Send for Spim0 {}
unsafe impl Sync for Spim0 {}
impl Spim0 {
    #[doc = "Start SPI transaction"]
    #[inline(always)]
    pub fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Stop SPI transaction"]
    #[inline(always)]
    pub fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Suspend SPI transaction"]
    #[inline(always)]
    pub fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Resume SPI transaction"]
    #[inline(always)]
    pub fn tasks_resume(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "SPI transaction has stopped"]
    #[inline(always)]
    pub fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "End of RXD buffer reached"]
    #[inline(always)]
    pub fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
    }
    #[doc = "End of RXD buffer and TXD buffer reached"]
    #[inline(always)]
    pub fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(280usize)) }
    }
    #[doc = "End of TXD buffer reached"]
    #[inline(always)]
    pub fn events_endtx(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(288usize)) }
    }
    #[doc = "Transaction started"]
    #[inline(always)]
    pub fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(332usize)) }
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
    #[doc = "Stall status for EasyDMA RAM accesses. The fields in this register is set to STALL by hardware whenever a stall occurres and can be cleared (set to NOSTALL) by the CPU."]
    #[inline(always)]
    pub fn stallstat(self) -> crate::common::Reg<regs::Stallstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1024usize)) }
    }
    #[doc = "Enable SPIM"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn psel(self) -> Psel {
        unsafe { Psel(self.0.add(1288usize)) }
    }
    #[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
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
    #[doc = "Configuration register"]
    #[inline(always)]
    pub fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1364usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn iftiming(self) -> Iftiming {
        unsafe { Iftiming(self.0.add(1376usize)) }
    }
    #[doc = "Polarity of CSN output"]
    #[inline(always)]
    pub fn csnpol(self) -> crate::common::Reg<regs::Csnpol, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1384usize)) }
    }
    #[doc = "Pin select for DCX signal"]
    #[inline(always)]
    pub fn pseldcx(self) -> crate::common::Reg<regs::Pseldcx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1388usize)) }
    }
    #[doc = "DCX configuration"]
    #[inline(always)]
    pub fn dcxcnt(self) -> crate::common::Reg<regs::Dcxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1392usize)) }
    }
    #[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
    #[inline(always)]
    pub fn orc(self) -> crate::common::Reg<regs::Orc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1472usize)) }
    }
}
pub mod regs;
pub mod vals;
