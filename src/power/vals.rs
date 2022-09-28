#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Outputrdy(pub u8);
impl Outputrdy {
    #[doc = "USBREG output settling time not elapsed"]
    pub const NOTREADY: Self = Self(0);
    #[doc = "USBREG output settling time elapsed (same information as USBPWRRDY event)"]
    pub const READY: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Systemoff(pub u8);
impl Systemoff {
    #[doc = "Enable System OFF mode"]
    pub const ENTER: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Vbusdetect(pub u8);
impl Vbusdetect {
    #[doc = "VBUS voltage below valid threshold"]
    pub const NOVBUS: Self = Self(0);
    #[doc = "VBUS voltage above valid threshold"]
    pub const VBUSPRESENT: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Nfc(pub u8);
impl Nfc {
    #[doc = "Not detected"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "Detected"]
    pub const DETECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dog(pub u8);
impl Dog {
    #[doc = "Not detected"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "Detected"]
    pub const DETECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lpcomp(pub u8);
impl Lpcomp {
    #[doc = "Not detected"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "Detected"]
    pub const DETECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Resetpin(pub u8);
impl Resetpin {
    #[doc = "Not detected"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "Detected"]
    pub const DETECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pof(pub u8);
impl Pof {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Lockup(pub u8);
impl Lockup {
    #[doc = "Not detected"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "Detected"]
    pub const DETECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sreq(pub u8);
impl Sreq {
    #[doc = "Not detected"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "Detected"]
    pub const DETECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Threshold(pub u8);
impl Threshold {
    #[doc = "Set threshold to 1.7 V"]
    pub const V17: Self = Self(0x04);
    #[doc = "Set threshold to 1.8 V"]
    pub const V18: Self = Self(0x05);
    #[doc = "Set threshold to 1.9 V"]
    pub const V19: Self = Self(0x06);
    #[doc = "Set threshold to 2.0 V"]
    pub const V20: Self = Self(0x07);
    #[doc = "Set threshold to 2.1 V"]
    pub const V21: Self = Self(0x08);
    #[doc = "Set threshold to 2.2 V"]
    pub const V22: Self = Self(0x09);
    #[doc = "Set threshold to 2.3 V"]
    pub const V23: Self = Self(0x0a);
    #[doc = "Set threshold to 2.4 V"]
    pub const V24: Self = Self(0x0b);
    #[doc = "Set threshold to 2.5 V"]
    pub const V25: Self = Self(0x0c);
    #[doc = "Set threshold to 2.6 V"]
    pub const V26: Self = Self(0x0d);
    #[doc = "Set threshold to 2.7 V"]
    pub const V27: Self = Self(0x0e);
    #[doc = "Set threshold to 2.8 V"]
    pub const V28: Self = Self(0x0f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Thresholdvddh(pub u8);
impl Thresholdvddh {
    #[doc = "Set threshold to 2.7 V"]
    pub const V27: Self = Self(0);
    #[doc = "Set threshold to 2.8 V"]
    pub const V28: Self = Self(0x01);
    #[doc = "Set threshold to 2.9 V"]
    pub const V29: Self = Self(0x02);
    #[doc = "Set threshold to 3.0 V"]
    pub const V30: Self = Self(0x03);
    #[doc = "Set threshold to 3.1 V"]
    pub const V31: Self = Self(0x04);
    #[doc = "Set threshold to 3.2 V"]
    pub const V32: Self = Self(0x05);
    #[doc = "Set threshold to 3.3 V"]
    pub const V33: Self = Self(0x06);
    #[doc = "Set threshold to 3.4 V"]
    pub const V34: Self = Self(0x07);
    #[doc = "Set threshold to 3.5 V"]
    pub const V35: Self = Self(0x08);
    #[doc = "Set threshold to 3.6 V"]
    pub const V36: Self = Self(0x09);
    #[doc = "Set threshold to 3.7 V"]
    pub const V37: Self = Self(0x0a);
    #[doc = "Set threshold to 3.8 V"]
    pub const V38: Self = Self(0x0b);
    #[doc = "Set threshold to 3.9 V"]
    pub const V39: Self = Self(0x0c);
    #[doc = "Set threshold to 4.0 V"]
    pub const V40: Self = Self(0x0d);
    #[doc = "Set threshold to 4.1 V"]
    pub const V41: Self = Self(0x0e);
    #[doc = "Set threshold to 4.2 V"]
    pub const V42: Self = Self(0x0f);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mainregstatus(pub u8);
impl Mainregstatus {
    #[doc = "Normal voltage mode. Voltage supplied on VDD."]
    pub const NORMAL: Self = Self(0);
    #[doc = "High voltage mode. Voltage supplied on VDDH."]
    pub const HIGH: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dcdcen0dcdcen(pub u8);
impl Dcdcen0dcdcen {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Off(pub u8);
impl Off {
    #[doc = "Not detected"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "Detected"]
    pub const DETECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DcdcenDcdcen(pub u8);
impl DcdcenDcdcen {
    #[doc = "Disable"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Vbus(pub u8);
impl Vbus {
    #[doc = "Not detected"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "Detected"]
    pub const DETECTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dif(pub u8);
impl Dif {
    #[doc = "Not detected"]
    pub const NOTDETECTED: Self = Self(0);
    #[doc = "Detected"]
    pub const DETECTED: Self = Self(0x01);
}
