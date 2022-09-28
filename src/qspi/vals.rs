#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Length(pub u8);
impl Length {
    #[doc = "Send opcode only."]
    pub const _1B: Self = Self(0x01);
    #[doc = "Send opcode, CINSTRDAT0.BYTE0."]
    pub const _2B: Self = Self(0x02);
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE1."]
    pub const _3B: Self = Self(0x03);
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE2."]
    pub const _4B: Self = Self(0x04);
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE3."]
    pub const _5B: Self = Self(0x05);
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE4."]
    pub const _6B: Self = Self(0x06);
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE5."]
    pub const _7B: Self = Self(0x07);
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE6."]
    pub const _8B: Self = Self(0x08);
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE7."]
    pub const _9B: Self = Self(0x09);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SckConnect(pub u8);
impl SckConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct StatusReady(pub u8);
impl StatusReady {
    #[doc = "QSPI peripheral is busy. It is not allowed to trigger any new tasks, writing custom instructions or enter/exit DPM."]
    pub const BUSY: Self = Self(0);
    #[doc = "QSPI peripheral is ready. It is allowed to trigger new tasks, writing custom instructions or enter/exit DPM."]
    pub const READY: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ppsize(pub u8);
impl Ppsize {
    #[doc = "256 bytes."]
    pub const _256BYTES: Self = Self(0);
    #[doc = "512 bytes."]
    pub const _512BYTES: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dpm(pub u8);
impl Dpm {
    #[doc = "External flash is not in DPM."]
    pub const DISABLED: Self = Self(0);
    #[doc = "External flash is in DPM."]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Spimode(pub u8);
impl Spimode {
    #[doc = "Mode 0: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 0 (CPOL=0, CPHA=0)."]
    pub const MODE0: Self = Self(0);
    #[doc = "Mode 3: Data are captured on the clock falling edge and data is output on a rising edge. Base level of clock is 1 (CPOL=1, CPHA=1)."]
    pub const MODE3: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u8);
impl Enable {
    #[doc = "Disable QSPI"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable QSPI"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lfen(pub u8);
impl Lfen {
    #[doc = "Long frame mode disabled"]
    pub const DISABLE: Self = Self(0);
    #[doc = "Long frame mode enabled"]
    pub const ENABLE: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Addrmode(pub u8);
impl Addrmode {
    #[doc = "24-bit addressing."]
    pub const _24BIT: Self = Self(0);
    #[doc = "32-bit addressing."]
    pub const _32BIT: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mode(pub u8);
impl Mode {
    #[doc = "Do not send any instruction."]
    pub const NOINSTR: Self = Self(0);
    #[doc = "Send opcode."]
    pub const OPCODE: Self = Self(0x01);
    #[doc = "Send opcode, byte0."]
    pub const OPBYTE0: Self = Self(0x02);
    #[doc = "Send opcode, byte0, byte1."]
    pub const ALL: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Io1connect(pub u8);
impl Io1connect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Readoc(pub u8);
impl Readoc {
    #[doc = "Single data line SPI. FAST_READ (opcode 0x0B)."]
    pub const FASTREAD: Self = Self(0);
    #[doc = "Dual data line SPI. READ2O (opcode 0x3B)."]
    pub const READ2O: Self = Self(0x01);
    #[doc = "Dual data line SPI. READ2IO (opcode 0xBB)."]
    pub const READ2IO: Self = Self(0x02);
    #[doc = "Quad data line SPI. READ4O (opcode 0x6B)."]
    pub const READ4O: Self = Self(0x03);
    #[doc = "Quad data line SPI. READ4IO (opcode 0xEB)."]
    pub const READ4IO: Self = Self(0x04);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dpmen(pub u8);
impl Dpmen {
    #[doc = "Exit DPM."]
    pub const EXIT: Self = Self(0);
    #[doc = "Enter DPM."]
    pub const ENTER: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CinstrconfWren(pub u8);
impl CinstrconfWren {
    #[doc = "Do not send WREN."]
    pub const DISABLE: Self = Self(0);
    #[doc = "Send WREN."]
    pub const ENABLE: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dpmenable(pub u8);
impl Dpmenable {
    #[doc = "Disable DPM feature."]
    pub const DISABLE: Self = Self(0);
    #[doc = "Enable DPM feature."]
    pub const ENABLE: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Io3connect(pub u8);
impl Io3connect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Len(pub u8);
impl Len {
    #[doc = "Erase 4 kB block (flash command 0x20)"]
    pub const _4KB: Self = Self(0);
    #[doc = "Erase 64 kB block (flash command 0xD8)"]
    pub const _64KB: Self = Self(0x01);
    #[doc = "Erase all (flash command 0xC7)"]
    pub const ALL: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Writeoc(pub u8);
impl Writeoc {
    #[doc = "Single data line SPI. PP (opcode 0x02)."]
    pub const PP: Self = Self(0);
    #[doc = "Dual data line SPI. PP2O (opcode 0xA2)."]
    pub const PP2O: Self = Self(0x01);
    #[doc = "Quad data line SPI. PP4O (opcode 0x32)."]
    pub const PP4O: Self = Self(0x02);
    #[doc = "Quad data line SPI. PP4IO (opcode 0x38)."]
    pub const PP4IO: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AddrconfWipwait(pub u8);
impl AddrconfWipwait {
    #[doc = "No wait."]
    pub const DISABLE: Self = Self(0);
    #[doc = "Wait."]
    pub const ENABLE: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AddrconfWren(pub u8);
impl AddrconfWren {
    #[doc = "Do not send WREN."]
    pub const DISABLE: Self = Self(0);
    #[doc = "Send WREN."]
    pub const ENABLE: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Io2connect(pub u8);
impl Io2connect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Io0connect(pub u8);
impl Io0connect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CsnConnect(pub u8);
impl CsnConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CinstrconfWipwait(pub u8);
impl CinstrconfWipwait {
    #[doc = "No wait."]
    pub const DISABLE: Self = Self(0);
    #[doc = "Wait."]
    pub const ENABLE: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lfstop(pub u8);
impl Lfstop {
    #[doc = "Stop"]
    pub const STOP: Self = Self(0x01);
}
