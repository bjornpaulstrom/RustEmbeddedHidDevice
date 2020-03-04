extern crate hal_core;
use self::hal_core::*;

extern crate stm32f401;
use self::stm32f401::base::*;
use self::stm32f401::gpioa::*;
use self::stm32f401::usart2::*;
use self::stm32f401::rcc::*;

pub static mut usart: USART = USART {};

pub struct USART
{
	
}

pub fn usart2() -> &'static mut Usart2 {
    unsafe { &mut *(USART2_BASE as *mut Usart2) }
}

pub fn gpioa() -> &'static mut Gpioa {
    unsafe { &mut *(GPIOA_BASE as *mut Gpioa) }
}

pub fn rcc() -> &'static mut Rcc {
    unsafe { &mut *(RCC_BASE as *mut Rcc) }
}

impl USART
{
	pub fn initialize(&self)
	{
	    gpioa().moder.modify(|_, w| w.moder2().bits(2));
	    gpioa().moder.modify(|_, w| w.moder3().bits(2));
	    gpioa().otyper.modify(|_, w| w.ot2().bits(0));
	    gpioa().otyper.modify(|_, w| w.ot3().bits(0));
	    gpioa().afrl.modify(|_, w| w.afrl2().bits(7));
	    gpioa().afrl.modify(|_, w| w.afrl3().bits(7));
	
	    rcc().apb1enr.modify(|_, w| unsafe { w.usart2en().bits(1) }); //enable usart2
	    usart2().cr1.modify(|_, w| unsafe { w.over8().bits(0) }); //16bitoversampling
	
	    usart2().brr.modify(|_, w| unsafe { w.div_fraction().bits(0x7) }); //Baudrate fraction
	    usart2().brr.modify(|_, w| unsafe { w.div_mantissa().bits(0x111) }); //Baudrate Mantissa
	
	    usart2().cr1.modify(|_, w| unsafe { w.ue().bits(1) }); //USART enable
	    usart2().cr1.modify(|_, w| unsafe { w.te().bits(1) }); //Transmit enable
	    usart2().cr1.modify(|_, w| unsafe { w.re().bits(1) }); //Receieve enable
	}
	
	//Brief:	Send a series of chars through usart2
	//data: 	chars to send
	//return:	none
	pub fn send(&self, data: &str) 
	{
	    for c in data.chars() 
	    {
	        while usart2().sr.read().txe().bits() == 0 {}
	        usart2().dr.write(|w| unsafe { w.bits(c as u32) });
	    }
	}
}