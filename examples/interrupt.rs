//! Example application
#![no_std]
#![feature(plugin)]
#![plugin(interpolate_idents)]
#![feature(const_fn)]

//#[macro_use]
extern crate jap;
//#[macro_use]
extern crate hal_core;
use hal_core::*;
use hal_core::itm_driver::*;
use core::fmt::Write;

extern crate stm32f401;
use stm32f401::base::*;
use stm32f401::gpioa::*;
use stm32f401::rcc::*;


extern crate cortex_m;
use cortex_m::{register, interrupt};
use cortex_m::peripheral::nvic;
use jap::{exceptions, interrupts};

pub fn gpioa() -> &'static mut Gpioa {
    unsafe { &mut *(GPIOA_BASE as *mut Gpioa) }
}

pub fn rcc() -> &'static mut Rcc {
    unsafe { &mut *(RCC_BASE as *mut Rcc) }
}

// We need a `main` function, just like every other Rust program
fn main() {

    unsafe {
        // per : why unsafe?
        interrupt::free(|_| {
            println!("enter critical section");
            nvic().set_priority(interrupts::Interrupt::Exti1, 0); // highest
            nvic().set_priority(interrupts::Interrupt::Exti2, 1);
            nvic().set_priority(interrupts::Interrupt::Exti3, 15); // lowest, still higher than idle
            println!("Exti1  get_priority {}",
                     nvic().get_priority(interrupts::Interrupt::Exti1));
            println!("Exti2  get_priority {}",
                     nvic().get_priority(interrupts::Interrupt::Exti2));
            println!("Exti3  get_priority {}",
                     nvic().get_priority(interrupts::Interrupt::Exti3));
            nvic().enable(interrupts::Interrupt::Exti1);
            nvic().enable(interrupts::Interrupt::Exti2);
            nvic().enable(interrupts::Interrupt::Exti3);
            nvic().set_pending(interrupts::Interrupt::Exti1);
            nvic().set_pending(interrupts::Interrupt::Exti2);
            nvic().set_pending(interrupts::Interrupt::Exti3);
            println!("exit critical section");
        }); // interrupts will be handled (serviced) here
    }

    println!("after critical section");
}

/// exti1 handler
pub extern "C" fn exti1_handler() {
    println!("exti1_handler");
}

/// exti2 handler
pub extern "C" fn exti2_handler() {
    println!("exti2_handler");
}

/// exti3 handler
pub extern "C" fn exti3_handler() {
    println!("exti3_handler");
}

// The program must specify how exceptions will be handled
// Here we just use the default handler to handle all the exceptions
#[no_mangle]
pub static _EXCEPTIONS: exceptions::Handlers =
    exceptions::Handlers { ..exceptions::DEFAULT_HANDLERS };

// Likewise with interrupts
#[no_mangle]
pub static _INTERRUPTS: interrupts::Handlers = interrupts::Handlers {
    exti1: exti1_handler,
    exti2: exti2_handler,
    exti3: exti3_handler,
    ..interrupts::DEFAULT_HANDLERS
};
