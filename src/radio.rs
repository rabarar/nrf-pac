#[doc = "2.4 GHz radio"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Radio(pub *mut u8);
unsafe impl Send for Radio {}
unsafe impl Sync for Radio {}
impl Radio {
    #[doc = "Enable RADIO in TX mode"]
    #[inline(always)]
    pub fn tasks_txen(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Enable RADIO in RX mode"]
    #[inline(always)]
    pub fn tasks_rxen(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Start RADIO"]
    #[inline(always)]
    pub fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Stop RADIO"]
    #[inline(always)]
    pub fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Disable RADIO"]
    #[inline(always)]
    pub fn tasks_disable(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Start the RSSI and take one single sample of the receive signal strength"]
    #[inline(always)]
    pub fn tasks_rssistart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Stop the RSSI measurement"]
    #[inline(always)]
    pub fn tasks_rssistop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Start the bit counter"]
    #[inline(always)]
    pub fn tasks_bcstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Stop the bit counter"]
    #[inline(always)]
    pub fn tasks_bcstop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Start the energy detect measurement used in IEEE 802.15.4 mode"]
    #[inline(always)]
    pub fn tasks_edstart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Stop the energy detect measurement"]
    #[inline(always)]
    pub fn tasks_edstop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Start the clear channel assessment used in IEEE 802.15.4 mode"]
    #[inline(always)]
    pub fn tasks_ccastart(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Stop the clear channel assessment"]
    #[inline(always)]
    pub fn tasks_ccastop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "RADIO has ramped up and is ready to be started"]
    #[inline(always)]
    pub fn events_ready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "Address sent or received"]
    #[inline(always)]
    pub fn events_address(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "Packet payload sent or received"]
    #[inline(always)]
    pub fn events_payload(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }
    #[doc = "Packet sent or received"]
    #[inline(always)]
    pub fn events_end(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(268usize)) }
    }
    #[doc = "RADIO has been disabled"]
    #[inline(always)]
    pub fn events_disabled(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
    }
    #[doc = "A device address match occurred on the last received packet"]
    #[inline(always)]
    pub fn events_devmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(276usize)) }
    }
    #[doc = "No device address match occurred on the last received packet"]
    #[inline(always)]
    pub fn events_devmiss(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(280usize)) }
    }
    #[doc = "Sampling of receive signal strength complete"]
    #[inline(always)]
    pub fn events_rssiend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(284usize)) }
    }
    #[doc = "Bit counter reached bit count value"]
    #[inline(always)]
    pub fn events_bcmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }
    #[doc = "Packet received with CRC ok"]
    #[inline(always)]
    pub fn events_crcok(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(304usize)) }
    }
    #[doc = "Packet received with CRC error"]
    #[inline(always)]
    pub fn events_crcerror(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(308usize)) }
    }
    #[doc = "IEEE 802.15.4 length field received"]
    #[inline(always)]
    pub fn events_framestart(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(312usize)) }
    }
    #[doc = "Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register."]
    #[inline(always)]
    pub fn events_edend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(316usize)) }
    }
    #[doc = "The sampling of energy detection has stopped"]
    #[inline(always)]
    pub fn events_edstopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(320usize)) }
    }
    #[doc = "Wireless medium in idle - clear to send"]
    #[inline(always)]
    pub fn events_ccaidle(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(324usize)) }
    }
    #[doc = "Wireless medium busy - do not send"]
    #[inline(always)]
    pub fn events_ccabusy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(328usize)) }
    }
    #[doc = "The CCA has stopped"]
    #[inline(always)]
    pub fn events_ccastopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(332usize)) }
    }
    #[doc = "Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
    #[inline(always)]
    pub fn events_rateboost(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(336usize)) }
    }
    #[doc = "RADIO has ramped up and is ready to be started TX path"]
    #[inline(always)]
    pub fn events_txready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(340usize)) }
    }
    #[doc = "RADIO has ramped up and is ready to be started RX path"]
    #[inline(always)]
    pub fn events_rxready(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(344usize)) }
    }
    #[doc = "MAC header match found"]
    #[inline(always)]
    pub fn events_mhrmatch(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(348usize)) }
    }
    #[doc = "Generated in Ble_LR125Kbit, Ble_LR500Kbit and BleIeee802154_250Kbit modes when last bit is sent on air."]
    #[inline(always)]
    pub fn events_phyend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(364usize)) }
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
    #[doc = "CRC status"]
    #[inline(always)]
    pub fn crcstatus(self) -> crate::common::Reg<regs::Crcstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1024usize)) }
    }
    #[doc = "Received address"]
    #[inline(always)]
    pub fn rxmatch(self) -> crate::common::Reg<regs::Rxmatch, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1032usize)) }
    }
    #[doc = "CRC field of previously received packet"]
    #[inline(always)]
    pub fn rxcrc(self) -> crate::common::Reg<regs::Rxcrc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1036usize)) }
    }
    #[doc = "Device address match index"]
    #[inline(always)]
    pub fn dai(self) -> crate::common::Reg<regs::Dai, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1040usize)) }
    }
    #[doc = "Payload status"]
    #[inline(always)]
    pub fn pdustat(self) -> crate::common::Reg<regs::Pdustat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1044usize)) }
    }
    #[doc = "Packet pointer"]
    #[inline(always)]
    pub fn packetptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "Frequency"]
    #[inline(always)]
    pub fn frequency(self) -> crate::common::Reg<regs::Frequency, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "Output power"]
    #[inline(always)]
    pub fn txpower(self) -> crate::common::Reg<regs::Txpower, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1292usize)) }
    }
    #[doc = "Data rate and modulation"]
    #[inline(always)]
    pub fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize)) }
    }
    #[doc = "Packet configuration register 0"]
    #[inline(always)]
    pub fn pcnf0(self) -> crate::common::Reg<regs::Pcnf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1300usize)) }
    }
    #[doc = "Packet configuration register 1"]
    #[inline(always)]
    pub fn pcnf1(self) -> crate::common::Reg<regs::Pcnf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1304usize)) }
    }
    #[doc = "Base address 0"]
    #[inline(always)]
    pub fn base0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1308usize)) }
    }
    #[doc = "Base address 1"]
    #[inline(always)]
    pub fn base1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1312usize)) }
    }
    #[doc = "Prefixes bytes for logical addresses 0-3"]
    #[inline(always)]
    pub fn prefix0(self) -> crate::common::Reg<regs::Prefix0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1316usize)) }
    }
    #[doc = "Prefixes bytes for logical addresses 4-7"]
    #[inline(always)]
    pub fn prefix1(self) -> crate::common::Reg<regs::Prefix1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1320usize)) }
    }
    #[doc = "Transmit address select"]
    #[inline(always)]
    pub fn txaddress(self) -> crate::common::Reg<regs::Txaddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1324usize)) }
    }
    #[doc = "Receive address select"]
    #[inline(always)]
    pub fn rxaddresses(self) -> crate::common::Reg<regs::Rxaddresses, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1328usize)) }
    }
    #[doc = "CRC configuration"]
    #[inline(always)]
    pub fn crccnf(self) -> crate::common::Reg<regs::Crccnf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1332usize)) }
    }
    #[doc = "CRC polynomial"]
    #[inline(always)]
    pub fn crcpoly(self) -> crate::common::Reg<regs::Crcpoly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1336usize)) }
    }
    #[doc = "CRC initial value"]
    #[inline(always)]
    pub fn crcinit(self) -> crate::common::Reg<regs::Crcinit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1340usize)) }
    }
    #[doc = "Interframe spacing in us"]
    #[inline(always)]
    pub fn tifs(self) -> crate::common::Reg<regs::Tifs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1348usize)) }
    }
    #[doc = "RSSI sample"]
    #[inline(always)]
    pub fn rssisample(self) -> crate::common::Reg<regs::Rssisample, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1352usize)) }
    }
    #[doc = "Current radio state"]
    #[inline(always)]
    pub fn state(self) -> crate::common::Reg<regs::State, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1360usize)) }
    }
    #[doc = "Data whitening initial value"]
    #[inline(always)]
    pub fn datawhiteiv(self) -> crate::common::Reg<regs::Datawhiteiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1364usize)) }
    }
    #[doc = "Bit counter compare"]
    #[inline(always)]
    pub fn bcc(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1376usize)) }
    }
    #[doc = "Description collection\\[n\\]: Device address base segment n"]
    #[inline(always)]
    pub fn dab(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(1536usize + n * 4usize)) }
    }
    #[doc = "Description collection\\[n\\]: Device address prefix n"]
    #[inline(always)]
    pub fn dap(self, n: usize) -> crate::common::Reg<regs::Dap, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(1568usize + n * 4usize)) }
    }
    #[doc = "Device address match configuration"]
    #[inline(always)]
    pub fn dacnf(self) -> crate::common::Reg<regs::Dacnf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1600usize)) }
    }
    #[doc = "Search pattern configuration"]
    #[inline(always)]
    pub fn mhrmatchconf(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1604usize)) }
    }
    #[doc = "Pattern mask"]
    #[inline(always)]
    pub fn mhrmatchmas(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1608usize)) }
    }
    #[doc = "Radio mode configuration register 0"]
    #[inline(always)]
    pub fn modecnf0(self) -> crate::common::Reg<regs::Modecnf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1616usize)) }
    }
    #[doc = "IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub fn sfd(self) -> crate::common::Reg<regs::Sfd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1632usize)) }
    }
    #[doc = "IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    pub fn edcnt(self) -> crate::common::Reg<regs::Edcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1636usize)) }
    }
    #[doc = "IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub fn edsample(self) -> crate::common::Reg<regs::Edsample, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1640usize)) }
    }
    #[doc = "IEEE 802.15.4 clear channel assessment control"]
    #[inline(always)]
    pub fn ccactrl(self) -> crate::common::Reg<regs::Ccactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1644usize)) }
    }
    #[doc = "Peripheral power control"]
    #[inline(always)]
    pub fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4092usize)) }
    }
}
pub mod regs;
pub mod vals;
