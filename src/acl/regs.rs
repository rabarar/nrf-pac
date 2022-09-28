#[doc = "Description cluster\\[n\\]: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perm(pub u32);
impl Perm {
    #[doc = "Configure write and erase permissions for region n. Write '0' has no effect."]
    #[inline(always)]
    pub const fn write(&self) -> super::vals::Write {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Write(val as u8)
    }
    #[doc = "Configure write and erase permissions for region n. Write '0' has no effect."]
    #[inline(always)]
    pub fn set_write(&mut self, val: super::vals::Write) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.0 as u32) & 0x01) << 1usize);
    }
    #[doc = "Configure read permissions for region n. Write '0' has no effect."]
    #[inline(always)]
    pub const fn read(&self) -> super::vals::Read {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Read(val as u8)
    }
    #[doc = "Configure read permissions for region n. Write '0' has no effect."]
    #[inline(always)]
    pub fn set_read(&mut self, val: super::vals::Read) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.0 as u32) & 0x01) << 2usize);
    }
}
impl Default for Perm {
    #[inline(always)]
    fn default() -> Perm {
        Perm(0)
    }
}
