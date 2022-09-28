#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Operation(pub u8);
impl Operation {
    #[doc = "Sample and store one pair (Left + Right) of 16bit samples per RAM word R=\\[31:16\\]; L=\\[15:0\\]"]
    pub const STEREO: Self = Self(0);
    #[doc = "Sample and store two successive Left samples (16 bit each) per RAM word L1=\\[31:16\\]; L0=\\[15:0\\]"]
    pub const MONO: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gainl(pub u8);
impl Gainl {
    #[doc = "-20dB gain adjustment (minimum)"]
    pub const MINGAIN: Self = Self(0);
    #[doc = "0dB gain adjustment"]
    pub const DEFAULTGAIN: Self = Self(0x28);
    #[doc = "+20dB gain adjustment (maximum)"]
    pub const MAXGAIN: Self = Self(0x50);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DinConnect(pub u8);
impl DinConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ClkConnect(pub u8);
impl ClkConnect {
    #[doc = "Connect"]
    pub const CONNECTED: Self = Self(0);
    #[doc = "Disconnect"]
    pub const DISCONNECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u8);
impl Enable {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Edge(pub u8);
impl Edge {
    #[doc = "Left (or mono) is sampled on falling edge of PDM_CLK"]
    pub const LEFTFALLING: Self = Self(0);
    #[doc = "Left (or mono) is sampled on rising edge of PDM_CLK"]
    pub const LEFTRISING: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Freq(pub u32);
impl Freq {
    #[doc = "PDM_CLK = 32 MHz / 32 = 1.000 MHz"]
    pub const _1000K: Self = Self(0x0800_0000);
    #[doc = "PDM_CLK = 32 MHz / 31 = 1.032 MHz. Nominal clock for RATIO=Ratio64."]
    pub const DEFAULT: Self = Self(0x0840_0000);
    #[doc = "PDM_CLK = 32 MHz / 30 = 1.067 MHz"]
    pub const _1067K: Self = Self(0x0880_0000);
    #[doc = "PDM_CLK = 32 MHz / 26 = 1.231 MHz"]
    pub const _1231K: Self = Self(0x0980_0000);
    #[doc = "PDM_CLK = 32 MHz / 25 = 1.280 MHz. Nominal clock for RATIO=Ratio80."]
    pub const _1280K: Self = Self(0x0a00_0000);
    #[doc = "PDM_CLK = 32 MHz / 24 = 1.333 MHz"]
    pub const _1333K: Self = Self(0x0a80_0000);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gainr(pub u8);
impl Gainr {
    #[doc = "-20dB gain adjustment (minimum)"]
    pub const MINGAIN: Self = Self(0);
    #[doc = "0dB gain adjustment"]
    pub const DEFAULTGAIN: Self = Self(0x28);
    #[doc = "+20dB gain adjustment (maximum)"]
    pub const MAXGAIN: Self = Self(0x50);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ratio(pub u8);
impl Ratio {
    #[doc = "Ratio of 64"]
    pub const RATIO64: Self = Self(0);
    #[doc = "Ratio of 80"]
    pub const RATIO80: Self = Self(0x01);
}
