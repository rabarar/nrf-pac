#[doc = "Universal Asynchronous Receiver/Transmitter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart0(pub *mut u8);
unsafe impl Send for Uart0 {}
unsafe impl Sync for Uart0 {}
impl Uart0 {
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
    #[doc = "Suspend UART"]
    #[inline(always)]
    pub fn tasks_suspend(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
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
    #[doc = "Data received in RXD"]
    #[inline(always)]
    pub fn events_rxdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }
    #[doc = "Data sent from TXD"]
    #[inline(always)]
    pub fn events_txdrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(284usize)) }
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
    #[doc = "RXD register"]
    #[inline(always)]
    pub fn rxd(self) -> crate::common::Reg<regs::Uart0rxd, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1304usize)) }
    }
    #[doc = "TXD register"]
    #[inline(always)]
    pub fn txd(self) -> crate::common::Reg<regs::Uart0txd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1308usize)) }
    }
    #[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
    #[inline(always)]
    pub fn baudrate(self) -> crate::common::Reg<regs::Baudrate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1316usize)) }
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
    #[doc = "Pin select for RTS"]
    #[inline(always)]
    pub fn rts(self) -> crate::common::Reg<regs::Rts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Pin select for TXD"]
    #[inline(always)]
    pub fn txd(self) -> crate::common::Reg<regs::PselTxd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Pin select for CTS"]
    #[inline(always)]
    pub fn cts(self) -> crate::common::Reg<regs::Cts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Pin select for RXD"]
    #[inline(always)]
    pub fn rxd(self) -> crate::common::Reg<regs::PselRxd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
pub mod regs;
pub mod vals;
