
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;

pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

extern "C" {
    fn RCC();
    fn RTC();
    fn EXTI4_15();
    fn LED();
    fn TIM3();
    fn TIM1_CC();
    fn EXTI2_3();
    fn WWDG();
    fn USART2();
    fn USART1();
    fn TIM14();
    fn DMA1_CHANNEL2_3();
    fn TIM1_BRK_UP_TRG_COM();
    fn DMA1_CHANNEL1();
    fn TIM17();
    fn ADC_COMP();
    fn LPTIM1();
    fn TIM16();
    fn I2C1();
    fn SPI2();
    fn EXTI0_1();
    fn PVD();
    fn SPI1();
    fn FLASH();
    
}

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { handler: WWDG },
    Vector { handler: PVD },
    Vector { handler: RTC },
    Vector { handler: FLASH },
    Vector { handler: RCC },
    Vector { handler: EXTI0_1 },
    Vector { handler: EXTI2_3 },
    Vector { handler: EXTI4_15 },
    Vector { reserved: 0 },
    Vector { handler: DMA1_CHANNEL1 },
    Vector { handler: DMA1_CHANNEL2_3 },
    Vector { reserved: 0 },
    Vector { handler: ADC_COMP },
    Vector { handler: TIM1_BRK_UP_TRG_COM },
    Vector { handler: TIM1_CC },
    Vector { reserved: 0 },
    Vector { handler: TIM3 },
    Vector { handler: LPTIM1 },
    Vector { reserved: 0 },
    Vector { handler: TIM14 },
    Vector { reserved: 0 },
    Vector { handler: TIM16 },
    Vector { handler: TIM17 },
    Vector { handler: I2C1 },
    Vector { reserved: 0 },
    Vector { handler: SPI1 },
    Vector { handler: SPI2 },
    Vector { handler: USART1 },
    Vector { handler: USART2 },
    Vector { reserved: 0 },
    Vector { handler: LED },
    Vector { reserved: 0 },
];

#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - WWDG interrupt"]
    WWDG = 0,
    #[doc = "1 - PVD interrupt"]
    PVD = 1,
    #[doc = "2 - RTC interrupt"]
    RTC = 2,
    #[doc = "3 - FLASH interrupt"]
    FLASH = 3,
    #[doc = "4 - RCC interrupt"]
    RCC = 4,
    #[doc = "5 - EXTI0_1 interrupt"]
    EXTI0_1 = 5,
    #[doc = "6 - EXTI2_3 interrupt"]
    EXTI2_3 = 6,
    #[doc = "7 - EXTI4_15 interrupt"]
    EXTI4_15 = 7,
    #[doc = "9 - DMA1_CHANNEL1 interrupt"]
    DMA1_CHANNEL1 = 9,
    #[doc = "10 - DMA1_CHANNEL2_3 interrupt"]
    DMA1_CHANNEL2_3 = 10,
    #[doc = "12 - ADC_COMP interrupt"]
    ADC_COMP = 12,
    #[doc = "13 - TIM1_BRK_UP_TRG_COM interrupt"]
    TIM1_BRK_UP_TRG_COM = 13,
    #[doc = "14 - TIM1_CC interrupt"]
    TIM1_CC = 14,
    #[doc = "16 - TIM3 interrupt"]
    TIM3 = 16,
    #[doc = "17 - LPTIM1 interrupt"]
    LPTIM1 = 17,
    #[doc = "19 - TIM14 interrupt"]
    TIM14 = 19,
    #[doc = "21 - TIM16 interrupt"]
    TIM16 = 21,
    #[doc = "22 - TIM17 interrupt"]
    TIM17 = 22,
    #[doc = "23 - I2C1 interrupt"]
    I2C1 = 23,
    #[doc = "25 - SPI1 interrupt"]
    SPI1 = 25,
    #[doc = "26 - SPI2 interrupt"]
    SPI2 = 26,
    #[doc = "27 - USART1 interrupt"]
    USART1 = 27,
    #[doc = "28 - USART2 interrupt"]
    USART2 = 28,
    #[doc = "30 - LED interrupt"]
    LED = 30,
}

unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}