//! This program crashes!
//!
//! The goal is to demonstrate what happens when the program hits an exception
//! and how the debug the problem.

#![no_std]

extern crate jap;

use core::ptr;

use jap::exceptions::{self, Exceptions};
use jap::interrupts::{self, Interrupts};

fn main() {
    unsafe {
        ptr::read_volatile(0x2FFF_FFFF as *const u32);
    }
}

#[no_mangle]
pub static _EXCEPTIONS: Exceptions =
    Exceptions { ..exceptions::DEFAULT_HANDLERS };

#[no_mangle]
pub static _INTERRUPTS: Interrupts =
    Interrupts { ..interrupts::DEFAULT_HANDLERS };
