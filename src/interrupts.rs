# ! [ doc = r" Interrupts" ]
use cortex_m::exception;
use cortex_m::interrupt::Nr;
use cortex_m::{Handler, Reserved};
# [ doc = r" Interrupt handlers" ]
# [ repr ( C ) ]
pub struct Handlers {
    # [ doc = "Window Watchdog interrupt" ]
    pub wwdg: Handler,
    # [ doc = "PVD through EXTI line detection interrupt" ]
    pub pvd: Handler,
    # [ doc = "Tamper and TimeStamp interrupts through the EXTI line" ]
    pub tamp_stamp: Handler,
    # [ doc = "RTC Wakeup interrupt through the EXTI line" ]
    pub rtc_wkup: Handler,
    # [ doc = r" Reserved spot in the vector table" ]
    pub _reserved0: [Reserved; 1],
    # [ doc = "RCC global interrupt" ]
    pub rcc: Handler,
    # [ doc = "EXTI Line0 interrupt" ]
    pub exti0: Handler,
    # [ doc = "EXTI Line1 interrupt" ]
    pub exti1: Handler,
    # [ doc = "EXTI Line2 interrupt" ]
    pub exti2: Handler,
    # [ doc = "EXTI Line3 interrupt" ]
    pub exti3: Handler,
    # [ doc = "EXTI Line4 interrupt" ]
    pub exti4: Handler,
    # [ doc = "DMA1 Stream0 global interrupt" ]
    pub dma1_stream0: Handler,
    # [ doc = "DMA1 Stream1 global interrupt" ]
    pub dma1_stream1: Handler,
    # [ doc = "DMA1 Stream2 global interrupt" ]
    pub dma1_stream2: Handler,
    # [ doc = "DMA1 Stream3 global interrupt" ]
    pub dma1_stream3: Handler,
    # [ doc = "DMA1 Stream4 global interrupt" ]
    pub dma1_stream4: Handler,
    # [ doc = "DMA1 Stream5 global interrupt" ]
    pub dma1_stream5: Handler,
    # [ doc = "DMA1 Stream6 global interrupt" ]
    pub dma1_stream6: Handler,
    # [ doc = "ADC1 global interrupt" ]
    pub adc: Handler,
    # [ doc = "CAN1 TX interrupts" ]
    pub can1_tx: Handler,
    # [ doc = "CAN1 RX0 interrupts" ]
    pub can1_rx0: Handler,
    # [ doc = "CAN1 RX1 interrupts" ]
    pub can1_rx1: Handler,
    # [ doc = "CAN1 SCE interrupt" ]
    pub can1_sce: Handler,
    # [ doc = "EXTI Line[9:5] interrupts" ]
    pub exti9_5: Handler,
    # [ doc = "TIM1 Break interrupt and TIM9 global interrupt" ]
    pub tim1_brk_tim9: Handler,
    # [ doc = "TIM1 Update interrupt and TIM10 global interrupt" ]
    pub tim1_up_tim10: Handler,
    # [ doc = "TIM1 Trigger and Commutation interrupts and TIM11 global interrupt" ]
    pub tim1_trg_com_tim11: Handler,
    # [ doc = "TIM1 Capture Compare interrupt" ]
    pub tim1_cc: Handler,
    # [ doc = "TIM2 global interrupt" ]
    pub tim2: Handler,
    # [ doc = "TIM3 global interrupt" ]
    pub tim3: Handler,
    # [ doc = "TIM4 global interrupt" ]
    pub tim4: Handler,
    # [ doc = "I2C1 event interrupt" ]
    pub i2c1_ev: Handler,
    # [ doc = "I2C1 error interrupt" ]
    pub i2c1_er: Handler,
    # [ doc = "I2C2 event interrupt" ]
    pub i2c2_ev: Handler,
    # [ doc = "I2C2 error interrupt" ]
    pub i2c2_er: Handler,
    # [ doc = "SPI1 global interrupt" ]
    pub spi1: Handler,
    # [ doc = "SPI2 global interrupt" ]
    pub spi2: Handler,
    # [ doc = "USART1 global interrupt" ]
    pub usart1: Handler,
    # [ doc = "USART2 global interrupt" ]
    pub usart2: Handler,
    # [ doc = "USART3 global interrupt" ]
    pub usart3: Handler,
    # [ doc = "EXTI Line[15:10] interrupts" ]
    pub exti15_10: Handler,
    # [ doc = "RTC Alarms (A and B) through EXTI line interrupt" ]
    pub rtc_alarm: Handler,
    # [ doc = "USB On-The-Go FS Wakeup through EXTI line interrupt" ]
    pub otg_fs_wkup: Handler,
    # [ doc = "TIM8 Break interrupt and TIM12 global interrupt" ]
    pub tim8_brk_tim12: Handler,
    # [ doc = "TIM8 Update interrupt and TIM13 global interrupt" ]
    pub tim8_up_tim13: Handler,
    # [ doc = "TIM8 Trigger and Commutation interrupts and TIM14 global interrupt" ]
    pub tim8_trg_com_tim14: Handler,
    # [ doc = "TIM8 Capture Compare interrupt" ]
    pub tim8_cc: Handler,
    # [ doc = "DMA1 Stream7 global interrupt" ]
    pub dma1_stream7: Handler,
    # [ doc = "FSMC global interrupt" ]
    pub fsmc: Handler,
    # [ doc = "SDIO global interrupt" ]
    pub sdio: Handler,
    # [ doc = "TIM5 global interrupt" ]
    pub tim5: Handler,
    # [ doc = "SPI3 global interrupt" ]
    pub spi3: Handler,
    # [ doc = "UART4 global interrupt" ]
    pub uart4: Handler,
    # [ doc = "UART5 global interrupt" ]
    pub uart5: Handler,
    # [ doc = "TIM6 global interrupt, DAC1 and DAC2 underrun error interrupt" ]
    pub tim6_dac: Handler,
    # [ doc = "TIM7 global interrupt" ]
    pub tim7: Handler,
    # [ doc = "DMA2 Stream0 global interrupt" ]
    pub dma2_stream0: Handler,
    # [ doc = "DMA2 Stream1 global interrupt" ]
    pub dma2_stream1: Handler,
    # [ doc = "DMA2 Stream2 global interrupt" ]
    pub dma2_stream2: Handler,
    # [ doc = "DMA2 Stream3 global interrupt" ]
    pub dma2_stream3: Handler,
    # [ doc = "DMA2 Stream4 global interrupt" ]
    pub dma2_stream4: Handler,
    # [ doc = "Ethernet global interrupt" ]
    pub eth: Handler,
    # [ doc = "Ethernet Wakeup through EXTI line interrupt" ]
    pub eth_wkup: Handler,
    # [ doc = "CAN2 TX interrupts" ]
    pub can2_tx: Handler,
    # [ doc = "CAN2 RX0 interrupts" ]
    pub can2_rx0: Handler,
    # [ doc = "CAN2 RX1 interrupts" ]
    pub can2_rx1: Handler,
    # [ doc = "CAN2 SCE interrupt" ]
    pub can2_sce: Handler,
    # [ doc = "USB On The Go FS global interrupt" ]
    pub otg_fs: Handler,
    # [ doc = "DMA2 Stream5 global interrupt" ]
    pub dma2_stream5: Handler,
    # [ doc = "DMA2 Stream6 global interrupt" ]
    pub dma2_stream6: Handler,
    # [ doc = "DMA2 Stream7 global interrupt" ]
    pub dma2_stream7: Handler,
    # [ doc = "USART6 global interrupt" ]
    pub usart6: Handler,
    # [ doc = "I2C3 event interrupt" ]
    pub i2c3_ev: Handler,
    # [ doc = "I2C3 error interrupt" ]
    pub i2c3_er: Handler,
    # [ doc = "USB On The Go HS End Point 1 Out global interrupt" ]
    pub otg_hs_ep1_out: Handler,
    # [ doc = "USB On The Go HS End Point 1 In global interrupt" ]
    pub otg_hs_ep1_in: Handler,
    # [ doc = "USB On The Go HS Wakeup through EXTI interrupt" ]
    pub otg_hs_wkup: Handler,
    # [ doc = "USB On The Go HS global interrupt" ]
    pub otg_hs: Handler,
    # [ doc = "DCMI global interrupt" ]
    pub dcmi: Handler,
    # [ doc = r" Reserved spot in the vector table" ]
    pub _reserved1: [Reserved; 2],
    # [ doc = "FPU interrupt" ]
    pub fpu: Handler,
}
# [ doc = r" Default interrupt handlers" ]
pub const DEFAULT_HANDLERS: Handlers = Handlers {
    wwdg: exception::default_handler,
    pvd: exception::default_handler,
    tamp_stamp: exception::default_handler,
    rtc_wkup: exception::default_handler,
    _reserved0: [Reserved::Vector; 1],
    rcc: exception::default_handler,
    exti0: exception::default_handler,
    exti1: exception::default_handler,
    exti2: exception::default_handler,
    exti3: exception::default_handler,
    exti4: exception::default_handler,
    dma1_stream0: exception::default_handler,
    dma1_stream1: exception::default_handler,
    dma1_stream2: exception::default_handler,
    dma1_stream3: exception::default_handler,
    dma1_stream4: exception::default_handler,
    dma1_stream5: exception::default_handler,
    dma1_stream6: exception::default_handler,
    adc: exception::default_handler,
    can1_tx: exception::default_handler,
    can1_rx0: exception::default_handler,
    can1_rx1: exception::default_handler,
    can1_sce: exception::default_handler,
    exti9_5: exception::default_handler,
    tim1_brk_tim9: exception::default_handler,
    tim1_up_tim10: exception::default_handler,
    tim1_trg_com_tim11: exception::default_handler,
    tim1_cc: exception::default_handler,
    tim2: exception::default_handler,
    tim3: exception::default_handler,
    tim4: exception::default_handler,
    i2c1_ev: exception::default_handler,
    i2c1_er: exception::default_handler,
    i2c2_ev: exception::default_handler,
    i2c2_er: exception::default_handler,
    spi1: exception::default_handler,
    spi2: exception::default_handler,
    usart1: exception::default_handler,
    usart2: exception::default_handler,
    usart3: exception::default_handler,
    exti15_10: exception::default_handler,
    rtc_alarm: exception::default_handler,
    otg_fs_wkup: exception::default_handler,
    tim8_brk_tim12: exception::default_handler,
    tim8_up_tim13: exception::default_handler,
    tim8_trg_com_tim14: exception::default_handler,
    tim8_cc: exception::default_handler,
    dma1_stream7: exception::default_handler,
    fsmc: exception::default_handler,
    sdio: exception::default_handler,
    tim5: exception::default_handler,
    spi3: exception::default_handler,
    uart4: exception::default_handler,
    uart5: exception::default_handler,
    tim6_dac: exception::default_handler,
    tim7: exception::default_handler,
    dma2_stream0: exception::default_handler,
    dma2_stream1: exception::default_handler,
    dma2_stream2: exception::default_handler,
    dma2_stream3: exception::default_handler,
    dma2_stream4: exception::default_handler,
    eth: exception::default_handler,
    eth_wkup: exception::default_handler,
    can2_tx: exception::default_handler,
    can2_rx0: exception::default_handler,
    can2_rx1: exception::default_handler,
    can2_sce: exception::default_handler,
    otg_fs: exception::default_handler,
    dma2_stream5: exception::default_handler,
    dma2_stream6: exception::default_handler,
    dma2_stream7: exception::default_handler,
    usart6: exception::default_handler,
    i2c3_ev: exception::default_handler,
    i2c3_er: exception::default_handler,
    otg_hs_ep1_out: exception::default_handler,
    otg_hs_ep1_in: exception::default_handler,
    otg_hs_wkup: exception::default_handler,
    otg_hs: exception::default_handler,
    dcmi: exception::default_handler,
    _reserved1: [Reserved::Vector; 2],
    fpu: exception::default_handler,
};
# [ doc = r" Interrupts" ]
# [ derive(Copy, Clone) ]
pub enum Interrupt {
    # [ doc = "Window Watchdog interrupt" ]
    Wwdg,
    # [ doc = "PVD through EXTI line detection interrupt" ]
    Pvd,
    # [ doc = "Tamper and TimeStamp interrupts through the EXTI line" ]
    TampStamp,
    # [ doc = "RTC Wakeup interrupt through the EXTI line" ]
    RtcWkup,
    # [ doc = "RCC global interrupt" ]
    Rcc,
    # [ doc = "EXTI Line0 interrupt" ]
    Exti0,
    # [ doc = "EXTI Line1 interrupt" ]
    Exti1,
    # [ doc = "EXTI Line2 interrupt" ]
    Exti2,
    # [ doc = "EXTI Line3 interrupt" ]
    Exti3,
    # [ doc = "EXTI Line4 interrupt" ]
    Exti4,
    # [ doc = "DMA1 Stream0 global interrupt" ]
    Dma1Stream0,
    # [ doc = "DMA1 Stream1 global interrupt" ]
    Dma1Stream1,
    # [ doc = "DMA1 Stream2 global interrupt" ]
    Dma1Stream2,
    # [ doc = "DMA1 Stream3 global interrupt" ]
    Dma1Stream3,
    # [ doc = "DMA1 Stream4 global interrupt" ]
    Dma1Stream4,
    # [ doc = "DMA1 Stream5 global interrupt" ]
    Dma1Stream5,
    # [ doc = "DMA1 Stream6 global interrupt" ]
    Dma1Stream6,
    # [ doc = "ADC1 global interrupt" ]
    Adc,
    # [ doc = "CAN1 TX interrupts" ]
    Can1Tx,
    # [ doc = "CAN1 RX0 interrupts" ]
    Can1Rx0,
    # [ doc = "CAN1 RX1 interrupts" ]
    Can1Rx1,
    # [ doc = "CAN1 SCE interrupt" ]
    Can1Sce,
    # [ doc = "EXTI Line[9:5] interrupts" ]
    Exti95,
    # [ doc = "TIM1 Break interrupt and TIM9 global interrupt" ]
    Tim1BrkTim9,
    # [ doc = "TIM1 Update interrupt and TIM10 global interrupt" ]
    Tim1UpTim10,
    # [ doc = "TIM1 Trigger and Commutation interrupts and TIM11 global interrupt" ]
    Tim1TrgComTim11,
    # [ doc = "TIM1 Capture Compare interrupt" ]
    Tim1Cc,
    # [ doc = "TIM2 global interrupt" ]
    Tim2,
    # [ doc = "TIM3 global interrupt" ]
    Tim3,
    # [ doc = "TIM4 global interrupt" ]
    Tim4,
    # [ doc = "I2C1 event interrupt" ]
    I2c1Ev,
    # [ doc = "I2C1 error interrupt" ]
    I2c1Er,
    # [ doc = "I2C2 event interrupt" ]
    I2c2Ev,
    # [ doc = "I2C2 error interrupt" ]
    I2c2Er,
    # [ doc = "SPI1 global interrupt" ]
    Spi1,
    # [ doc = "SPI2 global interrupt" ]
    Spi2,
    # [ doc = "USART1 global interrupt" ]
    Usart1,
    # [ doc = "USART2 global interrupt" ]
    Usart2,
    # [ doc = "USART3 global interrupt" ]
    Usart3,
    # [ doc = "EXTI Line[15:10] interrupts" ]
    Exti1510,
    # [ doc = "RTC Alarms (A and B) through EXTI line interrupt" ]
    RtcAlarm,
    # [ doc = "USB On-The-Go FS Wakeup through EXTI line interrupt" ]
    OtgFsWkup,
    # [ doc = "TIM8 Break interrupt and TIM12 global interrupt" ]
    Tim8BrkTim12,
    # [ doc = "TIM8 Update interrupt and TIM13 global interrupt" ]
    Tim8UpTim13,
    # [ doc = "TIM8 Trigger and Commutation interrupts and TIM14 global interrupt" ]
    Tim8TrgComTim14,
    # [ doc = "TIM8 Capture Compare interrupt" ]
    Tim8Cc,
    # [ doc = "DMA1 Stream7 global interrupt" ]
    Dma1Stream7,
    # [ doc = "FSMC global interrupt" ]
    Fsmc,
    # [ doc = "SDIO global interrupt" ]
    Sdio,
    # [ doc = "TIM5 global interrupt" ]
    Tim5,
    # [ doc = "SPI3 global interrupt" ]
    Spi3,
    # [ doc = "UART4 global interrupt" ]
    Uart4,
    # [ doc = "UART5 global interrupt" ]
    Uart5,
    # [ doc = "TIM6 global interrupt, DAC1 and DAC2 underrun error interrupt" ]
    Tim6Dac,
    # [ doc = "TIM7 global interrupt" ]
    Tim7,
    # [ doc = "DMA2 Stream0 global interrupt" ]
    Dma2Stream0,
    # [ doc = "DMA2 Stream1 global interrupt" ]
    Dma2Stream1,
    # [ doc = "DMA2 Stream2 global interrupt" ]
    Dma2Stream2,
    # [ doc = "DMA2 Stream3 global interrupt" ]
    Dma2Stream3,
    # [ doc = "DMA2 Stream4 global interrupt" ]
    Dma2Stream4,
    # [ doc = "Ethernet global interrupt" ]
    Eth,
    # [ doc = "Ethernet Wakeup through EXTI line interrupt" ]
    EthWkup,
    # [ doc = "CAN2 TX interrupts" ]
    Can2Tx,
    # [ doc = "CAN2 RX0 interrupts" ]
    Can2Rx0,
    # [ doc = "CAN2 RX1 interrupts" ]
    Can2Rx1,
    # [ doc = "CAN2 SCE interrupt" ]
    Can2Sce,
    # [ doc = "USB On The Go FS global interrupt" ]
    OtgFs,
    # [ doc = "DMA2 Stream5 global interrupt" ]
    Dma2Stream5,
    # [ doc = "DMA2 Stream6 global interrupt" ]
    Dma2Stream6,
    # [ doc = "DMA2 Stream7 global interrupt" ]
    Dma2Stream7,
    # [ doc = "USART6 global interrupt" ]
    Usart6,
    # [ doc = "I2C3 event interrupt" ]
    I2c3Ev,
    # [ doc = "I2C3 error interrupt" ]
    I2c3Er,
    # [ doc = "USB On The Go HS End Point 1 Out global interrupt" ]
    OtgHsEp1Out,
    # [ doc = "USB On The Go HS End Point 1 In global interrupt" ]
    OtgHsEp1In,
    # [ doc = "USB On The Go HS Wakeup through EXTI interrupt" ]
    OtgHsWkup,
    # [ doc = "USB On The Go HS global interrupt" ]
    OtgHs,
    # [ doc = "DCMI global interrupt" ]
    Dcmi,
    # [ doc = "FPU interrupt" ]
    Fpu,
}
unsafe impl Nr for Interrupt {
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::Wwdg => 0,
            Interrupt::Pvd => 1,
            Interrupt::TampStamp => 2,
            Interrupt::RtcWkup => 3,
            Interrupt::Rcc => 5,
            Interrupt::Exti0 => 6,
            Interrupt::Exti1 => 7,
            Interrupt::Exti2 => 8,
            Interrupt::Exti3 => 9,
            Interrupt::Exti4 => 10,
            Interrupt::Dma1Stream0 => 11,
            Interrupt::Dma1Stream1 => 12,
            Interrupt::Dma1Stream2 => 13,
            Interrupt::Dma1Stream3 => 14,
            Interrupt::Dma1Stream4 => 15,
            Interrupt::Dma1Stream5 => 16,
            Interrupt::Dma1Stream6 => 17,
            Interrupt::Adc => 18,
            Interrupt::Can1Tx => 19,
            Interrupt::Can1Rx0 => 20,
            Interrupt::Can1Rx1 => 21,
            Interrupt::Can1Sce => 22,
            Interrupt::Exti95 => 23,
            Interrupt::Tim1BrkTim9 => 24,
            Interrupt::Tim1UpTim10 => 25,
            Interrupt::Tim1TrgComTim11 => 26,
            Interrupt::Tim1Cc => 27,
            Interrupt::Tim2 => 28,
            Interrupt::Tim3 => 29,
            Interrupt::Tim4 => 30,
            Interrupt::I2c1Ev => 31,
            Interrupt::I2c1Er => 32,
            Interrupt::I2c2Ev => 33,
            Interrupt::I2c2Er => 34,
            Interrupt::Spi1 => 35,
            Interrupt::Spi2 => 36,
            Interrupt::Usart1 => 37,
            Interrupt::Usart2 => 38,
            Interrupt::Usart3 => 39,
            Interrupt::Exti1510 => 40,
            Interrupt::RtcAlarm => 41,
            Interrupt::OtgFsWkup => 42,
            Interrupt::Tim8BrkTim12 => 43,
            Interrupt::Tim8UpTim13 => 44,
            Interrupt::Tim8TrgComTim14 => 45,
            Interrupt::Tim8Cc => 46,
            Interrupt::Dma1Stream7 => 47,
            Interrupt::Fsmc => 48,
            Interrupt::Sdio => 49,
            Interrupt::Tim5 => 50,
            Interrupt::Spi3 => 51,
            Interrupt::Uart4 => 52,
            Interrupt::Uart5 => 53,
            Interrupt::Tim6Dac => 54,
            Interrupt::Tim7 => 55,
            Interrupt::Dma2Stream0 => 56,
            Interrupt::Dma2Stream1 => 57,
            Interrupt::Dma2Stream2 => 58,
            Interrupt::Dma2Stream3 => 59,
            Interrupt::Dma2Stream4 => 60,
            Interrupt::Eth => 61,
            Interrupt::EthWkup => 62,
            Interrupt::Can2Tx => 63,
            Interrupt::Can2Rx0 => 64,
            Interrupt::Can2Rx1 => 65,
            Interrupt::Can2Sce => 66,
            Interrupt::OtgFs => 67,
            Interrupt::Dma2Stream5 => 68,
            Interrupt::Dma2Stream6 => 69,
            Interrupt::Dma2Stream7 => 70,
            Interrupt::Usart6 => 71,
            Interrupt::I2c3Ev => 72,
            Interrupt::I2c3Er => 73,
            Interrupt::OtgHsEp1Out => 74,
            Interrupt::OtgHsEp1In => 75,
            Interrupt::OtgHsWkup => 76,
            Interrupt::OtgHs => 77,
            Interrupt::Dcmi => 78,
            Interrupt::Fpu => 81,
        }
    }
}
