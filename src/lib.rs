//! Based on `cortex-m-template`
//!
//! https://github.com/japaric/cortex-m-template

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![no_std]

extern crate compiler_builtins;
#[cfg_attr(feature = "semihosting",
           macro_reexport(ehprint, ehprintln, hprint, hprintln))]
#[macro_reexport(bkpt)]
#[macro_use]
extern crate cortex_m;
pub use cortex_m::peripheral::nvic;
extern crate r0;

//#[macro_use]
extern crate trust;
//#[macro_reexport(hw_prio)]

mod lang_items;

pub mod exceptions;
pub mod interrupts;
