#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ReqstatusRr4(pub u8);
impl ReqstatusRr4 {
    #[doc = "RR\\[4\\] register is not enabled, or are already requesting reload"]
    pub const DISABLEDORREQUESTED: Self = Self(0);
    #[doc = "RR\\[4\\] register is enabled, and are not yet requesting reload"]
    pub const ENABLEDANDUNREQUESTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ReqstatusRr6(pub u8);
impl ReqstatusRr6 {
    #[doc = "RR\\[6\\] register is not enabled, or are already requesting reload"]
    pub const DISABLEDORREQUESTED: Self = Self(0);
    #[doc = "RR\\[6\\] register is enabled, and are not yet requesting reload"]
    pub const ENABLEDANDUNREQUESTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RrenRr0(pub u8);
impl RrenRr0 {
    #[doc = "Disable RR\\[0\\] register"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable RR\\[0\\] register"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RrenRr7(pub u8);
impl RrenRr7 {
    #[doc = "Disable RR\\[7\\] register"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable RR\\[7\\] register"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RrenRr6(pub u8);
impl RrenRr6 {
    #[doc = "Disable RR\\[6\\] register"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable RR\\[6\\] register"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Runstatus(pub u8);
impl Runstatus {
    #[doc = "Watchdog not running"]
    pub const NOTRUNNING: Self = Self(0);
    #[doc = "Watchdog is running"]
    pub const RUNNING: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RrenRr3(pub u8);
impl RrenRr3 {
    #[doc = "Disable RR\\[3\\] register"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable RR\\[3\\] register"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Halt(pub u8);
impl Halt {
    #[doc = "Pause watchdog while the CPU is halted by the debugger"]
    pub const PAUSE: Self = Self(0);
    #[doc = "Keep the watchdog running while the CPU is halted by the debugger"]
    pub const RUN: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RrenRr5(pub u8);
impl RrenRr5 {
    #[doc = "Disable RR\\[5\\] register"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable RR\\[5\\] register"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ReqstatusRr0(pub u8);
impl ReqstatusRr0 {
    #[doc = "RR\\[0\\] register is not enabled, or are already requesting reload"]
    pub const DISABLEDORREQUESTED: Self = Self(0);
    #[doc = "RR\\[0\\] register is enabled, and are not yet requesting reload"]
    pub const ENABLEDANDUNREQUESTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sleep(pub u8);
impl Sleep {
    #[doc = "Pause watchdog while the CPU is sleeping"]
    pub const PAUSE: Self = Self(0);
    #[doc = "Keep the watchdog running while the CPU is sleeping"]
    pub const RUN: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ReqstatusRr3(pub u8);
impl ReqstatusRr3 {
    #[doc = "RR\\[3\\] register is not enabled, or are already requesting reload"]
    pub const DISABLEDORREQUESTED: Self = Self(0);
    #[doc = "RR\\[3\\] register is enabled, and are not yet requesting reload"]
    pub const ENABLEDANDUNREQUESTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RrenRr2(pub u8);
impl RrenRr2 {
    #[doc = "Disable RR\\[2\\] register"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable RR\\[2\\] register"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RrenRr4(pub u8);
impl RrenRr4 {
    #[doc = "Disable RR\\[4\\] register"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable RR\\[4\\] register"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RrenRr1(pub u8);
impl RrenRr1 {
    #[doc = "Disable RR\\[1\\] register"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enable RR\\[1\\] register"]
    pub const ENABLED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ReqstatusRr7(pub u8);
impl ReqstatusRr7 {
    #[doc = "RR\\[7\\] register is not enabled, or are already requesting reload"]
    pub const DISABLEDORREQUESTED: Self = Self(0);
    #[doc = "RR\\[7\\] register is enabled, and are not yet requesting reload"]
    pub const ENABLEDANDUNREQUESTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ReqstatusRr2(pub u8);
impl ReqstatusRr2 {
    #[doc = "RR\\[2\\] register is not enabled, or are already requesting reload"]
    pub const DISABLEDORREQUESTED: Self = Self(0);
    #[doc = "RR\\[2\\] register is enabled, and are not yet requesting reload"]
    pub const ENABLEDANDUNREQUESTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ReqstatusRr1(pub u8);
impl ReqstatusRr1 {
    #[doc = "RR\\[1\\] register is not enabled, or are already requesting reload"]
    pub const DISABLEDORREQUESTED: Self = Self(0);
    #[doc = "RR\\[1\\] register is enabled, and are not yet requesting reload"]
    pub const ENABLEDANDUNREQUESTED: Self = Self(0x01);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rr(pub u32);
impl Rr {
    #[doc = "Value to request a reload of the watchdog timer"]
    pub const RELOAD: Self = Self(0x6e52_4635);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ReqstatusRr5(pub u8);
impl ReqstatusRr5 {
    #[doc = "RR\\[5\\] register is not enabled, or are already requesting reload"]
    pub const DISABLEDORREQUESTED: Self = Self(0);
    #[doc = "RR\\[5\\] register is enabled, and are not yet requesting reload"]
    pub const ENABLEDANDUNREQUESTED: Self = Self(0x01);
}
