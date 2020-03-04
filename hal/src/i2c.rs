extern crate stm32f401;
use self::stm32f401::gpioa::*;
use self::stm32f401::gpiob::*;
use self::stm32f401::base::*;
use self::stm32f401::rcc::*;
use self::stm32f401::i2c1::*;

extern crate hal_core;
use self::hal_core::*;

pub struct I2C {}

pub fn gpioa() -> &'static mut Gpioa {
    unsafe { &mut *(GPIOA_BASE as *mut Gpioa) }
}

pub fn gpiob() -> &'static mut Gpiob {
    unsafe { &mut *(GPIOB_BASE as *mut Gpiob) }
}

pub fn rcc() -> &'static mut Rcc {
    unsafe { &mut *(RCC_BASE as *mut Rcc) }
}

pub fn i2c1() -> &'static mut I2c1 {
    unsafe { &mut *(I2C1_BASE as *mut I2c1) }
}

pub static i2c: I2C = I2C {};

impl I2C
{
	pub fn initialize(&self)
	{
		rcc().apb1enr.modify(|_, w| unsafe { w.i2c1en().bits(1) });
        
        i2c1().cr2.modify(|_, w| unsafe { w.freq().bits(0x2a) });
        i2c1().trise.modify(|_, w| unsafe { w.trise().bits(0x0d) });
        i2c1().ccr.modify(|_, w| unsafe { w.f_s().bits(1) });
        i2c1().ccr.modify(|_, w| unsafe { w.ccr().bits(0x23) });
        i2c1().oar1.modify(|_, w| unsafe { w.bits(0x4033) });
        i2c1().cr1.modify(|_, w| unsafe { w.pe().bits(1) });
        
        gpiob().moder.modify(|_, w| unsafe { w.moder8().bits(2) }); 
        gpiob().otyper.modify(|_, w| unsafe { w.ot8().bits(1) });
        gpiob().ospeedr.modify(|_, w| unsafe { w.ospeedr8().bits(3) });
        gpiob().afrh.modify(|_, w| unsafe { w.afrh8().bits(4) });
        gpiob().pupdr.modify(|_, w| unsafe { w.pupdr8().bits(1) });
        
        gpiob().moder.modify(|_, w| unsafe { w.moder9().bits(2) }); 
        gpiob().otyper.modify(|_, w| unsafe { w.ot9().bits(1) });
        gpiob().ospeedr.modify(|_, w| unsafe { w.ospeedr9().bits(3) });
        gpiob().afrh.modify(|_, w| unsafe { w.afrh9().bits(4) });
        gpiob().pupdr.modify(|_, w| unsafe { w.pupdr9().bits(1) });
        
        //rcc().apb1rstr.modify(|_, w| unsafe { w.i2c1rst().bits(1) });
        //rcc().apb1rstr.modify(|_, w| unsafe { w.i2c1rst().bits(0) });
        
        i2c1().cr1.modify(|_, w| unsafe { w.pe().bits(1) });
	}
	
	pub fn read_memory(&self, memory_address: u32) -> u32
	{
	    /* Disable Pos */
	    i2c1().cr1.modify(|_, w| unsafe { w.pos().bits(0) });
	    
	    /* Send Slave Address and Memory Address */
	    self.request_memory_read(memory_address);
	    
	    //Disable Acknowledge
		i2c1().cr1.modify(|_, w| unsafe { w.ack().bits(0) });
	    
		//Clear ADDR flag; This bit is cleared by software reading SR1 register followed reading SR2, or by hardware when PE=0.
		i2c1().sr1.read();
		i2c1().sr2.read();
	    
		//Generate stop
		i2c1().cr1.modify(|_, w| unsafe { w.stop().bits(1) });
		
		/* Wait until RXNE flag is set */
		while i2c1().sr1.read().rx_ne().bits() == 0 {}
		
		let value :u32 = i2c1().dr.read().bits();
		
		value
		
	}
	
	pub fn request_memory_read(&self, memory_address: u32)
	{
		//Enable Acknowledge
		i2c1().cr1.modify(|_, w| unsafe { w.ack().bits(1) });

		//Generate Start
		i2c1().cr1.modify(|_, w| unsafe { w.start().bits(1) });
		
		/* Wait until SB flag is set */
		while i2c1().sr1.read().sb().bits() == 0 {}
		
		/* Send slave address */
		i2c1().dr.write(|w| unsafe { w.bits(214) });
		
		// Wait until ADDR flag is set 
		while i2c1().sr1.read().addr().bits() == 0 {}
		
		//Clear ADDR flag; This bit is cleared by software reading SR1 register followed reading SR2, or by hardware when PE=0.
		i2c1().sr1.read();
		i2c1().sr2.read();
		
		//Wait until TXE flag is set
		while i2c1().sr1.read().tx_e().bits() == 0 {}
		
		//Send Memory Address
		i2c1().dr.write(|w| unsafe { w.bits(memory_address as u32) });
		
		//Wait until TXE flag is set
		while i2c1().sr1.read().tx_e().bits() == 0 {}
		
		/* Generate Restart */
		i2c1().cr1.modify(|_, w| unsafe { w.start().bits(1) });
		
		//Loop until start condition is set
		while i2c1().sr1.read().sb().bits() == 0 {}
		
		/* Send slave address */
		i2c1().dr.write(|w| unsafe { w.bits(215) });
		
		// Wait until ADDR flag is set 
		while i2c1().sr1.read().addr().bits() == 0 {}
	}
	
	pub fn write_memory(&self, data: [u8; 128], len: u8)
	{
	    /* Disable Pos */
	    i2c1().cr1.modify(|_, w| unsafe { w.pos().bits(0) });
	    
	    self.request_memory_write();
	    
	    //Send data
	    let mut x: u8 = 0;
	    while x < len
	    {
			//Wait until TXE flag is set
			while i2c1().sr1.read().tx_e().bits() == 0 {}
	    	i2c1().dr.write(|w| unsafe { w.bits(data[x as usize] as u32) });
	    	x = x + 1;
	    }
	    
	    //Wait until BTF flag is set
	    while i2c1().sr1.read().btf().bits() == 0 {}
	    
	    i2c1().cr1.modify(|_, w| unsafe { w.stop().bits(1) });
	    
	    dwt_driver::wait_cycles(30_000);
	}
	
	pub fn request_memory_write(&self) -> bool
	{
		/* Generate Start */
		i2c1().cr1.modify(|_, w| unsafe { w.start().bits(1) });
		
		//Loop until start condition is set
		while i2c1().sr1.read().sb().bits() == 0 {}
		
		//Send slave address
		i2c1().dr.write(|w| unsafe { w.bits(214) });
		
		// Wait until ADDR flag is set 
		while i2c1().sr1.read().addr().bits() == 0 {}
		
		//Clear ADDR flag; This bit is cleared by software reading SR1 register followed reading SR2, or by hardware when PE=0.
		i2c1().sr1.read();
		i2c1().sr2.read();
		
		//Wait until TXE flag is set
		while i2c1().sr1.read().tx_e().bits() == 0 {}
		
		//Send Memory Address
		i2c1().dr.write(|w| unsafe { w.bits(32) });
		
		true
	}
}