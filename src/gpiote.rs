#[doc = "GPIO Tasks and Events"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpiote(pub *mut u8);
unsafe impl Send for Gpiote {}
unsafe impl Sync for Gpiote {}
impl Gpiote {
    #[doc = "Description collection\\[n\\]: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
    #[inline(always)]
    pub fn tasks_out(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize + n * 4usize)) }
    }
    #[doc = "Description collection\\[n\\]: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high."]
    #[inline(always)]
    pub fn tasks_set(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize + n * 4usize)) }
    }
    #[doc = "Description collection\\[n\\]: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low."]
    #[inline(always)]
    pub fn tasks_clr(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize + n * 4usize)) }
    }
    #[doc = "Description collection\\[n\\]: Event generated from pin specified in CONFIG\\[n\\].PSEL"]
    #[inline(always)]
    pub fn events_in(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize + n * 4usize)) }
    }
    #[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
    #[inline(always)]
    pub fn events_port(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(380usize)) }
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
    #[doc = "Description collection\\[n\\]: Configuration for OUT\\[n\\], SET\\[n\\] and CLR\\[n\\] tasks and IN\\[n\\] event"]
    #[inline(always)]
    pub fn config(self, n: usize) -> crate::common::Reg<regs::Config, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize + n * 4usize)) }
    }
}
pub mod regs;
pub mod vals;
