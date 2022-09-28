#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pregion(pub *mut u8);
unsafe impl Send for Pregion {}
unsafe impl Sync for Pregion {}
impl Pregion {
    #[doc = "Description cluster\\[n\\]: Reserved for future use"]
    #[inline(always)]
    pub fn start(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Reserved for future use"]
    #[inline(always)]
    pub fn end(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Subregions of region n"]
    #[inline(always)]
    pub fn subs(self) -> crate::common::Reg<regs::Subs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EventsRegion(pub *mut u8);
unsafe impl Send for EventsRegion {}
unsafe impl Sync for EventsRegion {}
impl EventsRegion {
    #[doc = "Description cluster\\[n\\]: Write access to region n detected"]
    #[inline(always)]
    pub fn wa(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Read access to region n detected"]
    #[inline(always)]
    pub fn ra(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
#[doc = "Memory Watch Unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwu(pub *mut u8);
unsafe impl Send for Mwu {}
unsafe impl Sync for Mwu {}
impl Mwu {
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn events_region(self, n: usize) -> EventsRegion {
        assert!(n < 4usize);
        unsafe { EventsRegion(self.0.add(256usize + n * 8usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn events_pregion(self, n: usize) -> EventsPregion {
        assert!(n < 2usize);
        unsafe { EventsPregion(self.0.add(352usize + n * 8usize)) }
    }
    #[doc = "Enable or disable interrupt"]
    #[inline(always)]
    pub fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(768usize)) }
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
    #[doc = "Enable or disable non-maskable interrupt"]
    #[inline(always)]
    pub fn nmien(self) -> crate::common::Reg<regs::Nmien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(800usize)) }
    }
    #[doc = "Enable non-maskable interrupt"]
    #[inline(always)]
    pub fn nmienset(self) -> crate::common::Reg<regs::Nmien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(804usize)) }
    }
    #[doc = "Disable non-maskable interrupt"]
    #[inline(always)]
    pub fn nmienclr(self) -> crate::common::Reg<regs::Nmien, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(808usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn perregion(self, n: usize) -> Perregion {
        assert!(n < 2usize);
        unsafe { Perregion(self.0.add(1024usize + n * 8usize)) }
    }
    #[doc = "Enable/disable regions watch"]
    #[inline(always)]
    pub fn regionen(self) -> crate::common::Reg<regs::Regionen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize)) }
    }
    #[doc = "Enable regions watch"]
    #[inline(always)]
    pub fn regionenset(self) -> crate::common::Reg<regs::Regionen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1300usize)) }
    }
    #[doc = "Disable regions watch"]
    #[inline(always)]
    pub fn regionenclr(self) -> crate::common::Reg<regs::Regionen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1304usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn region(self, n: usize) -> Region {
        assert!(n < 4usize);
        unsafe { Region(self.0.add(1536usize + n * 16usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn pregion(self, n: usize) -> Pregion {
        assert!(n < 2usize);
        unsafe { Pregion(self.0.add(1728usize + n * 16usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EventsPregion(pub *mut u8);
unsafe impl Send for EventsPregion {}
unsafe impl Sync for EventsPregion {}
impl EventsPregion {
    #[doc = "Description cluster\\[n\\]: Write access to peripheral region n detected"]
    #[inline(always)]
    pub fn wa(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Read access to peripheral region n detected"]
    #[inline(always)]
    pub fn ra(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Region(pub *mut u8);
unsafe impl Send for Region {}
unsafe impl Sync for Region {}
impl Region {
    #[doc = "Description cluster\\[n\\]: Start address for region n"]
    #[inline(always)]
    pub fn start(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Description cluster\\[n\\]: End address of region n"]
    #[inline(always)]
    pub fn end(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perregion(pub *mut u8);
unsafe impl Send for Perregion {}
unsafe impl Sync for Perregion {}
impl Perregion {
    #[doc = "Description cluster\\[n\\]: Source of event/interrupt in region n, write access detected while corresponding subregion was enabled for watching"]
    #[inline(always)]
    pub fn substatwa(self) -> crate::common::Reg<regs::Substatwa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Source of event/interrupt in region n, read access detected while corresponding subregion was enabled for watching"]
    #[inline(always)]
    pub fn substatra(self) -> crate::common::Reg<regs::Substatra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
pub mod regs;
