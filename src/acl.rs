#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AclAcl(pub *mut u8);
unsafe impl Send for AclAcl {}
unsafe impl Sync for AclAcl {}
impl AclAcl {
    #[doc = "Description cluster\\[n\\]: Configure the word-aligned start address of region n to protect"]
    #[inline(always)]
    pub fn addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Size of region to protect counting from address ACL\\[n\\].ADDR. Write '0' as no effect."]
    #[inline(always)]
    pub fn size(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Description cluster\\[n\\]: Access permissions for region n as defined by start address ACL\\[n\\].ADDR and size ACL\\[n\\].SIZE"]
    #[inline(always)]
    pub fn perm(self) -> crate::common::Reg<regs::Perm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn unused0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
#[doc = "Access control lists"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acl(pub *mut u8);
unsafe impl Send for Acl {}
unsafe impl Sync for Acl {}
impl Acl {
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn acl(self, n: usize) -> AclAcl {
        assert!(n < 8usize);
        unsafe { AclAcl(self.0.add(2048usize + n * 16usize)) }
    }
}
pub mod regs;
pub mod vals;
