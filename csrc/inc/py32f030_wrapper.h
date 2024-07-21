#ifndef PY32HAL_PY32F030_WRAPPER_H
#define PY32HAL_PY32F030_WRAPPER_H

#include "py32f0xx_hal.h"
#include "py32f0xx_hal_def.h"

void HAL_RCC_DMA_CLK_ENABLE();
void HAL_RCC_FLASH_CLK_ENABLE();
void HAL_RCC_SRAM_CLK_ENABLE();
void HAL_RCC_CRC_CLK_ENABLE();
void HAL_RCC_DMA_CLK_DISABLE();
void HAL_RCC_FLASH_CLK_DISABLE();
void HAL_RCC_SRAM_CLK_DISABLE();
void HAL_RCC_CRC_CLK_DISABLE();
void HAL_RCC_GPIOA_CLK_ENABLE();
void HAL_RCC_GPIOB_CLK_ENABLE();
void HAL_RCC_GPIOF_CLK_ENABLE();
void HAL_RCC_GPIOA_CLK_DISABLE();
void HAL_RCC_GPIOB_CLK_DISABLE();
void HAL_RCC_GPIOF_CLK_DISABLE();
void HAL_RCC_TIM3_CLK_ENABLE();
void HAL_RCC_RTCAPB_CLK_ENABLE();
void HAL_RCC_WWDG_CLK_ENABLE();
void HAL_RCC_SPI2_CLK_ENABLE();
void HAL_RCC_USART2_CLK_ENABLE();
void HAL_RCC_I2C_CLK_ENABLE();
void HAL_RCC_DBGMCU_CLK_ENABLE();
void HAL_RCC_PWR_CLK_ENABLE();
void HAL_RCC_LPTIM_CLK_ENABLE();
void HAL_RCC_SYSCFG_CLK_ENABLE();
void HAL_RCC_TIM1_CLK_ENABLE();
void HAL_RCC_SPI1_CLK_ENABLE();
void HAL_RCC_USART1_CLK_ENABLE();
void HAL_RCC_TIM14_CLK_ENABLE();
void HAL_RCC_TIM16_CLK_ENABLE();
void HAL_RCC_TIM17_CLK_ENABLE();
void HAL_RCC_ADC_CLK_ENABLE();
void HAL_RCC_COMP1_CLK_ENABLE();
void HAL_RCC_COMP2_CLK_ENABLE();
void HAL_RCC_LED_CLK_ENABLE();
void HAL_RCC_TIM3_CLK_DISABLE();
void HAL_RCC_RTCAPB_CLK_DISABLE();
void HAL_RCC_SPI2_CLK_DISABLE();
void HAL_RCC_USART2_CLK_DISABLE();
void HAL_RCC_I2C_CLK_DISABLE();
void HAL_RCC_DBGMCU_CLK_DISABLE();
void HAL_RCC_PWR_CLK_DISABLE();
void HAL_RCC_LPTIM_CLK_DISABLE();
void HAL_RCC_SYSCFG_CLK_DISABLE();
void HAL_RCC_TIM1_CLK_DISABLE();
void HAL_RCC_SPI1_CLK_DISABLE();
void HAL_RCC_USART1_CLK_DISABLE();
void HAL_RCC_TIM14_CLK_DISABLE();
void HAL_RCC_TIM16_CLK_DISABLE();
void HAL_RCC_TIM17_CLK_DISABLE();
void HAL_RCC_ADC_CLK_DISABLE();
void HAL_RCC_COMP1_CLK_DISABLE();
void HAL_RCC_COMP2_CLK_DISABLE();
void HAL_RCC_LED_CLK_DISABLE();
void HAL_RCC_DMA_IS_CLK_ENABLED();
void HAL_RCC_FLASH_IS_CLK_ENABLED();
void HAL_RCC_CRC_IS_CLK_ENABLED();
void HAL_RCC_DMA_IS_CLK_DISABLED();
void HAL_RCC_FLASH_IS_CLK_DISABLED();
void HAL_RCC_CRC_IS_CLK_DISABLED();
void HAL_RCC_GPIOA_IS_CLK_ENABLED();
void HAL_RCC_GPIOB_IS_CLK_ENABLED();
void HAL_RCC_GPIOF_IS_CLK_ENABLED();
void HAL_RCC_GPIOA_IS_CLK_DISABLED();
void HAL_RCC_GPIOB_IS_CLK_DISABLED();
void HAL_RCC_GPIOF_IS_CLK_DISABLED();
void HAL_RCC_TIM3_IS_CLK_ENABLED();
void HAL_RCC_RTCAPB_IS_CLK_ENABLED();
void HAL_RCC_WWDG_IS_CLK_ENABLED();
void HAL_RCC_SPI2_IS_CLK_ENABLED();
void HAL_RCC_USART2_IS_CLK_ENABLED();
void HAL_RCC_I2C_IS_CLK_ENABLED();
void HAL_RCC_DBGMCU_IS_CLK_ENABLED();
void HAL_RCC_PWR_IS_CLK_ENABLED();
void HAL_RCC_LPTIM_IS_CLK_ENABLED();
void HAL_RCC_TIM3_IS_CLK_DISABLED();
void HAL_RCC_RTCAPB_IS_CLK_DISABLED();
void HAL_RCC_WWDG_IS_CLK_DISABLED();
void HAL_RCC_SPI2_IS_CLK_DISABLED();
void HAL_RCC_USART2_IS_CLK_DISABLED();
void HAL_RCC_I2C1_IS_CLK_DISABLED();
void HAL_RCC_DBGMCU_IS_CLK_DISABLED();
void HAL_RCC_PWR_IS_CLK_DISABLED();
void HAL_RCC_LPTIM_IS_CLK_DISABLED();
void HAL_RCC_SYSCFG_IS_CLK_ENABLED();
void HAL_RCC_TIM1_IS_CLK_ENABLED();
void HAL_RCC_SPI1_IS_CLK_ENABLED();
void HAL_RCC_USART1_IS_CLK_ENABLED();
void HAL_RCC_TIM14_IS_CLK_ENABLED();
void HAL_RCC_TIM16_IS_CLK_ENABLED();
void HAL_RCC_TIM17_IS_CLK_ENABLED();
void HAL_RCC_ADC_IS_CLK_ENABLED();
void HAL_RCC_COMP1_IS_CLK_ENABLED();
void HAL_RCC_COMP2_IS_CLK_ENABLED();
void HAL_RCC_LED_IS_CLK_ENABLED();
void HAL_RCC_SYSCFG_IS_CLK_DISABLED();
void HAL_RCC_TIM1_IS_CLK_DISABLED();
void HAL_RCC_SPI1_IS_CLK_DISABLED();
void HAL_RCC_USART1_IS_CLK_DISABLED();
void HAL_RCC_TIM14_IS_CLK_DISABLED();
void HAL_RCC_TIM16_IS_CLK_DISABLED();
void HAL_RCC_TIM17_IS_CLK_DISABLED();
void HAL_RCC_ADC_IS_CLK_DISABLED();
void HAL_RCC_COMP1_IS_CLK_DISABLED();
void HAL_RCC_COMP2_IS_CLK_DISABLED();
void HAL_RCC_LED_IS_CLK_DISABLED();
void HAL_RCC_AHB_FORCE_RESET();
void HAL_RCC_DMA_FORCE_RESET();
void HAL_RCC_CRC_FORCE_RESET();
void HAL_RCC_AHB_RELEASE_RESET();
void HAL_RCC_DMA_RELEASE_RESET();
void HAL_RCC_CRC_RELEASE_RESET();
void HAL_RCC_IOP_FORCE_RESET();
void HAL_RCC_GPIOA_FORCE_RESET();
void HAL_RCC_GPIOB_FORCE_RESET();
void HAL_RCC_GPIOF_FORCE_RESET();
void HAL_RCC_IOP_RELEASE_RESET();
void HAL_RCC_GPIOA_RELEASE_RESET();
void HAL_RCC_GPIOB_RELEASE_RESET();
void HAL_RCC_GPIOF_RELEASE_RESET();
void HAL_RCC_APB1_FORCE_RESET();
void HAL_RCC_TIM3_FORCE_RESET();
void HAL_RCC_SPI2_FORCE_RESET();
void HAL_RCC_USART2_FORCE_RESET();
void HAL_RCC_I2C_FORCE_RESET();
void HAL_RCC_DBGMCU_FORCE_RESET();
void HAL_RCC_PWR_FORCE_RESET();
void HAL_RCC_LPTIM_FORCE_RESET();
void HAL_RCC_APB1_RELEASE_RESET();
void HAL_RCC_TIM3_RELEASE_RESET();
void HAL_RCC_SPI2_RELEASE_RESET();
void HAL_RCC_USART2_RELEASE_RESET();
void HAL_RCC_I2C_RELEASE_RESET();
void HAL_RCC_DBGMCU_RELEASE_RESET();
void HAL_RCC_PWR_RELEASE_RESET();
void HAL_RCC_LPTIM_RELEASE_RESET();
void HAL_RCC_APB2_FORCE_RESET();
void HAL_RCC_SYSCFG_FORCE_RESET();
void HAL_RCC_TIM1_FORCE_RESET();
void HAL_RCC_SPI1_FORCE_RESET();
void HAL_RCC_USART1_FORCE_RESET();
void HAL_RCC_TIM14_FORCE_RESET();
void HAL_RCC_TIM16_FORCE_RESET();
void HAL_RCC_TIM17_FORCE_RESET();
void HAL_RCC_ADC_FORCE_RESET();
void HAL_RCC_COMP1_FORCE_RESET();
void HAL_RCC_COMP2_FORCE_RESET();
void HAL_RCC_LED_FORCE_RESET();
void HAL_RCC_APB2_RELEASE_RESET();
void HAL_RCC_SYSCFG_RELEASE_RESET();
void HAL_RCC_TIM1_RELEASE_RESET();
void HAL_RCC_SPI1_RELEASE_RESET();
void HAL_RCC_USART1_RELEASE_RESET();
void HAL_RCC_TIM14_RELEASE_RESET();
void HAL_RCC_TIM16_RELEASE_RESET();
void HAL_RCC_TIM17_RELEASE_RESET();
void HAL_RCC_ADC_RELEASE_RESET();
void HAL_RCC_COMP1_RELEASE_RESET();
void HAL_RCC_COMP2_RELEASE_RESET();
void HAL_RCC_LED_RELEASE_RESET();
void HAL_RCC_BACKUPRESET_FORCE();
void HAL_RCC_BACKUPRESET_RELEASE();
void HAL_RCC_RTC_ENABLE();
void HAL_RCC_RTC_DISABLE();
void HAL_RCC_HSI_ENABLE();
void HAL_RCC_HSI_DISABLE();
void HAL_RCC_LSI_ENABLE();
void HAL_RCC_LSI_DISABLE();
void HAL_RCC_GET_RTC_SOURCE();
void HAL_RCC_PLL_ENABLE();
void HAL_RCC_PLL_DISABLE();
void HAL_RCC_GET_PLL_OSCSOURCE();
void HAL_RCC_GET_SYSCLK_SOURCE();
void HAL_RCC_CLEAR_RESET_FLAGS();

#endif //PY32HAL_PY32F030_WRAPPER_H
