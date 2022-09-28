#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub *mut u8);
unsafe impl Send for Config {}
unsafe impl Sync for Config {}
impl Config {
    #[doc = "I2S mode."]
    #[inline(always)]
    pub fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Reception (RX) enable."]
    #[inline(always)]
    pub fn rxen(self) -> crate::common::Reg<regs::Rxen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Transmission (TX) enable."]
    #[inline(always)]
    pub fn txen(self) -> crate::common::Reg<regs::Txen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Master clock generator enable."]
    #[inline(always)]
    pub fn mcken(self) -> crate::common::Reg<regs::Mcken, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Master clock generator frequency."]
    #[inline(always)]
    pub fn mckfreq(self) -> crate::common::Reg<regs::Mckfreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "MCK / LRCK ratio."]
    #[inline(always)]
    pub fn ratio(self) -> crate::common::Reg<regs::Ratio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Sample width."]
    #[inline(always)]
    pub fn swidth(self) -> crate::common::Reg<regs::Swidth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Alignment of sample within a frame."]
    #[inline(always)]
    pub fn align(self) -> crate::common::Reg<regs::Align, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Frame format."]
    #[inline(always)]
    pub fn format(self) -> crate::common::Reg<regs::Format, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Enable channels."]
    #[inline(always)]
    pub fn channels(self) -> crate::common::Reg<regs::Channels, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
}
#[doc = "Inter-IC Sound"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2s(pub *mut u8);
unsafe impl Send for I2s {}
unsafe impl Sync for I2s {}
impl I2s {
    #[doc = "Starts continuous I2S transfer. Also starts MCK generator when this is enabled."]
    #[inline(always)]
    pub fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the {event:STOPPED} event to be generated."]
    #[inline(always)]
    pub fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin."]
    #[inline(always)]
    pub fn events_rxptrupd(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "I2S transfer stopped."]
    #[inline(always)]
    pub fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }
    #[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
    #[inline(always)]
    pub fn events_txptrupd(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(276usize)) }
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
    #[doc = "Enable I2S module."]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn config(self) -> Config {
        unsafe { Config(self.0.add(1284usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn rxd(self) -> Rxd {
        unsafe { Rxd(self.0.add(1336usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn txd(self) -> Txd {
        unsafe { Txd(self.0.add(1344usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn rxtxd(self) -> Rxtxd {
        unsafe { Rxtxd(self.0.add(1360usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn psel(self) -> Psel {
        unsafe { Psel(self.0.add(1376usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxtxd(pub *mut u8);
unsafe impl Send for Rxtxd {}
unsafe impl Sync for Rxtxd {}
impl Rxtxd {
    #[doc = "Size of RXD and TXD buffers."]
    #[inline(always)]
    pub fn maxcnt(self) -> crate::common::Reg<regs::Maxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psel(pub *mut u8);
unsafe impl Send for Psel {}
unsafe impl Sync for Psel {}
impl Psel {
    #[doc = "Pin select for MCK signal."]
    #[inline(always)]
    pub fn mck(self) -> crate::common::Reg<regs::Mck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Pin select for SCK signal."]
    #[inline(always)]
    pub fn sck(self) -> crate::common::Reg<regs::Sck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Pin select for LRCK signal."]
    #[inline(always)]
    pub fn lrck(self) -> crate::common::Reg<regs::Lrck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Pin select for SDIN signal."]
    #[inline(always)]
    pub fn sdin(self) -> crate::common::Reg<regs::Sdin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Pin select for SDOUT signal."]
    #[inline(always)]
    pub fn sdout(self) -> crate::common::Reg<regs::Sdout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxd(pub *mut u8);
unsafe impl Send for Rxd {}
unsafe impl Sync for Rxd {}
impl Rxd {
    #[doc = "Receive buffer RAM start address."]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txd(pub *mut u8);
unsafe impl Send for Txd {}
unsafe impl Sync for Txd {}
impl Txd {
    #[doc = "Transmit buffer RAM start address."]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod regs;
pub mod vals;
