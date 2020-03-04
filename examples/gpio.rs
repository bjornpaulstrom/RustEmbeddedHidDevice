//! Example application
#![no_std]
#![feature(plugin)]
//#![plugin(interpolate_idents)]
#![feature(const_fn)]

//#[macro_use]
extern crate jap;
//#[macro_use]
extern crate hal_core;
use hal_core::*;

extern crate hal;
use hal::spi::*;
use hal::bt::*;
use hal::usart::*;
use hal::i2c::*;

extern crate stm32f401;
use stm32f401::base::*;
use stm32f401::gpioa::*;
use stm32f401::gpiob::*;
use stm32f401::rcc::*;
use stm32f401::usart2::*;
use stm32f401::spi1::*;
use stm32f401::exti::*;
use stm32f401::i2c1::*;

extern crate cortex_m;
//use cortex_m::peripheral::*;
use cortex_m::interrupt;

use jap::{exceptions, interrupts};

pub fn usart2() -> &'static mut Usart2 {
    unsafe { &mut *(USART2_BASE as *mut Usart2) }
}

pub fn i2c1() -> &'static mut I2c1 {
    unsafe { &mut *(I2C1_BASE as *mut I2c1) }
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

pub fn exti() -> &'static mut Exti {
    unsafe { &mut *(EXTI_BASE as *mut Exti) }
}

pub extern "C" fn exti0_handler() 
{
	
	unsafe { usart.send("Received data: \n\r") };
	
    exti().pr.modify(|_, w| unsafe { w.pr0().bits(1) }); // clr interrupt
}

// We need a `main` function, just like every other Rust program
fn main() 
{
	
	let mut hid_service_handle: [u8;2] = [0;2];
	let mut hid_report_char_handle: [u8;2] = [0;2];
	let mut mode: bool = true;

    interrupt::free(|_| 
	{
		//nvic().set_priority(interrupts::Interrupt::Exti0, 0); // highest
        //nvic().enable(interrupts::Interrupt::Exti0);
    });

    dwt_driver::set_cyccntena();

    ////////////////////////////////////////////////////////////////////////////////////////////////
    //                                      Clock stuff!
    ////////////////////////////////////////////////////////////////////////////////////////////////
    //													 PP	  PLLN	    PLLM
    //rcc().pllcfgr.write(|w| w.bits(0b0000 0000 0000 00 01 0 101010000 010000));
    rcc().cfgr.modify(|_, w| unsafe { w.sw0().bits(0) }); //Switch to HSI
    rcc().cfgr.modify(|_, w| unsafe { w.sw1().bits(0) }); //Switch to HSI
    rcc().cfgr.modify(|_, w| unsafe { w.ppre1().bits(4) }); //Configure apb1 prescaler = 2
    rcc().apb1enr.modify(|_, w| unsafe { w.pwren().bits(1) });
    rcc().cr.write(|w| unsafe { w.pllon().bits(0) }); //Enable PLL
    rcc().pllcfgr.write(|w| unsafe { w.bits(0b00000000000000010101010000010000) }); //Configure PLL
    rcc().cr.modify(|_, w| unsafe { w.pllon().bits(1) }); //Enable PLL
    rcc().cfgr.modify(|_, w| unsafe { w.sw0().bits(0) }); //Switch to PLL
    rcc().cfgr.modify(|_, w| unsafe { w.sw1().bits(1) });

    rcc().apb2enr.modify(|_, w| unsafe { w.syscfgen().bits(1) });

    rcc().ahb1enr.modify(|_, w| unsafe { w.gpioaen().bits(1) }); //Enable GPIOA clock
    rcc().ahb1enr.modify(|_, w| unsafe { w.gpioben().bits(1) }); //Enable GPIOB clock

    ////////////////////////////////////////////////////////////////////////////////////////////////
    //										USART2 stuff
    ////////////////////////////////////////////////////////////////////////////////////////////////

    unsafe { usart.initialize() };
    unsafe { usart.send("\n\n\nUSART initialized\n\r") };
    
    ////////////////////////////////////////////////////////////////////////////////////////////////

    spi.initialize();
    i2c.initialize();

	//exti().imr.modify(|_, w| unsafe { w.mr0().bits(1) }); // interrupt mask
	//exti().rtsr.modify(|_, w| unsafe { w.tr0().bits(1) }); // rising edge

	loop
	{
		if mode == true {
			bt.restart_bt();
		
		    bt.configure_bt_address();
		    dwt_driver::wait_cycles(30_000);
		
		    bt.gatt_service_init();
		    dwt_driver::wait_cycles(30_000);
		
		    bt.gap_service_init();
		    dwt_driver::wait_cycles(30_000);
		
		    bt.gap_set_io_capability();
		    dwt_driver::wait_cycles(30_000);
		
		    bt.gap_set_auth_requirments();
		    dwt_driver::wait_cycles(30_000);
		
		    hid_service_handle = bt.start_hid_service();
		    dwt_driver::wait_cycles(30_000);
	
		    let hid_protocol_mode_char_handle = bt.hid_add_protocol_mode_characteristic(hid_service_handle);
		    dwt_driver::wait_cycles(30_000);
		
		    bt.hid_set_protocol_mode_characteristic(hid_service_handle, hid_protocol_mode_char_handle);
		    dwt_driver::wait_cycles(30_000);
		
		    /*let hid_boot_mouse_report_char_handle = */bt.hid_add_boot_mouse_input_report_characteristic(hid_service_handle);
		    dwt_driver::wait_cycles(30_000);
		
		    let hid_information_char_handle = bt.hid_add_information_characteristic(hid_service_handle);
		    dwt_driver::wait_cycles(30_000);
		
		    bt.hid_set_information_characteristic(hid_service_handle, hid_information_char_handle);
		    dwt_driver::wait_cycles(30_000);
		
		    /*let HIDControlPointCharactersiticHandle = */bt.hid_add_control_point_characteristic(hid_service_handle);
		    dwt_driver::wait_cycles(30_000);
		
		    let hid_report_map_char_handle = bt.hid_add_report_map_characteristic(hid_service_handle);
		    dwt_driver::wait_cycles(30_000);
	
		    bt.hid_set_report_map_characteristic(hid_service_handle, hid_report_map_char_handle);
		    dwt_driver::wait_cycles(30_000);
		
		    hid_report_char_handle = bt.hid_add_report_characteristic(hid_service_handle);
		    dwt_driver::wait_cycles(30_000);
		
		    bt.hid_add_report_description(hid_service_handle, hid_report_char_handle);
		    dwt_driver::wait_cycles(30_000);
		
		    let battery_level_service_handle = bt.start_battery_level_service();
		    dwt_driver::wait_cycles(30_000);
		
		    let battery_level_char_handle = bt.add_battery_level_characteristic(battery_level_service_handle);
		    dwt_driver::wait_cycles(30_000);
		
		    bt.add_presentation_format_description(battery_level_service_handle, battery_level_char_handle);
		    dwt_driver::wait_cycles(30_000);
		
		    let device_information_service_handle = bt.start_device_information_service();
		    dwt_driver::wait_cycles(30_000);
		
		    bt.add_pnp_id_characteristic(device_information_service_handle);
		    dwt_driver::wait_cycles(30_000);
	
		    bt.gap_set_discoverable();
		    dwt_driver::wait_cycles(30_000);
		    
		    mode = false;
		    
		} else {
			let mut data : [u8;128] = [0;128];
		
			data[0] = 0x60;
			
			i2c.write_memory(data, 1);
		
			let ident = i2c.read_memory(0x0f);
			if ident != 104
			{
				loop {}
			}
			let xindata = i2c.read_memory(0x28);
			let xindata2 = i2c.read_memory(0x29);
			let yindata = i2c.read_memory(0x2A);
			let yindata2 = i2c.read_memory(0x2B);
			
			let xvalue: i16 = (((xindata2 as i16) << 8 ) + (xindata as i16))/500; // 100;
			let yvalue: i16 = (((yindata2 as i16) << 8 ) + (yindata as i16))/500; // 100;
			
			bt.set_report_characteristic(hid_service_handle, hid_report_char_handle, xvalue, yvalue);
			
			dwt_driver::wait_cycles(84_000_000);
		}
		
		let response = bt.spi_read();

		if response[0] == 4 && response[1] == 5 &&response[2] == 4 /*&& response[3] == 0 && response[4] == 1 && response[5] == 8 && response[6] == 19*/ {
			unsafe { usart.send("Disconnected\n\r") };
			mode = true;
		} else if response[0] == 4 && response[1] == 255 &&response[2] == 3 && response[3] == 1 && response[4] == 0 && response[5] == 1 {
			unsafe { usart.send("Restarted BT\n\r") };
		} else if response[0] == 4 && response[1] == 62 &&response[2] == 19 && response[3] == 1 {
			unsafe { usart.send("Connected complete event!\n\r") };
		} else if response[0] == 4 && response[1] == 62 &&response[2] == 10 && response[3] == 3 {
			unsafe { usart.send("Connection update complete\n\r") };
		} else if response[0] == 4 && response[1] == 8 &&response[2] == 4 {
			unsafe { usart.send("Encryption change\n\r") };
		} else {
			//Nothing we bother with
			//unsafe { usart.send("Received data: unknown\n\r") };
		}
	}
}

// The program must specify how exceptions will be handled
// Here we just use the default handler to handle all the exceptions
#[no_mangle]
pub static _EXCEPTIONS: exceptions::Handlers =
    exceptions::Handlers { ..exceptions::DEFAULT_HANDLERS };

// Likewise with interrupts
#[no_mangle]
pub static _INTERRUPTS: interrupts::Handlers =
    interrupts::Handlers { exti0: exti0_handler, ..interrupts::DEFAULT_HANDLERS };
