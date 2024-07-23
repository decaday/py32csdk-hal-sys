
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

extern "C" {
    fn EXTI2_3_IRQHandler();
    fn TIM16_IRQHandler();
    fn PendSV_Handler();
    fn Reset_Handler();
    fn PVD_IRQHandler();
    fn HardFault_Handler();
    fn SPI2_IRQHandler();
    fn EXTI0_1_IRQHandler();
    fn DMA1_Channel2_3_IRQHandler();
    fn RCC_IRQHandler();
    fn TIM1_BRK_UP_TRG_COM_IRQHandler();
    fn TIM3_IRQHandler();
    fn LED_IRQHandler();
    fn RTC_IRQHandler();
    fn TIM1_CC_IRQHandler();
    fn TIM17_IRQHandler();
    fn USART2_IRQHandler();
    fn WWDG_IRQHandler();
    fn FLASH_IRQHandler();
    fn DMA1_Channel1_IRQHandler();
    fn TIM14_IRQHandler();
    fn SysTick_Handler();
    fn USART1_IRQHandler();
    fn SPI1_IRQHandler();
    fn EXTI4_15_IRQHandler();
    fn __initial_sp();
    fn ADC_COMP_IRQHandler();
    fn LPTIM1_IRQHandler();
    fn SVC_Handler();
    fn I2C1_IRQHandler();
    fn NMI_Handler();
    
}

#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 48] = [
    Vector { handler: __initial_sp },
    Vector { handler: Reset_Handler },
    Vector { handler: NMI_Handler },
    Vector { handler: HardFault_Handler },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVC_Handler },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV_Handler },
    Vector { handler: SysTick_Handler },
    Vector { handler: WWDG_IRQHandler },
    Vector { handler: PVD_IRQHandler },
    Vector { handler: RTC_IRQHandler },
    Vector { handler: FLASH_IRQHandler },
    Vector { handler: RCC_IRQHandler },
    Vector { handler: EXTI0_1_IRQHandler },
    Vector { handler: EXTI2_3_IRQHandler },
    Vector { handler: EXTI4_15_IRQHandler },
    Vector { reserved: 0 },
    Vector { handler: DMA1_Channel1_IRQHandler },
    Vector { handler: DMA1_Channel2_3_IRQHandler },
    Vector { reserved: 0 },
    Vector { handler: ADC_COMP_IRQHandler },
    Vector { handler: TIM1_BRK_UP_TRG_COM_IRQHandler },
    Vector { handler: TIM1_CC_IRQHandler },
    Vector { reserved: 0 },
    Vector { handler: TIM3_IRQHandler },
    Vector { handler: LPTIM1_IRQHandler },
    Vector { reserved: 0 },
    Vector { handler: TIM14_IRQHandler },
    Vector { reserved: 0 },
    Vector { handler: TIM16_IRQHandler },
    Vector { handler: TIM17_IRQHandler },
    Vector { handler: I2C1_IRQHandler },
    Vector { reserved: 0 },
    Vector { handler: SPI1_IRQHandler },
    Vector { handler: SPI2_IRQHandler },
    Vector { handler: USART1_IRQHandler },
    Vector { handler: USART2_IRQHandler },
    Vector { reserved: 0 },
    Vector { handler: LED_IRQHandler },
    Vector { reserved: 0 },
];
