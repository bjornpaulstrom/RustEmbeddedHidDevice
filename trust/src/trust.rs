//use hal_core::itm_driver::Itm;

use core::ops;
extern crate cortex_m;
use cortex_m::peripheral::*;
use cortex_m::register;

use cortex_m::interrupt::*;

/// static (global) counter to record number of overruns
/// optimized out by default
static mut NR_OVERRUNS: u32 = 0;

/// generic pending operation
/// if else structure will be optmized out at compile time
#[inline(always)]
//is_active<I>(&self, interrupt: I) -> bool

pub fn pend<I>(interrupt: I)
    where I: Nr + Copy
{
    if cfg!(feature = "trust_over_abort") {
        // abort on overrun
        unsafe {
            free(|_| if nvic().is_pending(interrupt) {
                assert!(false);
            } else {
                nvic().set_pending(interrupt);
            });
        }
    } else if cfg!(feature = "trust_over_cnt") {
        // increment counter on overrun
        unsafe {
            // per : why unsafe?
            free(|_| if nvic().is_pending(interrupt) {
                unsafe {
                    NR_OVERRUNS += 1;
                }
            } else {
                nvic().set_pending(interrupt);
            });
        }
    } else {
        nvic().set_pending(interrupt);
    }
}

/// collect statistics if feature "trust_over_cnt" enabled
pub fn get_nr_overruns() -> u32 {
    unsafe { NR_OVERRUNS }
}


/// Experimental
/// TRust resource
use core::cell::UnsafeCell;
use core::marker::PhantomData;

pub struct Res<T, I> {
    data: UnsafeCell<T>,
    res_ceiling: u32,
    old_ceiling: UnsafeCell<u32>,
    phantom: PhantomData<I>, // to ensure type uniqueness of each Res instance
}

#[must_use] // compiler will throw a warning for empty critical section
pub struct ResGuard<'a, T: 'a, I: 'a> {
    __lock: &'a Res<T, I>,
}

unsafe impl<T: Send, I> Sync for Res<T, I> {}

use core::mem::transmute;

impl<T, I> Res<T, I> {
    pub const fn new(value: T, ceiling: u32) -> Res<T, I> {
        Res {
            data: UnsafeCell::new(value),
            res_ceiling: ceiling,
            old_ceiling: UnsafeCell::new(0),
            phantom: PhantomData,
        }
    }

    //    pub const fn new_ref(addr: usize, ceiling: i32) -> Res<T, I> {
    //        Res {
    //            data: unsafe { *(transmute::<usize, &UnsafeCell<T>>(addr)) },
    //            res_ceiling: ceiling,
    //            old_ceiling: UnsafeCell::new(0),
    //            phantom: PhantomData,
    //        }
    //    }

    pub fn claim<'a>(&'a self) -> ResGuard<'a, T, I> {
        unsafe {
            *self.old_ceiling.get() = register::basepri::read() as u32;
            register::basepri_max::write(self.res_ceiling as u8);
        }
        // itm::Itm.putc_unsafe('+' as u8); // Debugging
        ResGuard { __lock: self }
    }

    // the opt_claim can only be used when no races may occur
    pub unsafe fn opt_claim<'a>(&'a self) -> ResGuard<'a, T, I> {
        *self.old_ceiling.get() = register::basepri::read() as u32;
        // itm::Itm.puts_unsafe("++"); // Debugging
        ResGuard { __lock: self }
    }
}

impl<'res, T, I> ops::Deref for ResGuard<'res, T, I> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.__lock.data.get() }
    }
}

impl<'res, T, I> ops::DerefMut for ResGuard<'res, T, I> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.__lock.data.get() }
    }
}

impl<'res, T, I> Drop for ResGuard<'res, T, I> {
    fn drop(&mut self) {
        unsafe {
            register::basepri::write(*self.__lock.old_ceiling.get() as u8);
        }
    }
}

// this code is just here for perfomance comparison
// this type of resourses should not be used
pub struct OptRes<T, I> {
    data: UnsafeCell<T>,
    phantom: PhantomData<I>, // to ensure type uniqueness of each Res instance
}

pub struct OptResGuard<'a, T: 'a, I: 'a> {
    pub __lock: &'a OptRes<T, I>,
}

unsafe impl<T: Send, I> Sync for OptRes<T, I> {}

impl<T, I> OptRes<T, I> {
    pub const fn new(value: T) -> OptRes<T, I> {
        OptRes {
            data: UnsafeCell::new(value),
            phantom: PhantomData,
        }
    }
    pub fn claim<'a>(&'a self) -> OptResGuard<'a, T, I> {
        //Itm.puts("+ +"); // Debugging
        OptResGuard { __lock: self }
    }
}

impl<'res, T, I> ops::Deref for OptResGuard<'res, T, I> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.__lock.data.get() }
    }
}

impl<'res, T, I> ops::DerefMut for OptResGuard<'res, T, I> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.__lock.data.get() }
    }
}

impl<'res, T, I> Drop for OptResGuard<'res, T, I> {
    fn drop(&mut self) {
        // Itm.puts("- -"); // Debugging
    }
}
