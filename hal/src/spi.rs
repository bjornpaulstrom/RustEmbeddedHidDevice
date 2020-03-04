extern crate stm32f401;
use self::stm32f401::spi1::*;
use self::stm32f401::gpioa::*;
use self::stm32f401::gpiob::*;
use self::stm32f401::base::*;
use self::stm32f401::rcc::*;

pub struct SPI {
    pub receivebuffer: [u8; 128],
    pub sendbuffer: [u8; 128],
    pub receiveindex: u8,
    pub sendindex: u8,
    pub sendlength: u8,
    pub receivelength: u8,
}

pub fn spi1() -> &'static mut Spi1 {
    unsafe { &mut *(SPI1_BASE as *mut Spi1) }
}

pub fn gpioa() -> &'static mut Gpioa {
    unsafe { &mut *(GPIOA_BASE as *mut Gpioa) }
}

pub fn gpiob() -> &'static mut Gpiob {
    unsafe { &mut *(GPIOB_BASE as *mut Gpiob) }
}

pub fn rcc() -> &'static mut Rcc {
    unsafe { &mut *(RCC_BASE as *mut Rcc) }
}

pub static spi: SPI = SPI {
    receivebuffer: [0; 128],
    sendbuffer: [0; 128],
    receiveindex: 0,
    sendindex: 0,
    sendlength: 0,
    receivelength: 0,
};

impl SPI {
    pub fn initialize(&self) {
        rcc().apb2enr.modify(|_, w| unsafe { w.spi1en().bits(1) });
        spi1().cr1.modify(|_, w| unsafe { w.spe().bits(0) }); //enable SPI

        gpioa().moder.modify(|_, w| w.moder6().bits(2)); //TX,RX
        gpioa().moder.modify(|_, w| w.moder7().bits(2));
        gpioa().otyper.modify(|_, w| w.ot6().bits(0));
        gpioa().otyper.modify(|_, w| w.ot7().bits(0));
        gpioa().ospeedr.modify(|_, w| w.ospeedr6().bits(3));
        gpioa().ospeedr.modify(|_, w| w.ospeedr7().bits(3));
        gpioa().afrl.modify(|_, w| w.afrl6().bits(5));
        gpioa().afrl.modify(|_, w| w.afrl7().bits(5));

        gpioa().moder.modify(|_, w| w.moder1().bits(1)); //CS
        gpioa().otyper.modify(|_, w| w.ot1().bits(0));
        gpioa().ospeedr.modify(|_, w| w.ospeedr1().bits(3));
        gpioa().pupdr.modify(|_, w| w.pupdr1().bits(1));

        gpioa().moder.modify(|_, w| w.moder8().bits(1)); //restart pin
        gpioa().otyper.modify(|_, w| w.ot8().bits(0));
        gpioa().pupdr.modify(|_, w| w.pupdr8().bits(1));

        gpioa().moder.modify(|_, w| w.moder0().bits(0)); //BT chip got data pin
        gpioa().otyper.modify(|_, w| w.ot0().bits(0));
        gpioa().pupdr.modify(|_, w| w.pupdr0().bits(0));

        gpiob().moder.modify(|_, w| unsafe { w.moder3().bits(2) }); //SPI clock
        gpiob().otyper.modify(|_, w| unsafe { w.ot3().bits(0) });
        gpiob().ospeedr.modify(|_, w| unsafe { w.ospeedr3().bits(3) });
        gpiob().afrl.modify(|_, w| unsafe { w.afrl3().bits(5) });
        gpiob().pupdr.modify(|_, w| unsafe { w.pupdr3().bits(2) });

        spi1().cr1.modify(|_, w| unsafe { w.br().bits(0b100) }); // CLK / 32 prescaler
        spi1().cr1.modify(|_, w| unsafe { w.ssi().bits(1) }); // NSS
        spi1().cr1.modify(|_, w| unsafe { w.ssm().bits(1) }); // Software chip select
        spi1().cr1.modify(|_, w| unsafe { w.mstr().bits(1) }); //Master
        spi1().cr1.modify(|_, w| unsafe { w.spe().bits(1) }); //enable SPI
    }

    //Brief:	Send a series of uint8 through spi1
    //data: 	u8 to send
    //len:		length of data
    pub fn spi1send(&self, data: [u8; 128], len: u8) {
        let mut x: u8 = 0;
        while x < len {
            while spi1().sr.read().txe().bits() == 0 {}
            spi1().dr.write(|w| unsafe { w.bits(data[x as usize] as u32) });
            while spi1().sr.read().rxne().bits() == 0 {}
            let _ = spi1().dr.read().dr().bits() as u8;
            x = x + 1;
        }
    }

    pub fn spi1receive(&self, length: u8) -> [u8; 128] 
    {
    	let mut response: [u8; 128] = [0; 128];
        let mut x: u8 = 0;

        while x < length 
        {
            while spi1().sr.read().txe().bits() == 0 {}
            spi1().dr.write(|w| unsafe { w.bits(0 as u32) });
            while spi1().sr.read().rxne().bits() == 0 {}
            response[x as usize] = spi1().dr.read().dr().bits() as u8;
            x = x + 1;
        }
        response
    }

    //Brief:	Send a series of uint8 through spi1 returning answer
    //data: 	u8 to send
    //len:		length of data
    //return:	return bytes from spi
    pub fn spi1sendreceive(&self, data: [u8; 128], response: &mut [u8; 128], len: u8)
    {
        let mut x: u8 = 0;
        while x < len 
        {
            while spi1().sr.read().txe().bits() == 0 {}
            spi1().dr.write(|w| unsafe { w.bits(data[x as usize] as u32) });
            while spi1().sr.read().rxne().bits() == 0 {}
            response[x as usize] = spi1().dr.read().dr().bits() as u8;
            x = x + 1;
        }
    }
}
