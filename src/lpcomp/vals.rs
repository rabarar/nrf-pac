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
pub struct Extrefsel(pub u8);
impl Extrefsel {
    #[doc = "Use AIN0 as external analog reference"]
    pub const ANALOGREFERENCE0: Self = Self(0);
    #[doc = "Use AIN1 as external analog reference"]
    pub const ANALOGREFERENCE1: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hyst(pub u8);
impl Hyst {
    #[doc = "Comparator hysteresis disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Comparator hysteresis enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Anadetect(pub u8);
impl Anadetect {
    #[doc = "Generate ANADETECT on crossing, both upward crossing and downward crossing"]
    pub const CROSS: Self = Self(0);
    #[doc = "Generate ANADETECT on upward crossing only"]
    pub const UP: Self = Self(0x01);
    #[doc = "Generate ANADETECT on downward crossing only"]
    pub const DOWN: Self = Self(0x02);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Result(pub u8);
impl Result {
    #[doc = "Input voltage is below the reference threshold (VIN+ &lt; VIN-)."]
    pub const BELOW: Self = Self(0);
    #[doc = "Input voltage is above the reference threshold (VIN+ &gt; VIN-)."]
    pub const ABOVE: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Refsel(pub u8);
impl Refsel {
    #[doc = "VDD * 1/8 selected as reference"]
    pub const REF1_8VDD: Self = Self(0);
    #[doc = "VDD * 2/8 selected as reference"]
    pub const REF2_8VDD: Self = Self(0x01);
    #[doc = "VDD * 3/8 selected as reference"]
    pub const REF3_8VDD: Self = Self(0x02);
    #[doc = "VDD * 4/8 selected as reference"]
    pub const REF4_8VDD: Self = Self(0x03);
    #[doc = "VDD * 5/8 selected as reference"]
    pub const REF5_8VDD: Self = Self(0x04);
    #[doc = "VDD * 6/8 selected as reference"]
    pub const REF6_8VDD: Self = Self(0x05);
    #[doc = "VDD * 7/8 selected as reference"]
    pub const REF7_8VDD: Self = Self(0x06);
    #[doc = "External analog reference selected"]
    pub const AREF: Self = Self(0x07);
    #[doc = "VDD * 1/16 selected as reference"]
    pub const REF1_16VDD: Self = Self(0x08);
    #[doc = "VDD * 3/16 selected as reference"]
    pub const REF3_16VDD: Self = Self(0x09);
    #[doc = "VDD * 5/16 selected as reference"]
    pub const REF5_16VDD: Self = Self(0x0a);
    #[doc = "VDD * 7/16 selected as reference"]
    pub const REF7_16VDD: Self = Self(0x0b);
    #[doc = "VDD * 9/16 selected as reference"]
    pub const REF9_16VDD: Self = Self(0x0c);
    #[doc = "VDD * 11/16 selected as reference"]
    pub const REF11_16VDD: Self = Self(0x0d);
    #[doc = "VDD * 13/16 selected as reference"]
    pub const REF13_16VDD: Self = Self(0x0e);
    #[doc = "VDD * 15/16 selected as reference"]
    pub const REF15_16VDD: Self = Self(0x0f);
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
