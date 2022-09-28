#[doc = "Slope of 3rd piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A2(pub u32);
impl A2 {
    #[doc = "Slope of 3rd piece wise linear function"]
    #[inline(always)]
    pub const fn a2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Slope of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn set_a2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A2 {
    #[inline(always)]
    fn default() -> A2 {
        A2(0)
    }
}
#[doc = "Slope of 2nd piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A1(pub u32);
impl A1 {
    #[doc = "Slope of 2nd piece wise linear function"]
    #[inline(always)]
    pub const fn a1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Slope of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn set_a1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A1 {
    #[inline(always)]
    fn default() -> A1 {
        A1(0)
    }
}
#[doc = "y-intercept of 1st piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B0(pub u32);
impl B0 {
    #[doc = "y-intercept of 1st piece wise linear function"]
    #[inline(always)]
    pub const fn b0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "y-intercept of 1st piece wise linear function"]
    #[inline(always)]
    pub fn set_b0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for B0 {
    #[inline(always)]
    fn default() -> B0 {
        B0(0)
    }
}
#[doc = "End point of 3rd piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T2(pub u32);
impl T2 {
    #[doc = "End point of 3rd piece wise linear function"]
    #[inline(always)]
    pub const fn t2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "End point of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn set_t2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T2 {
    #[inline(always)]
    fn default() -> T2 {
        T2(0)
    }
}
#[doc = "y-intercept of 2nd piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B1(pub u32);
impl B1 {
    #[doc = "y-intercept of 2nd piece wise linear function"]
    #[inline(always)]
    pub const fn b1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "y-intercept of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn set_b1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for B1 {
    #[inline(always)]
    fn default() -> B1 {
        B1(0)
    }
}
#[doc = "Slope of 6th piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A5(pub u32);
impl A5 {
    #[doc = "Slope of 6th piece wise linear function"]
    #[inline(always)]
    pub const fn a5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Slope of 6th piece wise linear function"]
    #[inline(always)]
    pub fn set_a5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A5 {
    #[inline(always)]
    fn default() -> A5 {
        A5(0)
    }
}
#[doc = "y-intercept of 4th piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B3(pub u32);
impl B3 {
    #[doc = "y-intercept of 4th piece wise linear function"]
    #[inline(always)]
    pub const fn b3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "y-intercept of 4th piece wise linear function"]
    #[inline(always)]
    pub fn set_b3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for B3 {
    #[inline(always)]
    fn default() -> B3 {
        B3(0)
    }
}
#[doc = "End point of 2nd piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T1(pub u32);
impl T1 {
    #[doc = "End point of 2nd piece wise linear function"]
    #[inline(always)]
    pub const fn t1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "End point of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn set_t1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T1 {
    #[inline(always)]
    fn default() -> T1 {
        T1(0)
    }
}
#[doc = "End point of 5th piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T4(pub u32);
impl T4 {
    #[doc = "End point of 5th piece wise linear function"]
    #[inline(always)]
    pub const fn t4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "End point of 5th piece wise linear function"]
    #[inline(always)]
    pub fn set_t4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T4 {
    #[inline(always)]
    fn default() -> T4 {
        T4(0)
    }
}
#[doc = "y-intercept of 5th piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B4(pub u32);
impl B4 {
    #[doc = "y-intercept of 5th piece wise linear function"]
    #[inline(always)]
    pub const fn b4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "y-intercept of 5th piece wise linear function"]
    #[inline(always)]
    pub fn set_b4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for B4 {
    #[inline(always)]
    fn default() -> B4 {
        B4(0)
    }
}
#[doc = "y-intercept of 3rd piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B2(pub u32);
impl B2 {
    #[doc = "y-intercept of 3rd piece wise linear function"]
    #[inline(always)]
    pub const fn b2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "y-intercept of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn set_b2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for B2 {
    #[inline(always)]
    fn default() -> B2 {
        B2(0)
    }
}
#[doc = "Enable interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Write '1' to enable interrupt for DATARDY event"]
    #[inline(always)]
    pub const fn datardy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write '1' to enable interrupt for DATARDY event"]
    #[inline(always)]
    pub fn set_datardy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "End point of 1st piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T0(pub u32);
impl T0 {
    #[doc = "End point of 1st piece wise linear function"]
    #[inline(always)]
    pub const fn t0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "End point of 1st piece wise linear function"]
    #[inline(always)]
    pub fn set_t0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T0 {
    #[inline(always)]
    fn default() -> T0 {
        T0(0)
    }
}
#[doc = "Slope of 4th piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A3(pub u32);
impl A3 {
    #[doc = "Slope of 4th piece wise linear function"]
    #[inline(always)]
    pub const fn a3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Slope of 4th piece wise linear function"]
    #[inline(always)]
    pub fn set_a3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A3 {
    #[inline(always)]
    fn default() -> A3 {
        A3(0)
    }
}
#[doc = "End point of 4th piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T3(pub u32);
impl T3 {
    #[doc = "End point of 4th piece wise linear function"]
    #[inline(always)]
    pub const fn t3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "End point of 4th piece wise linear function"]
    #[inline(always)]
    pub fn set_t3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for T3 {
    #[inline(always)]
    fn default() -> T3 {
        T3(0)
    }
}
#[doc = "y-intercept of 6th piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B5(pub u32);
impl B5 {
    #[doc = "y-intercept of 6th piece wise linear function"]
    #[inline(always)]
    pub const fn b5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "y-intercept of 6th piece wise linear function"]
    #[inline(always)]
    pub fn set_b5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for B5 {
    #[inline(always)]
    fn default() -> B5 {
        B5(0)
    }
}
#[doc = "Slope of 1st piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A0(pub u32);
impl A0 {
    #[doc = "Slope of 1st piece wise linear function"]
    #[inline(always)]
    pub const fn a0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Slope of 1st piece wise linear function"]
    #[inline(always)]
    pub fn set_a0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A0 {
    #[inline(always)]
    fn default() -> A0 {
        A0(0)
    }
}
#[doc = "Slope of 5th piece wise linear function"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct A4(pub u32);
impl A4 {
    #[doc = "Slope of 5th piece wise linear function"]
    #[inline(always)]
    pub const fn a4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Slope of 5th piece wise linear function"]
    #[inline(always)]
    pub fn set_a4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for A4 {
    #[inline(always)]
    fn default() -> A4 {
        A4(0)
    }
}
