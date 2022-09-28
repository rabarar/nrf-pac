#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxd(pub *mut u8);
unsafe impl Send for Rxd {}
unsafe impl Sync for Rxd {}
impl Rxd {
    #[doc = "RXD data pointer"]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Maximum number of bytes in receive buffer"]
    #[inline(always)]
    pub fn maxcnt(self) -> crate::common::Reg<regs::RxdMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Number of bytes received in last granted transaction"]
    #[inline(always)]
    pub fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
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
    #[doc = "Pin select for MISO signal"]
    #[inline(always)]
    pub fn miso(self) -> crate::common::Reg<regs::Miso, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Pin select for MOSI signal"]
    #[inline(always)]
    pub fn mosi(self) -> crate::common::Reg<regs::Mosi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Pin select for CSN signal"]
    #[inline(always)]
    pub fn csn(self) -> crate::common::Reg<regs::Csn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
#[doc = "SPI Slave 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spis0(pub *mut u8);
unsafe impl Send for Spis0 {}
unsafe impl Sync for Spis0 {}
impl Spis0 {
    #[doc = "Acquire SPI semaphore"]
    #[inline(always)]
    pub fn tasks_acquire(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Release SPI semaphore, enabling the SPI slave to acquire it"]
    #[inline(always)]
    pub fn tasks_release(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Granted transaction completed"]
    #[inline(always)]
    pub fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "End of RXD buffer reached"]
    #[inline(always)]
    pub fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
    }
    #[doc = "Semaphore acquired"]
    #[inline(always)]
    pub fn events_acquired(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
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
    #[doc = "Semaphore status register"]
    #[inline(always)]
    pub fn semstat(self) -> crate::common::Reg<regs::Semstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1024usize)) }
    }
    #[doc = "Status from last transaction"]
    #[inline(always)]
    pub fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1088usize)) }
    }
    #[doc = "Enable SPI slave"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn psel(self) -> Psel {
        unsafe { Psel(self.0.add(1288usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn rxd(self) -> Rxd {
        unsafe { Rxd(self.0.add(1332usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn txd(self) -> Txd {
        unsafe { Txd(self.0.add(1348usize)) }
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1364usize)) }
    }
    #[doc = "Default character. Character clocked out in case of an ignored transaction."]
    #[inline(always)]
    pub fn def(self) -> crate::common::Reg<regs::Def, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1372usize)) }
    }
    #[doc = "Over-read character"]
    #[inline(always)]
    pub fn orc(self) -> crate::common::Reg<regs::Orc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1472usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txd(pub *mut u8);
unsafe impl Send for Txd {}
unsafe impl Sync for Txd {}
impl Txd {
    #[doc = "TXD data pointer"]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Maximum number of bytes in transmit buffer"]
    #[inline(always)]
    pub fn maxcnt(self) -> crate::common::Reg<regs::TxdMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Number of bytes transmitted in last granted transaction"]
    #[inline(always)]
    pub fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
pub mod regs;
pub mod vals;
