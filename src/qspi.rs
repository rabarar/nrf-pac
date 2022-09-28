#[doc = "External flash interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspi(pub *mut u8);
unsafe impl Send for Qspi {}
unsafe impl Sync for Qspi {}
impl Qspi {
    #[doc = "Activate QSPI interface"]
    #[inline(always)]
    pub fn tasks_activate(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Start transfer from external flash memory to internal RAM"]
    #[inline(always)]
    pub fn tasks_readstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Start transfer from internal RAM to external flash memory"]
    #[inline(always)]
    pub fn tasks_writestart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Start external flash memory erase operation"]
    #[inline(always)]
    pub fn tasks_erasestart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Deactivate QSPI interface"]
    #[inline(always)]
    pub fn tasks_deactivate(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "QSPI peripheral is ready. This event will be generated as a response to any QSPI task."]
    #[inline(always)]
    pub fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
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
    #[doc = "Enable QSPI peripheral and acquire the pins selected in PSELn registers"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn read(self) -> Read {
        unsafe { Read(self.0.add(1284usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn write(self) -> Write {
        unsafe { Write(self.0.add(1296usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn erase(self) -> Erase {
        unsafe { Erase(self.0.add(1308usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn psel(self) -> Psel {
        unsafe { Psel(self.0.add(1316usize)) }
    }
    #[doc = "Address offset into the external memory for Execute in Place operation."]
    #[inline(always)]
    pub fn xipoffset(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1344usize)) }
    }
    #[doc = "Interface configuration."]
    #[inline(always)]
    pub fn ifconfig0(self) -> crate::common::Reg<regs::Ifconfig0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1348usize)) }
    }
    #[doc = "Interface configuration."]
    #[inline(always)]
    pub fn ifconfig1(self) -> crate::common::Reg<regs::Ifconfig1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1536usize)) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1540usize)) }
    }
    #[doc = "Set the duration required to enter/exit deep power-down mode (DPM)."]
    #[inline(always)]
    pub fn dpmdur(self) -> crate::common::Reg<regs::Dpmdur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1556usize)) }
    }
    #[doc = "Extended address configuration."]
    #[inline(always)]
    pub fn addrconf(self) -> crate::common::Reg<regs::Addrconf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1572usize)) }
    }
    #[doc = "Custom instruction configuration register."]
    #[inline(always)]
    pub fn cinstrconf(self) -> crate::common::Reg<regs::Cinstrconf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1588usize)) }
    }
    #[doc = "Custom instruction data register 0."]
    #[inline(always)]
    pub fn cinstrdat0(self) -> crate::common::Reg<regs::Cinstrdat0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1592usize)) }
    }
    #[doc = "Custom instruction data register 1."]
    #[inline(always)]
    pub fn cinstrdat1(self) -> crate::common::Reg<regs::Cinstrdat1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1596usize)) }
    }
    #[doc = "SPI interface timing."]
    #[inline(always)]
    pub fn iftiming(self) -> crate::common::Reg<regs::Iftiming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1600usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Read(pub *mut u8);
unsafe impl Send for Read {}
unsafe impl Sync for Read {}
impl Read {
    #[doc = "Flash memory source address"]
    #[inline(always)]
    pub fn src(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "RAM destination address"]
    #[inline(always)]
    pub fn dst(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Read transfer length"]
    #[inline(always)]
    pub fn cnt(self) -> crate::common::Reg<regs::ReadCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Write(pub *mut u8);
unsafe impl Send for Write {}
unsafe impl Sync for Write {}
impl Write {
    #[doc = "Flash destination address"]
    #[inline(always)]
    pub fn dst(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "RAM source address"]
    #[inline(always)]
    pub fn src(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Write transfer length"]
    #[inline(always)]
    pub fn cnt(self) -> crate::common::Reg<regs::WriteCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psel(pub *mut u8);
unsafe impl Send for Psel {}
unsafe impl Sync for Psel {}
impl Psel {
    #[doc = "Pin select for serial clock SCK"]
    #[inline(always)]
    pub fn sck(self) -> crate::common::Reg<regs::Sck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Pin select for chip select signal CSN."]
    #[inline(always)]
    pub fn csn(self) -> crate::common::Reg<regs::Csn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Pin select for serial data MOSI/IO0."]
    #[inline(always)]
    pub fn io0(self) -> crate::common::Reg<regs::Io0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Pin select for serial data MISO/IO1."]
    #[inline(always)]
    pub fn io1(self) -> crate::common::Reg<regs::Io1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Pin select for serial data IO2."]
    #[inline(always)]
    pub fn io2(self) -> crate::common::Reg<regs::Io2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Pin select for serial data IO3."]
    #[inline(always)]
    pub fn io3(self) -> crate::common::Reg<regs::Io3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erase(pub *mut u8);
unsafe impl Send for Erase {}
unsafe impl Sync for Erase {}
impl Erase {
    #[doc = "Start address of flash block to be erased"]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Size of block to be erased."]
    #[inline(always)]
    pub fn len(self) -> crate::common::Reg<regs::Len, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
pub mod regs;
pub mod vals;
