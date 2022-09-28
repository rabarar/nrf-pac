#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (5c3daee 2022-09-28))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - POWER_CLOCK"]
    POWER_CLOCK = 0,
    #[doc = "1 - RADIO"]
    RADIO = 1,
    #[doc = "2 - UARTE0_UART0"]
    UARTE0_UART0 = 2,
    #[doc = "3 - SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0"]
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 = 3,
    #[doc = "4 - SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1"]
    SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1 = 4,
    #[doc = "5 - NFCT"]
    NFCT = 5,
    #[doc = "6 - GPIOTE"]
    GPIOTE = 6,
    #[doc = "7 - SAADC"]
    SAADC = 7,
    #[doc = "8 - TIMER0"]
    TIMER0 = 8,
    #[doc = "9 - TIMER1"]
    TIMER1 = 9,
    #[doc = "10 - TIMER2"]
    TIMER2 = 10,
    #[doc = "11 - RTC0"]
    RTC0 = 11,
    #[doc = "12 - TEMP"]
    TEMP = 12,
    #[doc = "13 - RNG"]
    RNG = 13,
    #[doc = "14 - ECB"]
    ECB = 14,
    #[doc = "15 - CCM_AAR"]
    CCM_AAR = 15,
    #[doc = "16 - WDT"]
    WDT = 16,
    #[doc = "17 - RTC1"]
    RTC1 = 17,
    #[doc = "18 - QDEC"]
    QDEC = 18,
    #[doc = "19 - COMP_LPCOMP"]
    COMP_LPCOMP = 19,
    #[doc = "20 - SWI0_EGU0"]
    SWI0_EGU0 = 20,
    #[doc = "21 - SWI1_EGU1"]
    SWI1_EGU1 = 21,
    #[doc = "22 - SWI2_EGU2"]
    SWI2_EGU2 = 22,
    #[doc = "23 - SWI3_EGU3"]
    SWI3_EGU3 = 23,
    #[doc = "24 - SWI4_EGU4"]
    SWI4_EGU4 = 24,
    #[doc = "25 - SWI5_EGU5"]
    SWI5_EGU5 = 25,
    #[doc = "26 - TIMER3"]
    TIMER3 = 26,
    #[doc = "27 - TIMER4"]
    TIMER4 = 27,
    #[doc = "28 - PWM0"]
    PWM0 = 28,
    #[doc = "29 - PDM"]
    PDM = 29,
    #[doc = "32 - MWU"]
    MWU = 32,
    #[doc = "33 - PWM1"]
    PWM1 = 33,
    #[doc = "34 - PWM2"]
    PWM2 = 34,
    #[doc = "35 - SPIM2_SPIS2_SPI2"]
    SPIM2_SPIS2_SPI2 = 35,
    #[doc = "36 - RTC2"]
    RTC2 = 36,
    #[doc = "37 - I2S"]
    I2S = 37,
    #[doc = "38 - FPU"]
    FPU = 38,
    #[doc = "39 - USBD"]
    USBD = 39,
    #[doc = "40 - UARTE1"]
    UARTE1 = 40,
    #[doc = "41 - QSPI"]
    QSPI = 41,
    #[doc = "42 - CRYPTOCELL"]
    CRYPTOCELL = 42,
    #[doc = "45 - PWM3"]
    PWM3 = 45,
    #[doc = "47 - SPIM3"]
    SPIM3 = 47,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "Factory information configuration registers"]
pub const FICR: ficr::Ficr = ficr::Ficr(0x1000_0000 as u32 as _);
#[doc = "User information configuration registers"]
pub const UICR: uicr::Uicr = uicr::Uicr(0x1000_1000 as u32 as _);
#[doc = "Clock control"]
pub const CLOCK: clock::Clock = clock::Clock(0x4000_0000 as u32 as _);
#[doc = "Power control"]
pub const POWER: power::Power = power::Power(0x4000_0000 as u32 as _);
#[doc = "2.4 GHz radio"]
pub const RADIO: radio::Radio = radio::Radio(0x4000_1000 as u32 as _);
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub const UART0: uart0::Uart0 = uart0::Uart0(0x4000_2000 as u32 as _);
#[doc = "UART with EasyDMA 0"]
pub const UARTE0: uarte0::Uarte0 = uarte0::Uarte0(0x4000_2000 as u32 as _);
#[doc = "Serial Peripheral Interface 0"]
pub const SPI0: spi0::Spi0 = spi0::Spi0(0x4000_3000 as u32 as _);
#[doc = "Serial Peripheral Interface Master with EasyDMA 0"]
pub const SPIM0: spim0::Spim0 = spim0::Spim0(0x4000_3000 as u32 as _);
#[doc = "SPI Slave 0"]
pub const SPIS0: spis0::Spis0 = spis0::Spis0(0x4000_3000 as u32 as _);
#[doc = "I2C compatible Two-Wire Interface 0"]
pub const TWI0: twi0::Twi0 = twi0::Twi0(0x4000_3000 as u32 as _);
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
pub const TWIM0: twim0::Twim0 = twim0::Twim0(0x4000_3000 as u32 as _);
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 0"]
pub const TWIS0: twis0::Twis0 = twis0::Twis0(0x4000_3000 as u32 as _);
#[doc = "Serial Peripheral Interface 1"]
pub const SPI1: spi0::Spi0 = spi0::Spi0(0x4000_4000 as u32 as _);
#[doc = "Serial Peripheral Interface Master with EasyDMA 1"]
pub const SPIM1: spim0::Spim0 = spim0::Spim0(0x4000_4000 as u32 as _);
#[doc = "SPI Slave 1"]
pub const SPIS1: spis0::Spis0 = spis0::Spis0(0x4000_4000 as u32 as _);
#[doc = "I2C compatible Two-Wire Interface 1"]
pub const TWI1: twi0::Twi0 = twi0::Twi0(0x4000_4000 as u32 as _);
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 1"]
pub const TWIM1: twim0::Twim0 = twim0::Twim0(0x4000_4000 as u32 as _);
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 1"]
pub const TWIS1: twis0::Twis0 = twis0::Twis0(0x4000_4000 as u32 as _);
#[doc = "NFC-A compatible radio"]
pub const NFCT: nfct::Nfct = nfct::Nfct(0x4000_5000 as u32 as _);
#[doc = "GPIO Tasks and Events"]
pub const GPIOTE: gpiote::Gpiote = gpiote::Gpiote(0x4000_6000 as u32 as _);
#[doc = "Successive approximation register (SAR) analog-to-digital converter"]
pub const SAADC: saadc::Saadc = saadc::Saadc(0x4000_7000 as u32 as _);
#[doc = "Timer/Counter 0"]
pub const TIMER0: timer0::Timer0 = timer0::Timer0(0x4000_8000 as u32 as _);
#[doc = "Timer/Counter 1"]
pub const TIMER1: timer0::Timer0 = timer0::Timer0(0x4000_9000 as u32 as _);
#[doc = "Timer/Counter 2"]
pub const TIMER2: timer0::Timer0 = timer0::Timer0(0x4000_a000 as u32 as _);
#[doc = "Real time counter 0"]
pub const RTC0: rtc0::Rtc0 = rtc0::Rtc0(0x4000_b000 as u32 as _);
#[doc = "Temperature Sensor"]
pub const TEMP: temp::Temp = temp::Temp(0x4000_c000 as u32 as _);
#[doc = "Random Number Generator"]
pub const RNG: rng::Rng = rng::Rng(0x4000_d000 as u32 as _);
#[doc = "AES ECB Mode Encryption"]
pub const ECB: ecb::Ecb = ecb::Ecb(0x4000_e000 as u32 as _);
#[doc = "Accelerated Address Resolver"]
pub const AAR: aar::Aar = aar::Aar(0x4000_f000 as u32 as _);
#[doc = "AES CCM Mode Encryption"]
pub const CCM: ccm::Ccm = ccm::Ccm(0x4000_f000 as u32 as _);
#[doc = "Watchdog Timer"]
pub const WDT: wdt::Wdt = wdt::Wdt(0x4001_0000 as u32 as _);
#[doc = "Real time counter 1"]
pub const RTC1: rtc0::Rtc0 = rtc0::Rtc0(0x4001_1000 as u32 as _);
#[doc = "Quadrature Decoder"]
pub const QDEC: qdec::Qdec = qdec::Qdec(0x4001_2000 as u32 as _);
#[doc = "Comparator"]
pub const COMP: comp::Comp = comp::Comp(0x4001_3000 as u32 as _);
#[doc = "Low Power Comparator"]
pub const LPCOMP: lpcomp::Lpcomp = lpcomp::Lpcomp(0x4001_3000 as u32 as _);
#[doc = "Event Generator Unit 0"]
pub const EGU0: egu0::Egu0 = egu0::Egu0(0x4001_4000 as u32 as _);
#[doc = "Software interrupt 0"]
pub const SWI0: swi0::Swi0 = swi0::Swi0(0x4001_4000 as u32 as _);
#[doc = "Event Generator Unit 1"]
pub const EGU1: egu0::Egu0 = egu0::Egu0(0x4001_5000 as u32 as _);
#[doc = "Software interrupt 1"]
pub const SWI1: swi0::Swi0 = swi0::Swi0(0x4001_5000 as u32 as _);
#[doc = "Event Generator Unit 2"]
pub const EGU2: egu0::Egu0 = egu0::Egu0(0x4001_6000 as u32 as _);
#[doc = "Software interrupt 2"]
pub const SWI2: swi0::Swi0 = swi0::Swi0(0x4001_6000 as u32 as _);
#[doc = "Event Generator Unit 3"]
pub const EGU3: egu0::Egu0 = egu0::Egu0(0x4001_7000 as u32 as _);
#[doc = "Software interrupt 3"]
pub const SWI3: swi0::Swi0 = swi0::Swi0(0x4001_7000 as u32 as _);
#[doc = "Event Generator Unit 4"]
pub const EGU4: egu0::Egu0 = egu0::Egu0(0x4001_8000 as u32 as _);
#[doc = "Software interrupt 4"]
pub const SWI4: swi0::Swi0 = swi0::Swi0(0x4001_8000 as u32 as _);
#[doc = "Event Generator Unit 5"]
pub const EGU5: egu0::Egu0 = egu0::Egu0(0x4001_9000 as u32 as _);
#[doc = "Software interrupt 5"]
pub const SWI5: swi0::Swi0 = swi0::Swi0(0x4001_9000 as u32 as _);
#[doc = "Timer/Counter 3"]
pub const TIMER3: timer3::Timer3 = timer3::Timer3(0x4001_a000 as u32 as _);
#[doc = "Timer/Counter 4"]
pub const TIMER4: timer3::Timer3 = timer3::Timer3(0x4001_b000 as u32 as _);
#[doc = "Pulse width modulation unit 0"]
pub const PWM0: pwm0::Pwm0 = pwm0::Pwm0(0x4001_c000 as u32 as _);
#[doc = "Pulse Density Modulation (Digital Microphone) Interface"]
pub const PDM: pdm::Pdm = pdm::Pdm(0x4001_d000 as u32 as _);
#[doc = "Access control lists"]
pub const ACL: acl::Acl = acl::Acl(0x4001_e000 as u32 as _);
#[doc = "Non Volatile Memory Controller"]
pub const NVMC: nvmc::Nvmc = nvmc::Nvmc(0x4001_e000 as u32 as _);
#[doc = "Programmable Peripheral Interconnect"]
pub const PPI: ppi::Ppi = ppi::Ppi(0x4001_f000 as u32 as _);
#[doc = "Memory Watch Unit"]
pub const MWU: mwu::Mwu = mwu::Mwu(0x4002_0000 as u32 as _);
#[doc = "Pulse width modulation unit 1"]
pub const PWM1: pwm0::Pwm0 = pwm0::Pwm0(0x4002_1000 as u32 as _);
#[doc = "Pulse width modulation unit 2"]
pub const PWM2: pwm0::Pwm0 = pwm0::Pwm0(0x4002_2000 as u32 as _);
#[doc = "Serial Peripheral Interface 2"]
pub const SPI2: spi0::Spi0 = spi0::Spi0(0x4002_3000 as u32 as _);
#[doc = "Serial Peripheral Interface Master with EasyDMA 2"]
pub const SPIM2: spim0::Spim0 = spim0::Spim0(0x4002_3000 as u32 as _);
#[doc = "SPI Slave 2"]
pub const SPIS2: spis0::Spis0 = spis0::Spis0(0x4002_3000 as u32 as _);
#[doc = "Real time counter 2"]
pub const RTC2: rtc0::Rtc0 = rtc0::Rtc0(0x4002_4000 as u32 as _);
#[doc = "Inter-IC Sound"]
pub const I2S: i2s::I2s = i2s::I2s(0x4002_5000 as u32 as _);
#[doc = "FPU"]
pub const FPU: fpu::Fpu = fpu::Fpu(0x4002_6000 as u32 as _);
#[doc = "Universal serial bus device"]
pub const USBD: usbd::Usbd = usbd::Usbd(0x4002_7000 as u32 as _);
#[doc = "UART with EasyDMA 1"]
pub const UARTE1: uarte0::Uarte0 = uarte0::Uarte0(0x4002_8000 as u32 as _);
#[doc = "External flash interface"]
pub const QSPI: qspi::Qspi = qspi::Qspi(0x4002_9000 as u32 as _);
#[doc = "Pulse width modulation unit 3"]
pub const PWM3: pwm0::Pwm0 = pwm0::Pwm0(0x4002_d000 as u32 as _);
#[doc = "Serial Peripheral Interface Master with EasyDMA 3"]
pub const SPIM3: spim0::Spim0 = spim0::Spim0(0x4002_f000 as u32 as _);
#[doc = "GPIO Port 1"]
pub const P0: p0::P0 = p0::P0(0x5000_0000 as u32 as _);
#[doc = "GPIO Port 2"]
pub const P1: p0::P0 = p0::P0(0x5000_0300 as u32 as _);
#[doc = "CRYPTOCELL HOST_RGF interface"]
pub const CC_HOST_RGF: cc_host_rgf::CcHostRgf = cc_host_rgf::CcHostRgf(0x5002_a000 as u32 as _);
#[doc = "ARM TrustZone CryptoCell register interface"]
pub const CRYPTOCELL: cryptocell::Cryptocell = cryptocell::Cryptocell(0x5002_a000 as u32 as _);
pub mod aar;
pub mod acl;
pub mod cc_host_rgf;
pub mod ccm;
pub mod clock;
pub mod common;
pub mod comp;
pub mod cryptocell;
pub mod ecb;
pub mod egu0;
pub mod ficr;
pub mod fpu;
pub mod gpiote;
pub mod i2s;
pub mod lpcomp;
pub mod mwu;
pub mod nfct;
pub mod nvmc;
pub mod p0;
pub mod pdm;
pub mod power;
pub mod ppi;
pub mod pwm0;
pub mod qdec;
pub mod qspi;
pub mod radio;
pub mod rng;
pub mod rtc0;
pub mod saadc;
pub mod spi0;
pub mod spim0;
pub mod spis0;
pub mod swi0;
pub mod temp;
pub mod timer0;
pub mod timer3;
pub mod twi0;
pub mod twim0;
pub mod twis0;
pub mod uart0;
pub mod uarte0;
pub mod uicr;
pub mod usbd;
pub mod wdt;
