// ITM support with lean print/prinln support
// Per Lindgren, per.lindgren@ltu.se
// inspired by the Nix development by Nathan Zadoks (nathan@nathan7.eu)
// and the f3 crate by japaric https://github.com/japaric/f3

// Typical C implementation, example from STM HAL
// __STATIC_INLINE uint32_t ITM_SendChar (uint32_t ch)
// {
// if (((ITM->TCR & ITM_TCR_ITMENA_Msk) != 0UL) &&      /* ITM enabled */
// ((ITM->TER & 1UL               ) != 0UL)   )     /* ITM Port #0 enabled */
// {
// while (ITM->PORT[0U].u32 == 0UL)
// {
// __NOP();
// }
// ITM->PORT[0U].u8 = (uint8_t)ch;
// }
// return (ch);
// }
//
//

pub use core::fmt::{self, Arguments, Write};

use core::intrinsics::{volatile_load, volatile_store};

use base;

const ITM_PORT: usize = base::ITM;

fn putc(ch: u8) {
    while unsafe { volatile_load(ITM_PORT as *mut u8) } == 0 {}
    // at this point we know the port is ready
    unsafe { volatile_store(ITM_PORT as *mut u8, ch) };
}

pub struct Itm {}

impl Write for Itm {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            putc(byte);
        }
        Ok(())
    }
}

pub fn write_fmt(args: Arguments) {
    Itm {}.write_fmt(args).ok();
}

pub fn write_str(s: &str) {
    Itm {}.write_str(s).ok();
}

// unbuffered itm printing
#[macro_export]
macro_rules! print {
    ($s:expr) => {
	    $crate::itm_driver::write_str($s)       	
    };
	($($arg:tt)*) => {
	    $crate::itm_driver::write_fmt(format_args!($($arg)*))
	};
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!(concat!($fmt, "\n"), $($arg)*)
    };
}
