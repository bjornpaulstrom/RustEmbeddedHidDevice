//! Example application
#![no_std]
#![feature(plugin)]
#![plugin(interpolate_idents)]
#![feature(const_fn)]

extern crate jap;

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

use trust::trust::*;
use trust::macros::compiler_barrier;

#[derive(Debug)]
struct Vector {
    x: u32,
    y: u32,
}

const R1_CEILING: u32 = hw_prio!(2) << 4; // max_prio {task1, task2, task3}
resource!(R1, Vector, Vector { x: 0, y: 0 });

const R2_CEILING: u32 = hw_prio!(1) << 4; // max_prio {task1, task2, task3}
resource!(R2, Vector, Vector { x: 10, y: 10 });


use jap::{exceptions, interrupts};

macro_rules! task {
  ($task_id: ident, $task_irq: expr, $task_prio: expr) => ( interpolate_idents! {
      #[allow(non_upper_case_globals)] pub const [TASK_PRIO_ $task_id] : u8 = $task_prio;
      #[allow(non_upper_case_globals)] pub const [TASK_IRQ_ $task_id] : interrupts::Interrupt = $task_irq; 
  } )
}

macro_rules! pend {
  ($task_id: ident) => ( interpolate_idents! {
      pend ([TASK_IRQ_ $task_id] ); 
  } )
}

macro_rules! R {
  ($task_id: ident) => ( interpolate_idents! {
      pend ([TASK_IRQ_ $task_id] ); 
  } )
}

task!(j1, interrupts::Interrupt::Exti1, hw_prio!(1));

type R1t = &'static Res<Vector, PhantomR1>;

fn j1(r1r: R1t) {
    println!("j1");
    {
        let mut r1 = (*r1r).claim();
        println!("j1 claim R1 enter");
        //
        (*r1).x = 1;
        //        pend!(j2);
        //        r1.y = 2;
        println!("R1 {:?}", *r1);

        println!("j1 claim R1 exit");
    }

    /*
    {
        let r2 = R2.claim();
        pend(TASK_IRQ_j2);

        {
            let mut r1 = R1.claim();
            println!("j1 claim R1 enter");

            r1.x = 1;
            pend!(j2);
            r1.y = 2;
            println!("R1 {:?}", *r1);

            println!("j1 claim R1 exit");
        }
        pend!(j3);
        println!("R2 {:?}", *r2);
    }
    */
}

pub extern "C" fn j1_handler() {
    j1(&R1);
}

task!(j2, interrupts::Interrupt::Exti2, hw_prio!(2));
pub extern "C" fn j2() {
    println!("j2 enter");
    {
        let mut r1 = R1.claim();
        println!("j2 claim R1 enter");

        *r1 = Vector { x: 0, y: 0 };
        println!("R1 {:?}", *r1);

        println!("j2 claim R1 exit");
    }
    println!("j2 exit");
}

task!(j3, interrupts::Interrupt::Exti3, hw_prio!(3));
pub extern "C" fn j3() {
    println!("j3 enter");
    {
        let mut r2 = R2.claim();
        *r2 = Vector {
            x: r2.x + 5,
            y: r2.y + 5,
        };
    }
    pend!(j1);
    println!("j3 exit");
}

fn main() {
    nvic().set_priority(TASK_IRQ_j1, TASK_PRIO_j1);
    nvic().set_priority(TASK_IRQ_j2, TASK_PRIO_j2);
    nvic().set_priority(TASK_IRQ_j3, TASK_PRIO_j3);

    println!("j1 get_priority {}", nvic().get_priority(TASK_IRQ_j1));
    println!("j2 get_priority {}", nvic().get_priority(TASK_IRQ_j2));
    println!("j3 get_priority {}", nvic().get_priority(TASK_IRQ_j3));

    nvic().enable(TASK_IRQ_j1);
    nvic().enable(TASK_IRQ_j2);
    nvic().enable(TASK_IRQ_j3);
    pend!(j3);

    println!("in claim");
}

// The program must specify how exceptions will be handled
// Here we just use the default handler to handle all the exceptions
#[no_mangle]
pub static _EXCEPTIONS: exceptions::Handlers =
    exceptions::Handlers { ..exceptions::DEFAULT_HANDLERS };

// Likewise with interrupts
#[no_mangle]
pub static _INTERRUPTS: interrupts::Handlers = interrupts::Handlers {
    exti1: j1_handler,
    exti2: j2,
    exti3: j3,
    ..interrupts::DEFAULT_HANDLERS
};
