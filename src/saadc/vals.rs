#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Oversample(pub u8);
impl Oversample {
    #[doc = "Bypass oversampling"]
    pub const BYPASS: Self = Self(0);
    #[doc = "Oversample 2x"]
    pub const OVER2X: Self = Self(0x01);
    #[doc = "Oversample 4x"]
    pub const OVER4X: Self = Self(0x02);
    #[doc = "Oversample 8x"]
    pub const OVER8X: Self = Self(0x03);
    #[doc = "Oversample 16x"]
    pub const OVER16X: Self = Self(0x04);
    #[doc = "Oversample 32x"]
    pub const OVER32X: Self = Self(0x05);
    #[doc = "Oversample 64x"]
    pub const OVER64X: Self = Self(0x06);
    #[doc = "Oversample 128x"]
    pub const OVER128X: Self = Self(0x07);
    #[doc = "Oversample 256x"]
    pub const OVER256X: Self = Self(0x08);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u8);
impl Enable {
    #[doc = "Disable SAADC"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable SAADC"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Refsel(pub u8);
impl Refsel {
    #[doc = "Internal reference (0.6 V)"]
    pub const INTERNAL: Self = Self(0);
    #[doc = "VDD/4 as reference"]
    pub const VDD1_4: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Resp(pub u8);
impl Resp {
    #[doc = "Bypass resistor ladder"]
    pub const BYPASS: Self = Self(0);
    #[doc = "Pull-down to GND"]
    pub const PULLDOWN: Self = Self(0x01);
    #[doc = "Pull-up to VDD"]
    pub const PULLUP: Self = Self(0x02);
    #[doc = "Set input at VDD/2"]
    pub const VDD1_2: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Gain(pub u8);
impl Gain {
    #[doc = "1/6"]
    pub const GAIN1_6: Self = Self(0);
    #[doc = "1/5"]
    pub const GAIN1_5: Self = Self(0x01);
    #[doc = "1/4"]
    pub const GAIN1_4: Self = Self(0x02);
    #[doc = "1/3"]
    pub const GAIN1_3: Self = Self(0x03);
    #[doc = "1/2"]
    pub const GAIN1_2: Self = Self(0x04);
    #[doc = "1"]
    pub const GAIN1: Self = Self(0x05);
    #[doc = "2"]
    pub const GAIN2: Self = Self(0x06);
    #[doc = "4"]
    pub const GAIN4: Self = Self(0x07);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Val(pub u8);
impl Val {
    #[doc = "8 bits"]
    pub const _8BIT: Self = Self(0);
    #[doc = "10 bits"]
    pub const _10BIT: Self = Self(0x01);
    #[doc = "12 bits"]
    pub const _12BIT: Self = Self(0x02);
    #[doc = "14 bits"]
    pub const _14BIT: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pseln(pub u8);
impl Pseln {
    #[doc = "Not connected"]
    pub const NC: Self = Self(0);
    #[doc = "AIN0"]
    pub const ANALOGINPUT0: Self = Self(0x01);
    #[doc = "AIN1"]
    pub const ANALOGINPUT1: Self = Self(0x02);
    #[doc = "AIN2"]
    pub const ANALOGINPUT2: Self = Self(0x03);
    #[doc = "AIN3"]
    pub const ANALOGINPUT3: Self = Self(0x04);
    #[doc = "AIN4"]
    pub const ANALOGINPUT4: Self = Self(0x05);
    #[doc = "AIN5"]
    pub const ANALOGINPUT5: Self = Self(0x06);
    #[doc = "AIN6"]
    pub const ANALOGINPUT6: Self = Self(0x07);
    #[doc = "AIN7"]
    pub const ANALOGINPUT7: Self = Self(0x08);
    #[doc = "VDD"]
    pub const VDD: Self = Self(0x09);
    #[doc = "VDDH/5"]
    pub const VDDHDIV5: Self = Self(0x0d);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tacq(pub u8);
impl Tacq {
    #[doc = "3 us"]
    pub const _3US: Self = Self(0);
    #[doc = "5 us"]
    pub const _5US: Self = Self(0x01);
    #[doc = "10 us"]
    pub const _10US: Self = Self(0x02);
    #[doc = "15 us"]
    pub const _15US: Self = Self(0x03);
    #[doc = "20 us"]
    pub const _20US: Self = Self(0x04);
    #[doc = "40 us"]
    pub const _40US: Self = Self(0x05);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Status(pub u8);
impl Status {
    #[doc = "SAADC is ready. No on-going conversions."]
    pub const READY: Self = Self(0);
    #[doc = "SAADC is busy. Conversion in progress."]
    pub const BUSY: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Burst(pub u8);
impl Burst {
    #[doc = "Burst mode is disabled (normal operation)"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Burst mode is enabled. SAADC takes 2^OVERSAMPLE number of samples as fast as it can, and sends the average to Data RAM."]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SamplerateMode(pub u8);
impl SamplerateMode {
    #[doc = "Rate is controlled from SAMPLE task"]
    pub const TASK: Self = Self(0);
    #[doc = "Rate is controlled from local timer (use CC to control the rate)"]
    pub const TIMERS: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConfigMode(pub u8);
impl ConfigMode {
    #[doc = "Single-ended, PSELN will be ignored, negative input to SAADC shorted to GND"]
    pub const SE: Self = Self(0);
    #[doc = "Differential"]
    pub const DIFF: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pselp(pub u8);
impl Pselp {
    #[doc = "Not connected"]
    pub const NC: Self = Self(0);
    #[doc = "AIN0"]
    pub const ANALOGINPUT0: Self = Self(0x01);
    #[doc = "AIN1"]
    pub const ANALOGINPUT1: Self = Self(0x02);
    #[doc = "AIN2"]
    pub const ANALOGINPUT2: Self = Self(0x03);
    #[doc = "AIN3"]
    pub const ANALOGINPUT3: Self = Self(0x04);
    #[doc = "AIN4"]
    pub const ANALOGINPUT4: Self = Self(0x05);
    #[doc = "AIN5"]
    pub const ANALOGINPUT5: Self = Self(0x06);
    #[doc = "AIN6"]
    pub const ANALOGINPUT6: Self = Self(0x07);
    #[doc = "AIN7"]
    pub const ANALOGINPUT7: Self = Self(0x08);
    #[doc = "VDD"]
    pub const VDD: Self = Self(0x09);
    #[doc = "VDDH/5"]
    pub const VDDHDIV5: Self = Self(0x0d);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Resn(pub u8);
impl Resn {
    #[doc = "Bypass resistor ladder"]
    pub const BYPASS: Self = Self(0);
    #[doc = "Pull-down to GND"]
    pub const PULLDOWN: Self = Self(0x01);
    #[doc = "Pull-up to VDD"]
    pub const PULLUP: Self = Self(0x02);
    #[doc = "Set input at VDD/2"]
    pub const VDD1_2: Self = Self(0x03);
}
