//! Example application
#![no_std]
#![feature(plugin)]
#![plugin(interpolate_idents)]
#![feature(const_fn)]

extern crate jap;

#[macro_use]
extern crate hal_core;
//use hal_core::itm_driver::*;
//use core::fmt::Write;

extern crate cortex_m;
use cortex_m::peripheral::*;

#[macro_use]
extern crate trust;

use trust::trust::*;

extern crate cortex_m_srp;
use cortex_m_srp::*;

use jap::{exceptions, interrupts};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

static R1: Resource<Point> = Resource::new(Point { x: 10, y: 10 }, 2);
static R2: Resource<Point> = Resource::new(Point { x: 0, y: 0 }, 3);


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

task!(j1, interrupts::Interrupt::Exti1, hw_prio!(1));
pub extern "C" fn j1() {
    println!("j1 enter");
    R2.claim(|r2| {
        println!("j1 R2 enter");
        pend(TASK_IRQ_j2);
        println!("j1 after job request (pend)");
        R1.claim_mut(|r1| {
            println!("j1 R1 enter");
            r1.x = 1;
            pend!(j2);
            r1.y = 2;
            println!("j1 R1 {:?}", *r1);
            println!("j1 R1 exit");
        });
        pend!(j3);
        println!("j1 R2 {:?}", *r2);
        println!("j1 R2 exit");
    });
    println!("j1 exit");
}

task!(j2, interrupts::Interrupt::Exti2, hw_prio!(2));
pub extern "C" fn j2() {
    println!("j2 enter");
    R1.claim_mut(|r1| {
        println!("j2 R1 enter");
        *r1 = Point { x: 2, y: 1 };
        println!("j2 R1 {:?}", *r1);
        println!("j2 R1 exit");
    });
    println!("j2 exit");
}

task!(j3, interrupts::Interrupt::Exti3, hw_prio!(3));
pub extern "C" fn j3() {
    println!("j3 enter");
    R2.claim_mut(|r2| {
        *r2 = Point {
            x: r2.x + 5,
            y: r2.y + 5,
        };
        if r2.x < 10 {
            pend!(j1);
        }
        println!("j3 R2 {:?}", *r2);
    });
    println!("j3 exit");
}

fn main() {
    nvic().set_priority(TASK_IRQ_j1, TASK_PRIO_j1);
    nvic().set_priority(TASK_IRQ_j2, TASK_PRIO_j2);
    nvic().set_priority(TASK_IRQ_j3, TASK_PRIO_j3);

    println!("j1 get_priority {:x} {:x}",
             TASK_PRIO_j1,
             nvic().get_priority(TASK_IRQ_j1));
    println!("j2 get_priority {:x} {:x}",
             TASK_PRIO_j2,
             nvic().get_priority(TASK_IRQ_j2));
    println!("j3 get_priority {:x} {:x}",
             TASK_PRIO_j3,
             nvic().get_priority(TASK_IRQ_j3));

    nvic().enable(TASK_IRQ_j1);
    nvic().enable(TASK_IRQ_j2);
    nvic().enable(TASK_IRQ_j3);
    pend!(j3);
}

// The program must specify how exceptions will be handled
// Here we just use the default handler to handle all the exceptions
#[no_mangle]
pub static _EXCEPTIONS: exceptions::Handlers =
    exceptions::Handlers { ..exceptions::DEFAULT_HANDLERS };

// Likewise with interrupts
#[no_mangle]
pub static _INTERRUPTS: interrupts::Handlers = interrupts::Handlers {
    exti1: j1,
    exti2: j2,
    exti3: j3,
    ..interrupts::DEFAULT_HANDLERS
};
