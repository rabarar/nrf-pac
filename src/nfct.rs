#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txd(pub *mut u8);
unsafe impl Send for Txd {}
unsafe impl Sync for Txd {}
impl Txd {
    #[doc = "Configuration of outgoing frames"]
    #[inline(always)]
    pub fn frameconfig(self) -> crate::common::Reg<regs::TxdFrameconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Size of outgoing frame"]
    #[inline(always)]
    pub fn amount(self) -> crate::common::Reg<regs::TxdAmount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
#[doc = "NFC-A compatible radio"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nfct(pub *mut u8);
unsafe impl Send for Nfct {}
unsafe impl Sync for Nfct {}
impl Nfct {
    #[doc = "Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
    #[inline(always)]
    pub fn tasks_activate(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Disable NFCT peripheral"]
    #[inline(always)]
    pub fn tasks_disable(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Enable NFC sense field mode, change state to sense mode"]
    #[inline(always)]
    pub fn tasks_sense(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Start transmission of an outgoing frame, change state to transmit"]
    #[inline(always)]
    pub fn tasks_starttx(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Initializes the EasyDMA for receive."]
    #[inline(always)]
    pub fn tasks_enablerxdata(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Force state machine to IDLE state"]
    #[inline(always)]
    pub fn tasks_goidle(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Force state machine to SLEEP_A state"]
    #[inline(always)]
    pub fn tasks_gosleep(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "The NFCT peripheral is ready to receive and send frames"]
    #[inline(always)]
    pub fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "Remote NFC field detected"]
    #[inline(always)]
    pub fn events_fielddetected(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "Remote NFC field lost"]
    #[inline(always)]
    pub fn events_fieldlost(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }
    #[doc = "Marks the start of the first symbol of a transmitted frame"]
    #[inline(always)]
    pub fn events_txframestart(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(268usize)) }
    }
    #[doc = "Marks the end of the last transmitted on-air symbol of a frame"]
    #[inline(always)]
    pub fn events_txframeend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
    }
    #[doc = "Marks the end of the first symbol of a received frame"]
    #[inline(always)]
    pub fn events_rxframestart(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(276usize)) }
    }
    #[doc = "Received data has been checked (CRC, parity) and transferred to RAM, and EasyDMA has ended accessing the RX buffer"]
    #[inline(always)]
    pub fn events_rxframeend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(280usize)) }
    }
    #[doc = "NFC error reported. The ERRORSTATUS register contains details on the source of the error."]
    #[inline(always)]
    pub fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(284usize)) }
    }
    #[doc = "NFC RX frame error reported. The FRAMESTATUS.RX register contains details on the source of the error."]
    #[inline(always)]
    pub fn events_rxerror(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }
    #[doc = "RX buffer (as defined by PACKETPTR and MAXLEN) in Data RAM full."]
    #[inline(always)]
    pub fn events_endrx(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(300usize)) }
    }
    #[doc = "Transmission of data in RAM has ended, and EasyDMA has ended accessing the TX buffer"]
    #[inline(always)]
    pub fn events_endtx(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(304usize)) }
    }
    #[doc = "Auto collision resolution process has started"]
    #[inline(always)]
    pub fn events_autocolresstarted(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(312usize)) }
    }
    #[doc = "NFC auto collision resolution error reported."]
    #[inline(always)]
    pub fn events_collision(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(328usize)) }
    }
    #[doc = "NFC auto collision resolution successfully completed"]
    #[inline(always)]
    pub fn events_selected(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(332usize)) }
    }
    #[doc = "EasyDMA is ready to receive or send frames."]
    #[inline(always)]
    pub fn events_started(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(336usize)) }
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
    #[doc = "NFC Error Status register"]
    #[inline(always)]
    pub fn errorstatus(self) -> crate::common::Reg<regs::Errorstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1028usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn framestatus(self) -> Framestatus {
        unsafe { Framestatus(self.0.add(1036usize)) }
    }
    #[doc = "NfcTag state register"]
    #[inline(always)]
    pub fn nfctagstate(self) -> crate::common::Reg<regs::Nfctagstate, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1040usize)) }
    }
    #[doc = "Sleep state during automatic collision resolution"]
    #[inline(always)]
    pub fn sleepstate(self) -> crate::common::Reg<regs::Sleepstate, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1056usize)) }
    }
    #[doc = "Indicates the presence or not of a valid field"]
    #[inline(always)]
    pub fn fieldpresent(self) -> crate::common::Reg<regs::Fieldpresent, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1084usize)) }
    }
    #[doc = "Minimum frame delay"]
    #[inline(always)]
    pub fn framedelaymin(self) -> crate::common::Reg<regs::Framedelaymin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "Maximum frame delay"]
    #[inline(always)]
    pub fn framedelaymax(self) -> crate::common::Reg<regs::Framedelaymax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "Configuration register for the Frame Delay Timer"]
    #[inline(always)]
    pub fn framedelaymode(self) -> crate::common::Reg<regs::Framedelaymode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1292usize)) }
    }
    #[doc = "Packet pointer for TXD and RXD data storage in Data RAM"]
    #[inline(always)]
    pub fn packetptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize)) }
    }
    #[doc = "Size of the RAM buffer allocated to TXD and RXD data storage each"]
    #[inline(always)]
    pub fn maxlen(self) -> crate::common::Reg<regs::Maxlen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1300usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn txd(self) -> Txd {
        unsafe { Txd(self.0.add(1304usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn rxd(self) -> Rxd {
        unsafe { Rxd(self.0.add(1312usize)) }
    }
    #[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)"]
    #[inline(always)]
    pub fn nfcid1_last(self) -> crate::common::Reg<regs::Nfcid1last, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1424usize)) }
    }
    #[doc = "Second last NFCID1 part (7 or 10 bytes ID)"]
    #[inline(always)]
    pub fn nfcid1_2nd_last(self) -> crate::common::Reg<regs::Nfcid12ndLast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1428usize)) }
    }
    #[doc = "Third last NFCID1 part (10 bytes ID)"]
    #[inline(always)]
    pub fn nfcid1_3rd_last(self) -> crate::common::Reg<regs::Nfcid13rdLast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1432usize)) }
    }
    #[doc = "Controls the auto collision resolution function. This setting must be done before the NFCT peripheral is enabled."]
    #[inline(always)]
    pub fn autocolresconfig(self) -> crate::common::Reg<regs::Autocolresconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1436usize)) }
    }
    #[doc = "NFC-A SENS_RES auto-response settings"]
    #[inline(always)]
    pub fn sensres(self) -> crate::common::Reg<regs::Sensres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1440usize)) }
    }
    #[doc = "NFC-A SEL_RES auto-response settings"]
    #[inline(always)]
    pub fn selres(self) -> crate::common::Reg<regs::Selres, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1444usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxd(pub *mut u8);
unsafe impl Send for Rxd {}
unsafe impl Sync for Rxd {}
impl Rxd {
    #[doc = "Configuration of incoming frames"]
    #[inline(always)]
    pub fn frameconfig(self) -> crate::common::Reg<regs::RxdFrameconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Size of last incoming frame"]
    #[inline(always)]
    pub fn amount(self) -> crate::common::Reg<regs::RxdAmount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Framestatus(pub *mut u8);
unsafe impl Send for Framestatus {}
unsafe impl Sync for Framestatus {}
impl Framestatus {
    #[doc = "Result of last incoming frame"]
    #[inline(always)]
    pub fn rx(self) -> crate::common::Reg<regs::Rx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod regs;
pub mod vals;
