#[doc = "Quadrature Decoder"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdec(pub *mut u8);
unsafe impl Send for Qdec {}
unsafe impl Sync for Qdec {}
impl Qdec {
    #[doc = "Task starting the quadrature decoder"]
    #[inline(always)]
    pub fn tasks_start(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Task stopping the quadrature decoder"]
    #[inline(always)]
    pub fn tasks_stop(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Read and clear ACC and ACCDBL"]
    #[inline(always)]
    pub fn tasks_readclracc(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Read and clear ACC"]
    #[inline(always)]
    pub fn tasks_rdclracc(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Read and clear ACCDBL"]
    #[inline(always)]
    pub fn tasks_rdclrdbl(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Event being generated for every new sample value written to the SAMPLE register"]
    #[inline(always)]
    pub fn events_samplerdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "Non-null report ready"]
    #[inline(always)]
    pub fn events_reportrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "ACC or ACCDBL register overflow"]
    #[inline(always)]
    pub fn events_accof(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }
    #[doc = "Double displacement(s) detected"]
    #[inline(always)]
    pub fn events_dblrdy(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(268usize)) }
    }
    #[doc = "QDEC has been stopped"]
    #[inline(always)]
    pub fn events_stopped(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
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
    #[doc = "Enable the quadrature decoder"]
    #[inline(always)]
    pub fn enable(self) -> crate::common::Reg<regs::Enable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }
    #[doc = "LED output pin polarity"]
    #[inline(always)]
    pub fn ledpol(self) -> crate::common::Reg<regs::Ledpol, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }
    #[doc = "Sample period"]
    #[inline(always)]
    pub fn sampleper(self) -> crate::common::Reg<regs::Sampleper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }
    #[doc = "Motion sample value"]
    #[inline(always)]
    pub fn sample(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1292usize)) }
    }
    #[doc = "Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
    #[inline(always)]
    pub fn reportper(self) -> crate::common::Reg<regs::Reportper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize)) }
    }
    #[doc = "Register accumulating the valid transitions"]
    #[inline(always)]
    pub fn acc(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1300usize)) }
    }
    #[doc = "Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task"]
    #[inline(always)]
    pub fn accread(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1304usize)) }
    }
    #[doc = "Unspecified"]
    #[inline(always)]
    pub fn psel(self) -> Psel {
        unsafe { Psel(self.0.add(1308usize)) }
    }
    #[doc = "Enable input debounce filters"]
    #[inline(always)]
    pub fn dbfen(self) -> crate::common::Reg<regs::Dbfen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1320usize)) }
    }
    #[doc = "Time period the LED is switched ON prior to sampling"]
    #[inline(always)]
    pub fn ledpre(self) -> crate::common::Reg<regs::Ledpre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1344usize)) }
    }
    #[doc = "Register accumulating the number of detected double transitions"]
    #[inline(always)]
    pub fn accdbl(self) -> crate::common::Reg<regs::Accdbl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1348usize)) }
    }
    #[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
    #[inline(always)]
    pub fn accdblread(self) -> crate::common::Reg<regs::Accdblread, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1352usize)) }
    }
}
#[doc = "Unspecified"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psel(pub *mut u8);
unsafe impl Send for Psel {}
unsafe impl Sync for Psel {}
impl Psel {
    #[doc = "Pin select for LED signal"]
    #[inline(always)]
    pub fn led(self) -> crate::common::Reg<regs::Led, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Pin select for A signal"]
    #[inline(always)]
    pub fn a(self) -> crate::common::Reg<regs::A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Pin select for B signal"]
    #[inline(always)]
    pub fn b(self) -> crate::common::Reg<regs::B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
pub mod regs;
pub mod vals;
