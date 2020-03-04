
//! Interface to System Control Block.
//!
//! Data Watchpoint and Trace Unit memory location is 0xE0001000.
//! Link: DDI0403E_B_armv7m_arm.pdf (requires registration)

// use base;

use dwt::*;
use base::DWT;

#[inline(always)]
fn dwt() -> &'static mut Dwt {
    unsafe { &mut *(DWT as *mut Dwt) }
}

pub fn get_cyccntena() -> bool {
    dwt().ctrl.read().cyccntena().is_enabled()
}

pub fn set_cyccntena() {
    dwt().ctrl.modify(|_, w| w.cyccntena().enabled());
}

pub fn get_cyccnt() -> u32 {
    dwt().cyccnt.read().bits()
}
// Wait val number of cycles, burns CPU
// works on u32, 0x7FFF_FFFF is maximum number cycles to wait
// max 21 seconds on a 100Mz core clock
// #[inline(always)]
pub fn wait_to_cycle(stop: u32) {
    loop {
        if stop.wrapping_sub(get_cyccnt()) as i32 <= 0 {
            break;
        }
    }
}
// wait number off cycles relative to current
// #[inline(always)]
pub fn wait_cycles(val: u32) {
    let start: u32 = get_cyccnt();
    let stop: u32 = start.wrapping_add(val);

    wait_to_cycle(stop);
}
