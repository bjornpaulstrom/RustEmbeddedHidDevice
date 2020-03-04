#![no_std]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(plugin)]

extern crate volatile_register;

pub mod base;
pub mod gpioa;
pub mod gpiob;
pub mod rcc;
pub mod usart2;
pub mod spi1;
pub mod exti;
pub mod i2c1;
