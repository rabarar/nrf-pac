#[doc = "Serial Peripheral Interface 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi0(pub *mut u8);
unsafe impl Send for Spi0 {}
unsafe impl Sync for Spi0 {}
impl Spi0 {
    #[doc = "TXD byte sent and RXD byte received"]
    #[inline(always)]
    pub fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
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
    #[doc = "Enable SPI"]
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
    #[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
    #[inline(always)]
    pub fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1316usize)) }
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1364usize)) }
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
}
pub mod regs;
pub mod vals;
