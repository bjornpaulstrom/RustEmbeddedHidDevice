//! What happens when `panic!` is invoked?
//!
//! Find out with this app

#![no_std]

extern crate jap;

use jap::exceptions::{self, Exceptions};
use jap::interrupts::{self, Interrupts};

fn main() {
    panic!()
}

#[no_mangle]
pub static _EXCEPTIONS: Exceptions =
    Exceptions { ..exceptions::DEFAULT_HANDLERS };

#[no_mangle]
pub static _INTERRUPTS: Interrupts =
    Interrupts { ..interrupts::DEFAULT_HANDLERS };
