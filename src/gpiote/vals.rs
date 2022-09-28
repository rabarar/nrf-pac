#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mode(pub u8);
impl Mode {
    #[doc = "Disabled. Pin specified by PSEL will not be acquired by the GPIOTE module."]
    pub const DISABLED: Self = Self(0);
    #[doc = "Event mode"]
    pub const EVENT: Self = Self(0x01);
    #[doc = "Task mode"]
    pub const TASK: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Polarity(pub u8);
impl Polarity {
    #[doc = "Task mode: No effect on pin from OUT\\[n\\] task. Event mode: no IN\\[n\\] event generated on pin activity."]
    pub const NONE: Self = Self(0);
    #[doc = "Task mode: Set pin from OUT\\[n\\] task. Event mode: Generate IN\\[n\\] event when rising edge on pin."]
    pub const LOTOHI: Self = Self(0x01);
    #[doc = "Task mode: Clear pin from OUT\\[n\\] task. Event mode: Generate IN\\[n\\] event when falling edge on pin."]
    pub const HITOLO: Self = Self(0x02);
    #[doc = "Task mode: Toggle pin from OUT\\[n\\]. Event mode: Generate IN\\[n\\] when any change on pin."]
    pub const TOGGLE: Self = Self(0x03);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Outinit(pub u8);
impl Outinit {
    #[doc = "Task mode: Initial value of pin before task triggering is low"]
    pub const LOW: Self = Self(0);
    #[doc = "Task mode: Initial value of pin before task triggering is high"]
    pub const HIGH: Self = Self(0x01);
}
