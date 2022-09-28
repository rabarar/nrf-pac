#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Detectmode(pub u8);
impl Detectmode {
    #[doc = "DETECT directly connected to PIN DETECT signals"]
    pub const DEFAULT: Self = Self(0);
    #[doc = "Use the latched LDETECT behaviour"]
    pub const LDETECT: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sense(pub u8);
impl Sense {
    #[doc = "Disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Sense for high level"]
    pub const HIGH: Self = Self(0x02);
    #[doc = "Sense for low level"]
    pub const LOW: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pull(pub u8);
impl Pull {
    #[doc = "No pull"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Pull down on pin"]
    pub const PULLDOWN: Self = Self(0x01);
    #[doc = "Pull up on pin"]
    pub const PULLUP: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Drive(pub u8);
impl Drive {
    #[doc = "Standard '0', standard '1'"]
    pub const S0S1: Self = Self(0);
    #[doc = "High drive '0', standard '1'"]
    pub const H0S1: Self = Self(0x01);
    #[doc = "Standard '0', high drive '1'"]
    pub const S0H1: Self = Self(0x02);
    #[doc = "High drive '0', high 'drive '1''"]
    pub const H0H1: Self = Self(0x03);
    #[doc = "Disconnect '0' standard '1' (normally used for wired-or connections)"]
    pub const D0S1: Self = Self(0x04);
    #[doc = "Disconnect '0', high drive '1' (normally used for wired-or connections)"]
    pub const D0H1: Self = Self(0x05);
    #[doc = "Standard '0'. disconnect '1' (normally used for wired-and connections)"]
    pub const S0D1: Self = Self(0x06);
    #[doc = "High drive '0', disconnect '1' (normally used for wired-and connections)"]
    pub const H0D1: Self = Self(0x07);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dir(pub u8);
impl Dir {
    #[doc = "Pin set as input"]
    pub const INPUT: Self = Self(0);
    #[doc = "Pin set as output"]
    pub const OUTPUT: Self = Self(0x01);
}
