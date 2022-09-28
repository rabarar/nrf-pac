#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Whiteen(pub u8);
impl Whiteen {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pdustat(pub u8);
impl Pdustat {
    #[doc = "Payload less than PCNF1.MAXLEN"]
    pub const LESSTHAN: Self = Self(0);
    #[doc = "Payload greater than PCNF1.MAXLEN"]
    pub const GREATERTHAN: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ena0(pub u8);
impl Ena0 {
    #[doc = "Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ena7(pub u8);
impl Ena7 {
    #[doc = "Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Crcstatus(pub u8);
impl Crcstatus {
    #[doc = "Packet received with CRC error"]
    pub const CRCERROR: Self = Self(0);
    #[doc = "Packet received with CRC ok"]
    pub const CRCOK: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Crcinc(pub u8);
impl Crcinc {
    #[doc = "LENGTH does not contain CRC"]
    pub const EXCLUDE: Self = Self(0);
    #[doc = "LENGTH includes CRC"]
    pub const INCLUDE: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cistat(pub u8);
impl Cistat {
    #[doc = "Frame is received at 125kbps"]
    pub const LR125KBIT: Self = Self(0);
    #[doc = "Frame is received at 500kbps"]
    pub const LR500KBIT: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Endian(pub u8);
impl Endian {
    #[doc = "Least significant bit on air first"]
    pub const LITTLE: Self = Self(0);
    #[doc = "Most significant bit on air first"]
    pub const BIG: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Len(pub u8);
impl Len {
    #[doc = "CRC length is zero and CRC calculation is disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "CRC length is one byte and CRC calculation is enabled"]
    pub const ONE: Self = Self(0x01);
    #[doc = "CRC length is two bytes and CRC calculation is enabled"]
    pub const TWO: Self = Self(0x02);
    #[doc = "CRC length is three bytes and CRC calculation is enabled"]
    pub const THREE: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ena5(pub u8);
impl Ena5 {
    #[doc = "Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Addr6(pub u8);
impl Addr6 {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Addr4(pub u8);
impl Addr4 {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Addr5(pub u8);
impl Addr5 {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ccamode(pub u8);
impl Ccamode {
    #[doc = "Energy above threshold"]
    pub const EDMODE: Self = Self(0);
    #[doc = "Carrier seen"]
    pub const CARRIERMODE: Self = Self(0x01);
    #[doc = "Energy above threshold AND carrier seen"]
    pub const CARRIERANDEDMODE: Self = Self(0x02);
    #[doc = "Energy above threshold OR carrier seen"]
    pub const CARRIEROREDMODE: Self = Self(0x03);
    #[doc = "Energy above threshold test mode that will abort when first ED measurement over threshold is seen. No averaging."]
    pub const EDMODETEST1: Self = Self(0x04);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Addr1(pub u8);
impl Addr1 {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ena3(pub u8);
impl Ena3 {
    #[doc = "Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dtx(pub u8);
impl Dtx {
    #[doc = "Transmit '1'"]
    pub const B1: Self = Self(0);
    #[doc = "Transmit '0'"]
    pub const B0: Self = Self(0x01);
    #[doc = "Transmit center frequency"]
    pub const CENTER: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Addr2(pub u8);
impl Addr2 {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Map(pub u8);
impl Map {
    #[doc = "Channel map between 2400 MHZ .. 2500 MHz"]
    pub const DEFAULT: Self = Self(0);
    #[doc = "Channel map between 2360 MHZ .. 2460 MHz"]
    pub const LOW: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ena1(pub u8);
impl Ena1 {
    #[doc = "Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Addr3(pub u8);
impl Addr3 {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Txpower(pub u8);
impl Txpower {
    #[doc = "0 dBm"]
    pub const _0DBM: Self = Self(0);
    #[doc = "+2 dBm"]
    pub const POS2DBM: Self = Self(0x02);
    #[doc = "+3 dBm"]
    pub const POS3DBM: Self = Self(0x03);
    #[doc = "+4 dBm"]
    pub const POS4DBM: Self = Self(0x04);
    #[doc = "+5 dBm"]
    pub const POS5DBM: Self = Self(0x05);
    #[doc = "+6 dBm"]
    pub const POS6DBM: Self = Self(0x06);
    #[doc = "+7 dBm"]
    pub const POS7DBM: Self = Self(0x07);
    #[doc = "+8 dBm"]
    pub const POS8DBM: Self = Self(0x08);
    #[doc = "-40 dBm"]
    pub const NEG40DBM: Self = Self(0xd8);
    #[doc = "-20 dBm"]
    pub const NEG20DBM: Self = Self(0xec);
    #[doc = "-16 dBm"]
    pub const NEG16DBM: Self = Self(0xf0);
    #[doc = "-12 dBm"]
    pub const NEG12DBM: Self = Self(0xf4);
    #[doc = "-8 dBm"]
    pub const NEG8DBM: Self = Self(0xf8);
    #[doc = "-4 dBm"]
    pub const NEG4DBM: Self = Self(0xfc);
    #[doc = "Deprecated enumerator - -40 dBm"]
    pub const NEG30DBM: Self = Self(0xff);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Skipaddr(pub u8);
impl Skipaddr {
    #[doc = "CRC calculation includes address field"]
    pub const INCLUDE: Self = Self(0);
    #[doc = "CRC calculation does not include address field. The CRC calculation will start at the first byte after the address."]
    pub const SKIP: Self = Self(0x01);
    #[doc = "CRC calculation as per 802.15.4 standard. Starting at first byte after length field."]
    pub const IEEE802154: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mode(pub u8);
impl Mode {
    #[doc = "1 Mbit/s Nordic proprietary radio mode"]
    pub const NRF_1MBIT: Self = Self(0);
    #[doc = "2 Mbit/s Nordic proprietary radio mode"]
    pub const NRF_2MBIT: Self = Self(0x01);
    #[doc = "1 Mbit/s BLE"]
    pub const BLE_1MBIT: Self = Self(0x03);
    #[doc = "2 Mbit/s BLE"]
    pub const BLE_2MBIT: Self = Self(0x04);
    #[doc = "Long range 125 kbit/s TX, 125 kbit/s and 500 kbit/s RX"]
    pub const BLE_LR125KBIT: Self = Self(0x05);
    #[doc = "Long range 500 kbit/s TX, 125 kbit/s and 500 kbit/s RX"]
    pub const BLE_LR500KBIT: Self = Self(0x06);
    #[doc = "IEEE 802.15.4-2006 250 kbit/s"]
    pub const IEEE802154_250KBIT: Self = Self(0x0f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Power(pub u8);
impl Power {
    #[doc = "Peripheral is powered off"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Peripheral is powered on"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ena6(pub u8);
impl Ena6 {
    #[doc = "Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Plen(pub u8);
impl Plen {
    #[doc = "8-bit preamble"]
    pub const _8BIT: Self = Self(0);
    #[doc = "16-bit preamble"]
    pub const _16BIT: Self = Self(0x01);
    #[doc = "32-bit zero preamble - used for IEEE 802.15.4"]
    pub const _32BITZERO: Self = Self(0x02);
    #[doc = "Preamble - used for BLE long range"]
    pub const LONGRANGE: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct S1incl(pub u8);
impl S1incl {
    #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
    pub const AUTOMATIC: Self = Self(0);
    #[doc = "Always include S1 field in RAM independent of S1LEN"]
    pub const INCLUDE: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Addr7(pub u8);
impl Addr7 {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Addr0(pub u8);
impl Addr0 {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ena4(pub u8);
impl Ena4 {
    #[doc = "Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct State(pub u8);
impl State {
    #[doc = "RADIO is in the Disabled state"]
    pub const DISABLED: Self = Self(0);
    #[doc = "RADIO is in the RXRU state"]
    pub const RXRU: Self = Self(0x01);
    #[doc = "RADIO is in the RXIDLE state"]
    pub const RXIDLE: Self = Self(0x02);
    #[doc = "RADIO is in the RX state"]
    pub const RX: Self = Self(0x03);
    #[doc = "RADIO is in the RXDISABLED state"]
    pub const RXDISABLE: Self = Self(0x04);
    #[doc = "RADIO is in the TXRU state"]
    pub const TXRU: Self = Self(0x09);
    #[doc = "RADIO is in the TXIDLE state"]
    pub const TXIDLE: Self = Self(0x0a);
    #[doc = "RADIO is in the TX state"]
    pub const TX: Self = Self(0x0b);
    #[doc = "RADIO is in the TXDISABLED state"]
    pub const TXDISABLE: Self = Self(0x0c);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ena2(pub u8);
impl Ena2 {
    #[doc = "Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ru(pub u8);
impl Ru {
    #[doc = "Default ramp-up time (tRXEN), compatible with firmware written for nRF51"]
    pub const DEFAULT: Self = Self(0);
    #[doc = "Fast ramp-up (tRXEN,FAST), see electrical specification for more information"]
    pub const FAST: Self = Self(0x01);
}
