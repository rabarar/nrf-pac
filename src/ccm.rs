#[doc = "AES CCM Mode Encryption"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccm(pub *mut u8);
unsafe impl Send for Ccm {}
unsafe impl Sync for Ccm {}
impl Ccm {
    #[doc = "Start generation of key-stream. This operation will stop by itself when completed."]
    #[inline(always)]
    pub fn tasks_ksgen(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Start encryption/decryption. This operation will stop by itself when completed."]
    #[inline(always)]
    pub fn tasks_crypt(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Stop encryption/decryption"]
    #[inline(always)]
    pub fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
    #[inline(always)]
    pub fn tasks_rateoverride(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Key-stream generation complete"]
    #[inline(always)]
    pub fn events_endksgen(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "Encrypt/decrypt complete"]
    #[inline(always)]
    pub fn events_endcrypt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "Deprecated register - CCM error event"]
    #[inline(always)]
    pub fn events_error(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }
    #[doc = "Shortcut register"]
    #[inline(always)]
    pub fn shorts(self) -> crate::common::Reg<regs::Shorts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(512usize)) }
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
    #[doc = "MIC check result"]
    #[inline(always)]
    pub fn micstatus(self) -> crate::common::Reg<regs::Micstatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1024usize)) }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "Operation mode"]
    #[inline(always)]
    pub fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "Pointer to data structure holding AES key and NONCE vector"]
    #[inline(always)]
    pub fn cnfptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "Input pointer"]
    #[inline(always)]
    pub fn inptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1292usize)) }
    }
    #[doc = "Output pointer"]
    #[inline(always)]
    pub fn outptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize)) }
    }
    #[doc = "Pointer to data area used for temporary storage"]
    #[inline(always)]
    pub fn scratchptr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1300usize)) }
    }
    #[doc = "Length of key-stream generated when MODE.LENGTH = Extended."]
    #[inline(always)]
    pub fn maxpacketsize(self) -> crate::common::Reg<regs::Maxpacketsize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1304usize)) }
    }
    #[doc = "Data rate override setting."]
    #[inline(always)]
    pub fn rateoverride(self) -> crate::common::Reg<regs::Rateoverride, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1308usize)) }
    }
}
pub mod regs;
pub mod vals;
