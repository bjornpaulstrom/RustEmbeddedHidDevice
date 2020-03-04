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
use cortex_m::peripheral::*;
use cortex_m::register;
//use core::u32;

#[macro_use]
extern crate trust;
#[macro_use]
use trust::trust::*;
use trust::macros::compiler_barrier;

struct Vector {
    x: u32,
    y: u32,
}

const A_CEILING: u32 = hw_prio!(3) << 4; // max_prio {task1, task2, task3}
resource!(A, Vector, Vector { x: 0, y: 0 });

use jap::{exceptions, interrupts};

pub fn gpioa() -> &'static mut Gpioa {
    unsafe { &mut *(GPIOA_BASE as *mut Gpioa) }
}

pub fn rcc() -> &'static mut Rcc {
    unsafe { &mut *(RCC_BASE as *mut Rcc) }
}

pub fn led_on() {
    gpioa().bsrr.write(|w| w.bs5().variant(bsrr::BsrrW::SetResetBit));
    println!("on");
}

pub fn led_off() {
    gpioa().bsrr.write(|w| w.br5().variant(bsrr::BsrrW::SetResetBit));
    println!("off");
}


// We need a `main` function, just like every other Rust program
fn main() {
    //rcc().ahb1enrfo.write(|w| w.gpioaen().bits(1));
    //rcc().ahb1enr.write(|w| w.gpioaen().enable());
    //rcc().ahb1enr.write(|w| w.gpioaen().variant(ahb1enr::EnableW::Enable));
    //rcc().ahb1enr.modify(|_, w| w.gpioben().variant(ahb1enr::EnableW::Enable));

    let bp = unsafe {
        register::basepri::read();
    };

    claim!(A, a, {
        nvic().set_priority(interrupts::Interrupt::Exti1, hw_prio!(16) as u8);
        nvic().set_priority(interrupts::Interrupt::Exti2, hw_prio!(15) as u8);
        nvic().set_priority(interrupts::Interrupt::Exti3, hw_prio!(1) as u8);
        println!("hw_prio {} get_priority {}",
                 hw_prio!(16) as u8,
                 nvic().get_priority(interrupts::Interrupt::Exti1));
        println!("hw_prio {} get_priority {}",
                 hw_prio!(15) as u8,
                 nvic().get_priority(interrupts::Interrupt::Exti2));
        println!("hw_prio {} get_priority {}",
                 hw_prio!(1) as u8,
                 nvic().get_priority(interrupts::Interrupt::Exti3));
        nvic().enable(interrupts::Interrupt::Exti1);
        nvic().enable(interrupts::Interrupt::Exti2);
        nvic().enable(interrupts::Interrupt::Exti3);
        nvic().set_pending(interrupts::Interrupt::Exti1);
        nvic().set_pending(interrupts::Interrupt::Exti2);
        nvic().set_pending(interrupts::Interrupt::Exti3);
        println!("in claim");
    });

    println!("after claim");

    rcc()
        .ahb1enr
        .modify(|_, w| {
            w.gpioaen()
                .variant(ahb1enr::EnableW::Enable) // enable the usart
                .gpioben()
                .variant(ahb1enr::EnableW::Enable) // enable the gpio
        });

    /*
    rcc().ahb1enr.modify(|_, w| {
        w.gpioaen()
            .enable()
            .gpioben()
            .enable()
    });
    */
    rcc().ahb1enr.modify(|_, w| w.gpioben().variant(ahb1enr::EnableW::Enable));
    if rcc().ahb1enr.read().gpioaen().is_enable() {
        println!("is_enable");
    }
    if rcc().ahb1enr.read().gpioben().is_enable() {
        println!("is_enable");
    }

    let gp = gpioa();

    gp.moder.write(|w| w.moder5().variant(moder::ModerW::OutputMode));
    gp.otyper.write(|w| w.ot5().push_pull());

    dwt_driver::set_cyccntena();
    loop {
        dwt_driver::wait_cycles(64_000_000); // 1 second
        led_on();
        dwt_driver::wait_cycles(64_000_000); // 1 second
        led_off();
    }
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
