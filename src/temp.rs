#[doc = "Temperature Sensor"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Temp(pub *mut u8);
unsafe impl Send for Temp {}
unsafe impl Sync for Temp {}
impl Temp {
    #[doc = "Start temperature measurement"]
    #[inline(always)]
    pub fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Stop temperature measurement"]
    #[inline(always)]
    pub fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Temperature measurement complete, data ready"]
    #[inline(always)]
    pub fn events_datardy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn intenset(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(772usize)) }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn intenclr(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(776usize)) }
    }
    #[doc = "Temperature in degC (0.25deg steps)"]
    #[inline(always)]
    pub fn temp(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "Slope of 1st piece wise linear function"]
    #[inline(always)]
    pub fn a0(self) -> crate::common::Reg<regs::A0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1312usize)) }
    }
    #[doc = "Slope of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn a1(self) -> crate::common::Reg<regs::A1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1316usize)) }
    }
    #[doc = "Slope of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn a2(self) -> crate::common::Reg<regs::A2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1320usize)) }
    }
    #[doc = "Slope of 4th piece wise linear function"]
    #[inline(always)]
    pub fn a3(self) -> crate::common::Reg<regs::A3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1324usize)) }
    }
    #[doc = "Slope of 5th piece wise linear function"]
    #[inline(always)]
    pub fn a4(self) -> crate::common::Reg<regs::A4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1328usize)) }
    }
    #[doc = "Slope of 6th piece wise linear function"]
    #[inline(always)]
    pub fn a5(self) -> crate::common::Reg<regs::A5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1332usize)) }
    }
    #[doc = "y-intercept of 1st piece wise linear function"]
    #[inline(always)]
    pub fn b0(self) -> crate::common::Reg<regs::B0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1344usize)) }
    }
    #[doc = "y-intercept of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn b1(self) -> crate::common::Reg<regs::B1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1348usize)) }
    }
    #[doc = "y-intercept of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn b2(self) -> crate::common::Reg<regs::B2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1352usize)) }
    }
    #[doc = "y-intercept of 4th piece wise linear function"]
    #[inline(always)]
    pub fn b3(self) -> crate::common::Reg<regs::B3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1356usize)) }
    }
    #[doc = "y-intercept of 5th piece wise linear function"]
    #[inline(always)]
    pub fn b4(self) -> crate::common::Reg<regs::B4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1360usize)) }
    }
    #[doc = "y-intercept of 6th piece wise linear function"]
    #[inline(always)]
    pub fn b5(self) -> crate::common::Reg<regs::B5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1364usize)) }
    }
    #[doc = "End point of 1st piece wise linear function"]
    #[inline(always)]
    pub fn t0(self) -> crate::common::Reg<regs::T0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1376usize)) }
    }
    #[doc = "End point of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn t1(self) -> crate::common::Reg<regs::T1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1380usize)) }
    }
    #[doc = "End point of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn t2(self) -> crate::common::Reg<regs::T2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1384usize)) }
    }
    #[doc = "End point of 4th piece wise linear function"]
    #[inline(always)]
    pub fn t3(self) -> crate::common::Reg<regs::T3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1388usize)) }
    }
    #[doc = "End point of 5th piece wise linear function"]
    #[inline(always)]
    pub fn t4(self) -> crate::common::Reg<regs::T4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1392usize)) }
    }
}
pub mod regs;
