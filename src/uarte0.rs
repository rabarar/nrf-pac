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
}
#[doc = "UART with EasyDMA 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uarte0(pub *mut u8);
unsafe impl Send for Uarte0 {}
unsafe impl Sync for Uarte0 {}
impl Uarte0 {
    #[doc = "Start UART receiver"]
    #[inline(always)]
    pub fn tasks_startrx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Stop UART receiver"]
    #[inline(always)]
    pub fn tasks_stoprx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Start UART transmitter"]
    #[inline(always)]
    pub fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Stop UART transmitter"]
    #[inline(always)]
    pub fn tasks_stoptx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Flush RX FIFO into RX buffer"]
    #[inline(always)]
    pub fn tasks_flushrx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "CTS is activated (set low). Clear To Send."]
    #[inline(always)]
    pub fn events_cts(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "CTS is deactivated (set high). Not Clear To Send."]
    #[inline(always)]
    pub fn events_ncts(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "Data received in RXD (but potentially not yet transferred to Data RAM)"]
    #[inline(always)]
    pub fn events_rxdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }
    #[doc = "Receive buffer is filled up"]
    #[inline(always)]
    pub fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
    }
    #[doc = "Data sent from TXD"]
    #[inline(always)]
    pub fn events_txdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(284usize)) }
    }
    #[doc = "Last TX byte transmitted"]
    #[inline(always)]
    pub fn events_endtx(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(288usize)) }
    }
    #[doc = "Error detected"]
    #[inline(always)]
    pub fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(292usize)) }
    }
    #[doc = "Receiver timeout"]
    #[inline(always)]
    pub fn events_rxto(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(324usize)) }
    }
    #[doc = "UART receiver has started"]
    #[inline(always)]
    pub fn events_rxstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(332usize)) }
    }
    #[doc = "UART transmitter has started"]
    #[inline(always)]
    pub fn events_txstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(336usize)) }
    }
    #[doc = "Transmitter stopped"]
    #[inline(always)]
    pub fn events_txstopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(344usize)) }
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
    #[doc = "Error source Note : this register is read / write one to clear."]
    #[inline(always)]
    pub fn errorsrc(self) -> crate::common::Reg<regs::Errorsrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1152usize)) }
    }
    #[doc = "Enable UART"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn psel(self) -> Psel {
        unsafe { Psel(self.0.add(1288usize)) }
    }
    #[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
    #[inline(always)]
    pub fn baudrate(self) -> crate::common::Reg<regs::Baudrate, crate::common::RW> {
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
    #[doc = "Configuration of parity and hardware flow control"]
    #[inline(always)]
    pub fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1388usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psel(pub *mut u8);
unsafe impl Send for Psel {}
unsafe impl Sync for Psel {}
impl Psel {
    #[doc = "Pin select for RTS signal"]
    #[inline(always)]
    pub fn rts(self) -> crate::common::Reg<regs::Rts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Pin select for TXD signal"]
    #[inline(always)]
    pub fn txd(self) -> crate::common::Reg<regs::Txd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Pin select for CTS signal"]
    #[inline(always)]
    pub fn cts(self) -> crate::common::Reg<regs::Cts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Pin select for RXD signal"]
    #[inline(always)]
    pub fn rxd(self) -> crate::common::Reg<regs::Rxd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
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
}
pub mod regs;
pub mod vals;
