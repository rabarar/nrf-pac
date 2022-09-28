#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Refsel(pub u8);
impl Refsel {
    #[doc = "VREF = internal 1.2 V reference (VDD &gt;= 1.7 V)"]
    pub const INT1V2: Self = Self(0);
    #[doc = "VREF = internal 1.8 V reference (VDD &gt;= VREF + 0.2 V)"]
    pub const INT1V8: Self = Self(0x01);
    #[doc = "VREF = internal 2.4 V reference (VDD &gt;= VREF + 0.2 V)"]
    pub const INT2V4: Self = Self(0x02);
    #[doc = "VREF = VDD"]
    pub const VDD: Self = Self(0x04);
    #[doc = "VREF = AREF (VDD &gt;= VREF &gt;= AREFMIN)"]
    pub const AREF: Self = Self(0x05);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable(pub u8);
impl Enable {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Psel(pub u8);
impl Psel {
    #[doc = "AIN0 selected as analog input"]
    pub const ANALOGINPUT0: Self = Self(0);
    #[doc = "AIN1 selected as analog input"]
    pub const ANALOGINPUT1: Self = Self(0x01);
    #[doc = "AIN2 selected as analog input"]
    pub const ANALOGINPUT2: Self = Self(0x02);
    #[doc = "AIN3 selected as analog input"]
    pub const ANALOGINPUT3: Self = Self(0x03);
    #[doc = "AIN4 selected as analog input"]
    pub const ANALOGINPUT4: Self = Self(0x04);
    #[doc = "AIN5 selected as analog input"]
    pub const ANALOGINPUT5: Self = Self(0x05);
    #[doc = "AIN6 selected as analog input"]
    pub const ANALOGINPUT6: Self = Self(0x06);
    #[doc = "AIN7 selected as analog input"]
    pub const ANALOGINPUT7: Self = Self(0x07);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sp(pub u8);
impl Sp {
    #[doc = "Low-power mode"]
    pub const LOW: Self = Self(0);
    #[doc = "Normal mode"]
    pub const NORMAL: Self = Self(0x01);
    #[doc = "High-speed mode"]
    pub const HIGH: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Result(pub u8);
impl Result {
    #[doc = "Input voltage is below the threshold (VIN+ &lt; VIN-)"]
    pub const BELOW: Self = Self(0);
    #[doc = "Input voltage is above the threshold (VIN+ &gt; VIN-)"]
    pub const ABOVE: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Main(pub u8);
impl Main {
    #[doc = "Single-ended mode"]
    pub const SE: Self = Self(0);
    #[doc = "Differential mode"]
    pub const DIFF: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hyst(pub u8);
impl Hyst {
    #[doc = "Comparator hysteresis disabled"]
    pub const NOHYST: Self = Self(0);
    #[doc = "Comparator hysteresis enabled"]
    pub const HYST50MV: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Extrefsel(pub u8);
impl Extrefsel {
    #[doc = "Use AIN0 as external analog reference"]
    pub const ANALOGREFERENCE0: Self = Self(0);
    #[doc = "Use AIN1 as external analog reference"]
    pub const ANALOGREFERENCE1: Self = Self(0x01);
    #[doc = "Use AIN2 as external analog reference"]
    pub const ANALOGREFERENCE2: Self = Self(0x02);
    #[doc = "Use AIN3 as external analog reference"]
    pub const ANALOGREFERENCE3: Self = Self(0x03);
    #[doc = "Use AIN4 as external analog reference"]
    pub const ANALOGREFERENCE4: Self = Self(0x04);
    #[doc = "Use AIN5 as external analog reference"]
    pub const ANALOGREFERENCE5: Self = Self(0x05);
    #[doc = "Use AIN6 as external analog reference"]
    pub const ANALOGREFERENCE6: Self = Self(0x06);
    #[doc = "Use AIN7 as external analog reference"]
    pub const ANALOGREFERENCE7: Self = Self(0x07);
}
