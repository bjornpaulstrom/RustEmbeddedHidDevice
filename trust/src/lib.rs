#![no_std]
#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(plugin)]
#![feature(const_fn)]

extern crate volatile_register;
extern crate cortex_m;

#[macro_use]
pub mod trust;
#[macro_use]
pub mod macros;
