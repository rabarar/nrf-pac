#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isoout(pub *mut u8);
unsafe impl Send for Isoout {}
unsafe impl Sync for Isoout {}
impl Isoout {
    #[doc = "Data pointer"]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Maximum number of bytes to transfer"]
    #[inline(always)]
    pub fn maxcnt(self) -> crate::common::Reg<regs::IsooutMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub fn amount(self) -> crate::common::Reg<regs::IsooutAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Halted(pub *mut u8);
unsafe impl Send for Halted {}
unsafe impl Sync for Halted {}
impl Halted {
    #[doc = "Description collection\\[n\\]: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub fn epin(self, n: usize) -> crate::common::Reg<regs::Epin, crate::common::R> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize + n * 4usize)) }
    }
    #[doc = "Description collection\\[n\\]: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    #[inline(always)]
    pub fn epout(self, n: usize) -> crate::common::Reg<regs::HaltedEpout, crate::common::R> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize + n * 4usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Size(pub *mut u8);
unsafe impl Send for Size {}
unsafe impl Sync for Size {}
impl Size {
    #[doc = "Description collection\\[n\\]: Number of bytes received last in the data stage of this OUT endpoint"]
    #[inline(always)]
    pub fn epout(self, n: usize) -> crate::common::Reg<regs::SizeEpout, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize + n * 4usize)) }
    }
    #[doc = "Number of bytes received last on this ISO OUT data endpoint"]
    #[inline(always)]
    pub fn isoout(self) -> crate::common::Reg<regs::Isoout, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
}
#[doc = "Universal serial bus device"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbd(pub *mut u8);
unsafe impl Send for Usbd {}
unsafe impl Sync for Usbd {}
impl Usbd {
    #[doc = "Description collection\\[n\\]: Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
    #[inline(always)]
    pub fn tasks_startepin(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize + n * 4usize)) }
    }
    #[doc = "Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
    #[inline(always)]
    pub fn tasks_startisoin(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Description collection\\[n\\]: Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host"]
    #[inline(always)]
    pub fn tasks_startepout(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize + n * 4usize)) }
    }
    #[doc = "Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint"]
    #[inline(always)]
    pub fn tasks_startisoout(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }
    #[doc = "Allows OUT data stage on control endpoint 0"]
    #[inline(always)]
    pub fn tasks_ep0rcvout(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }
    #[doc = "Allows status stage on control endpoint 0"]
    #[inline(always)]
    pub fn tasks_ep0status(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }
    #[doc = "Stalls data and status stage on control endpoint 0"]
    #[inline(always)]
    pub fn tasks_ep0stall(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }
    #[doc = "Forces D+ and D- lines into the state defined in the DPDMVALUE register"]
    #[inline(always)]
    pub fn tasks_dpdmdrive(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(88usize)) }
    }
    #[doc = "Stops forcing D+ and D- lines into any state (USB engine takes control)"]
    #[inline(always)]
    pub fn tasks_dpdmnodrive(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }
    #[doc = "Signals that a USB reset condition has been detected on USB lines"]
    #[inline(always)]
    pub fn events_usbreset(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "Confirms that the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT, or EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers have been captured on all endpoints reported in the EPSTATUS register"]
    #[inline(always)]
    pub fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "Description collection\\[n\\]: The whole EPIN\\[n\\] buffer has been consumed. The RAM buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endepin(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize + n * 4usize)) }
    }
    #[doc = "An acknowledged data transfer has taken place on the control endpoint"]
    #[inline(always)]
    pub fn events_ep0datadone(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }
    #[doc = "The whole ISOIN buffer has been consumed. The RAM buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endisoin(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(300usize)) }
    }
    #[doc = "Description collection\\[n\\]: The whole EPOUT\\[n\\] buffer has been consumed. The RAM buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endepout(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(304usize + n * 4usize)) }
    }
    #[doc = "The whole ISOOUT buffer has been consumed. The RAM buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endisoout(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(336usize)) }
    }
    #[doc = "Signals that a SOF (start of frame) condition has been detected on USB lines"]
    #[inline(always)]
    pub fn events_sof(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(340usize)) }
    }
    #[doc = "An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
    #[inline(always)]
    pub fn events_usbevent(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(344usize)) }
    }
    #[doc = "A valid SETUP token has been received (and acknowledged) on the control endpoint"]
    #[inline(always)]
    pub fn events_ep0setup(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(348usize)) }
    }
    #[doc = "A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
    #[inline(always)]
    pub fn events_epdata(self) -> crate::common::Reg<u32, crate::common::RW> {
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
    #[doc = "Details on what caused the USBEVENT event"]
    #[inline(always)]
    pub fn eventcause(self) -> crate::common::Reg<regs::Eventcause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1024usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn halted(self) -> Halted {
        unsafe { Halted(self.0.add(1056usize)) }
    }
    #[doc = "Provides information on which endpoint's EasyDMA registers have been captured"]
    #[inline(always)]
    pub fn epstatus(self) -> crate::common::Reg<regs::Epstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1128usize)) }
    }
    #[doc = "Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)"]
    #[inline(always)]
    pub fn epdatastatus(self) -> crate::common::Reg<regs::Epdatastatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1132usize)) }
    }
    #[doc = "Device USB address"]
    #[inline(always)]
    pub fn usbaddr(self) -> crate::common::Reg<regs::Usbaddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1136usize)) }
    }
    #[doc = "SETUP data, byte 0, bmRequestType"]
    #[inline(always)]
    pub fn bmrequesttype(self) -> crate::common::Reg<regs::Bmrequesttype, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1152usize)) }
    }
    #[doc = "SETUP data, byte 1, bRequest"]
    #[inline(always)]
    pub fn brequest(self) -> crate::common::Reg<regs::Brequest, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1156usize)) }
    }
    #[doc = "SETUP data, byte 2, LSB of wValue"]
    #[inline(always)]
    pub fn wvaluel(self) -> crate::common::Reg<regs::Wvaluel, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1160usize)) }
    }
    #[doc = "SETUP data, byte 3, MSB of wValue"]
    #[inline(always)]
    pub fn wvalueh(self) -> crate::common::Reg<regs::Wvalueh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1164usize)) }
    }
    #[doc = "SETUP data, byte 4, LSB of wIndex"]
    #[inline(always)]
    pub fn windexl(self) -> crate::common::Reg<regs::Windexl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1168usize)) }
    }
    #[doc = "SETUP data, byte 5, MSB of wIndex"]
    #[inline(always)]
    pub fn windexh(self) -> crate::common::Reg<regs::Windexh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1172usize)) }
    }
    #[doc = "SETUP data, byte 6, LSB of wLength"]
    #[inline(always)]
    pub fn wlengthl(self) -> crate::common::Reg<regs::Wlengthl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1176usize)) }
    }
    #[doc = "SETUP data, byte 7, MSB of wLength"]
    #[inline(always)]
    pub fn wlengthh(self) -> crate::common::Reg<regs::Wlengthh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1180usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn size(self) -> Size {
        unsafe { Size(self.0.add(1184usize)) }
    }
    #[doc = "Enable USB"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Control of the USB pull-up"]
    #[inline(always)]
    pub fn usbpullup(self) -> crate::common::Reg<regs::Usbpullup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing)."]
    #[inline(always)]
    pub fn dpdmvalue(self) -> crate::common::Reg<regs::Dpdmvalue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "Data toggle control and status"]
    #[inline(always)]
    pub fn dtoggle(self) -> crate::common::Reg<regs::Dtoggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1292usize)) }
    }
    #[doc = "Endpoint IN enable"]
    #[inline(always)]
    pub fn epinen(self) -> crate::common::Reg<regs::Epinen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize)) }
    }
    #[doc = "Endpoint OUT enable"]
    #[inline(always)]
    pub fn epouten(self) -> crate::common::Reg<regs::Epouten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1300usize)) }
    }
    #[doc = "STALL endpoints"]
    #[inline(always)]
    pub fn epstall(self) -> crate::common::Reg<regs::Epstall, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1304usize)) }
    }
    #[doc = "Controls the split of ISO buffers"]
    #[inline(always)]
    pub fn isosplit(self) -> crate::common::Reg<regs::Isosplit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1308usize)) }
    }
    #[doc = "Returns the current value of the start of frame counter"]
    #[inline(always)]
    pub fn framecntr(self) -> crate::common::Reg<regs::Framecntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1312usize)) }
    }
    #[doc = "Controls USBD peripheral low power mode during USB suspend"]
    #[inline(always)]
    pub fn lowpower(self) -> crate::common::Reg<regs::Lowpower, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1324usize)) }
    }
    #[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
    #[inline(always)]
    pub fn isoinconfig(self) -> crate::common::Reg<regs::Isoinconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1328usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn epin(self, n: usize) -> Epin {
        assert!(n < 8usize);
        unsafe { Epin(self.0.add(1536usize + n * 20usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn isoin(self) -> Isoin {
        unsafe { Isoin(self.0.add(1696usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn epout(self, n: usize) -> Epout {
        assert!(n < 8usize);
        unsafe { Epout(self.0.add(1792usize + n * 20usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn isoout(self) -> Isoout {
        unsafe { Isoout(self.0.add(1952usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epout(pub *mut u8);
unsafe impl Send for Epout {}
unsafe impl Sync for Epout {}
impl Epout {
    #[doc = "Description cluster\\[n\\]: Data pointer"]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Maximum number of bytes to transfer"]
    #[inline(always)]
    pub fn maxcnt(self) -> crate::common::Reg<regs::EpoutMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub fn amount(self) -> crate::common::Reg<regs::EpoutAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isoin(pub *mut u8);
unsafe impl Send for Isoin {}
unsafe impl Sync for Isoin {}
impl Isoin {
    #[doc = "Data pointer"]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Maximum number of bytes to transfer"]
    #[inline(always)]
    pub fn maxcnt(self) -> crate::common::Reg<regs::IsoinMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub fn amount(self) -> crate::common::Reg<regs::IsoinAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epin(pub *mut u8);
unsafe impl Send for Epin {}
unsafe impl Sync for Epin {}
impl Epin {
    #[doc = "Description cluster\\[n\\]: Data pointer"]
    #[inline(always)]
    pub fn ptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Maximum number of bytes to transfer"]
    #[inline(always)]
    pub fn maxcnt(self) -> crate::common::Reg<regs::EpinMaxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub fn amount(self) -> crate::common::Reg<regs::EpinAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
pub mod regs;
pub mod vals;
