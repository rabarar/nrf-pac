extern "C" {
    fn POWER_CLOCK();
    fn RADIO();
    fn UARTE0_UART0();
    fn SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0();
    fn SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1();
    fn NFCT();
    fn GPIOTE();
    fn SAADC();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn RTC0();
    fn TEMP();
    fn RNG();
    fn ECB();
    fn CCM_AAR();
    fn WDT();
    fn RTC1();
    fn QDEC();
    fn COMP_LPCOMP();
    fn SWI0_EGU0();
    fn SWI1_EGU1();
    fn SWI2_EGU2();
    fn SWI3_EGU3();
    fn SWI4_EGU4();
    fn SWI5_EGU5();
    fn TIMER3();
    fn TIMER4();
    fn PWM0();
    fn PDM();
    fn MWU();
    fn PWM1();
    fn PWM2();
    fn SPIM2_SPIS2_SPI2();
    fn RTC2();
    fn I2S();
    fn FPU();
    fn USBD();
    fn UARTE1();
    fn QSPI();
    fn CRYPTOCELL();
    fn PWM3();
    fn SPIM3();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 48] = [
    Vector {
        _handler: POWER_CLOCK,
    },
    Vector { _handler: RADIO },
    Vector {
        _handler: UARTE0_UART0,
    },
    Vector {
        _handler: SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0,
    },
    Vector {
        _handler: SPIM1_SPIS1_TWIM1_TWIS1_SPI1_TWI1,
    },
    Vector { _handler: NFCT },
    Vector { _handler: GPIOTE },
    Vector { _handler: SAADC },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: RTC0 },
    Vector { _handler: TEMP },
    Vector { _handler: RNG },
    Vector { _handler: ECB },
    Vector { _handler: CCM_AAR },
    Vector { _handler: WDT },
    Vector { _handler: RTC1 },
    Vector { _handler: QDEC },
    Vector {
        _handler: COMP_LPCOMP,
    },
    Vector {
        _handler: SWI0_EGU0,
    },
    Vector {
        _handler: SWI1_EGU1,
    },
    Vector {
        _handler: SWI2_EGU2,
    },
    Vector {
        _handler: SWI3_EGU3,
    },
    Vector {
        _handler: SWI4_EGU4,
    },
    Vector {
        _handler: SWI5_EGU5,
    },
    Vector { _handler: TIMER3 },
    Vector { _handler: TIMER4 },
    Vector { _handler: PWM0 },
    Vector { _handler: PDM },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: MWU },
    Vector { _handler: PWM1 },
    Vector { _handler: PWM2 },
    Vector {
        _handler: SPIM2_SPIS2_SPI2,
    },
    Vector { _handler: RTC2 },
    Vector { _handler: I2S },
    Vector { _handler: FPU },
    Vector { _handler: USBD },
    Vector { _handler: UARTE1 },
    Vector { _handler: QSPI },
    Vector {
        _handler: CRYPTOCELL,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PWM3 },
    Vector { _reserved: 0 },
    Vector { _handler: SPIM3 },
];
