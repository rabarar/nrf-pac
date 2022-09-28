#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hfxodebounce(pub u8);
impl Hfxodebounce {
    #[doc = "256 us debounce time. Recommended for TSX-3225, FA-20H and FA-128 crystals."]
    pub const DB256US: Self = Self(0x10);
    #[doc = "1024 us debounce time. Recommended for NX1612AA and NX1210AB crystals."]
    pub const DB1024US: Self = Self(0x40);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Traceportspeed(pub u8);
impl Traceportspeed {
    #[doc = "32 MHz trace port clock (TRACECLK = 16 MHz)"]
    pub const _32MHZ: Self = Self(0);
    #[doc = "16 MHz trace port clock (TRACECLK = 8 MHz)"]
    pub const _16MHZ: Self = Self(0x01);
    #[doc = "8 MHz trace port clock (TRACECLK = 4 MHz)"]
    pub const _8MHZ: Self = Self(0x02);
    #[doc = "4 MHz trace port clock (TRACECLK = 2 MHz)"]
    pub const _4MHZ: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct External(pub u8);
impl External {
    #[doc = "Disable external source (use with Xtal)"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable use of external source instead of Xtal (SRC needs to be set to Xtal)"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HfclkstatSrc(pub u8);
impl HfclkstatSrc {
    #[doc = "64 MHz internal oscillator (HFINT)"]
    pub const RC: Self = Self(0);
    #[doc = "64 MHz crystal oscillator (HFXO)"]
    pub const XTAL: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mode(pub u8);
impl Mode {
    #[doc = "Normal mode"]
    pub const NORMAL: Self = Self(0);
    #[doc = "Ultra-low power mode (ULP)"]
    pub const ULP: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LfclkstatSrc(pub u8);
impl LfclkstatSrc {
    #[doc = "32.768 kHz RC oscillator (LFRC)"]
    pub const RC: Self = Self(0);
    #[doc = "32.768 kHz crystal oscillator (LFXO)"]
    pub const XTAL: Self = Self(0x01);
    #[doc = "32.768 kHz synthesized from HFCLK (LFSYNT)"]
    pub const SYNTH: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bypass(pub u8);
impl Bypass {
    #[doc = "Disable (use with Xtal or low-swing external source)"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable (use with rail-to-rail external source)"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HfclkrunStatus(pub u8);
impl HfclkrunStatus {
    #[doc = "Task not triggered"]
    pub const NOTTRIGGERED: Self = Self(0);
    #[doc = "Task triggered"]
    pub const TRIGGERED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LfclksrcSrc(pub u8);
impl LfclksrcSrc {
    #[doc = "32.768 kHz RC oscillator (LFRC)"]
    pub const RC: Self = Self(0);
    #[doc = "32.768 kHz crystal oscillator (LFXO)"]
    pub const XTAL: Self = Self(0x01);
    #[doc = "32.768 kHz synthesized from HFCLK (LFSYNT)"]
    pub const SYNTH: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct HfclkstatState(pub u8);
impl HfclkstatState {
    #[doc = "HFCLK not running"]
    pub const NOTRUNNING: Self = Self(0);
    #[doc = "HFCLK running"]
    pub const RUNNING: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LfclkstatState(pub u8);
impl LfclkstatState {
    #[doc = "LFCLK not running"]
    pub const NOTRUNNING: Self = Self(0);
    #[doc = "LFCLK running"]
    pub const RUNNING: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tracemux(pub u8);
impl Tracemux {
    #[doc = "No trace signals routed to pins. All pins can be used as regular GPIOs."]
    pub const GPIO: Self = Self(0);
    #[doc = "SWO trace signal routed to pin. Remaining pins can be used as regular GPIOs."]
    pub const SERIAL: Self = Self(0x01);
    #[doc = "All trace signals (TRACECLK and TRACEDATA\\[n\\]) routed to pins."]
    pub const PARALLEL: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LfclkrunStatus(pub u8);
impl LfclkrunStatus {
    #[doc = "Task not triggered"]
    pub const NOTTRIGGERED: Self = Self(0);
    #[doc = "Task triggered"]
    pub const TRIGGERED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LfclksrccopySrc(pub u8);
impl LfclksrccopySrc {
    #[doc = "32.768 kHz RC oscillator (LFRC)"]
    pub const RC: Self = Self(0);
    #[doc = "32.768 kHz crystal oscillator (LFXO)"]
    pub const XTAL: Self = Self(0x01);
    #[doc = "32.768 kHz synthesized from HFCLK (LFSYNT)"]
    pub const SYNTH: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LfrcmodeStatus(pub u8);
impl LfrcmodeStatus {
    #[doc = "Normal mode"]
    pub const NORMAL: Self = Self(0);
    #[doc = "Ultra-low power mode (ULP)"]
    pub const ULP: Self = Self(0x01);
}
