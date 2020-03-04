extern crate hal_core;
use self::hal_core::*;

extern crate stm32f401;
use self::stm32f401::base::*;
use self::stm32f401::gpioa::*;
use self::stm32f401::exti::*;

use spi::*;
use usart::*;

pub static bt: BT = BT
{
};

pub struct BT {}

pub struct BluetoothWriteStatus 
{
    pub status: bool,
    pub bytes: u8,
}

pub struct BluetoothReadStatus 
{
    pub status: bool,
    pub bytes: u8,
}

pub struct HCICommand 
{
    pub ogf: u8,
    pub ocf: u16,
    pub length: u8,
    pub command: [u8; 128],
}

pub struct HCIResponse 
{
    //eventcode: u8,
    //length: u8,
    pub params: [u8; 128],
}

pub fn gpioa() -> &'static mut Gpioa 
{
    unsafe { &mut *(GPIOA_BASE as *mut Gpioa) }
}

pub fn exti() -> &'static mut Exti {
    unsafe { &mut *(EXTI_BASE as *mut Exti) }
}


impl BT
{
	pub fn restart_bt(&self)
	{
		unsafe { usart.send("Restarting Bluetooth...\n\r") };
		
	    gpioa().bsrr.write(|w| w.br8().bits(1)); // turn off bt
	    dwt_driver::wait_cycles(84_000 * 5); //5 msec
	    gpioa().bsrr.write(|w| w.bs8().bits(1)); // turn on bt
	    dwt_driver::wait_cycles(84_000 * 5); // 5msec
	    
	    let data = self.spi_read();
	}
	
	pub fn send_hci_command(&self, hci: HCICommand) -> [u8; 128]
	{
		
        let mut hc: [u8; 128] = [0; 128];
        let mut response: [u8; 128] = [0; 128];
        let mut bws = BluetoothWriteStatus { status: false, bytes: 0 };

        hc[0] = 0x01;
        hc[1] = hci.ocf as u8;
        hc[2] = hci.ogf << 2;
        hc[3] = hci.length;

        if hci.ocf > 255 {
            if hci.ocf & 0b1000000000 != 0 {
                hc[2] = hc[2] + 2;
            }
            if hci.ocf & 0b0100000000 != 0 {
                hc[2] = hc[2] + 1;
            }
        }

        let mut i = 0;
        while i < hci.length 
        {
            hc[4 + i as usize] = hci.command[i as usize];
            i = i + 1;
        }
	
		//exti().imr.modify(|_, w| unsafe { w.mr0().bits(0) }); // interrupt mask
		loop 
		{
			self.spi_select_bt();
			self.bluetooth_write_status(&mut bws);
			if bws.status == true
			{
				if bws.bytes > hci.length
				{
					spi.spi1sendreceive(hc, &mut response, hci.length+4);
					self.spi_deselect_bt();
					response = self.spi_read();
					//exti().imr.modify(|_, w| unsafe { w.mr0().bits(1) }); // interrupt mask
					return response;
				}
						
			}
			self.spi_deselect_bt();
		}	
	}
	
    pub fn spi_read(&self) -> [u8; 128]
    {
        let mut response: [u8; 128] = [0; 128];
        let mut brs = BluetoothReadStatus { status: false, bytes: 0 };
		dwt_driver::wait_cycles(10 * 84_000); //1msec
        while gpioa().idr.read().idr0().bits() == 1 
        {
            self.spi_select_bt();
            dwt_driver::wait_cycles(500 * 84); //500 usec
            self.bluetooth_read_status(&mut brs);
            if brs.status == true && brs.bytes > 0 
            {
                response = spi.spi1receive(brs.bytes);
                self.spi_deselect_bt();
                //dwt_driver::wait_cycles(50 * 84_000); //50msec
                break;
            }
            self.spi_deselect_bt();
            dwt_driver::wait_cycles(10 * 84_000); //1msec
        }
        response
    }

    pub fn configure_bt_address(&self) 
    {
    	unsafe { usart.send("Setting config data, bdaddress: ") };
    	
        let mut hcicommand: HCICommand = HCICommand { ogf: 0x3f, ocf: 0x000C, length: 8, command: [0; 128] };
        hcicommand.command[0] = 0x00;
        hcicommand.command[1] = 0x06;
        hcicommand.command[2] = 0xaa;
        hcicommand.command[3] = 0x00;
        hcicommand.command[4] = 0x00;
        hcicommand.command[5] = 0xe1;
        hcicommand.command[6] = 0x80;
        hcicommand.command[7] = 0x02;

        //self.bt_send_hci_command(hc);
        let hciresponse = self.send_hci_command(hcicommand);
        
        //The controller will generate a command complete event. BT Core specification Vol 2, Part E, 7.7.14 - Command Complete Event
        if hciresponse[6] == 0x00
        {
        	unsafe { usart.send("Success\n\r") };
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn gatt_service_init(&self) 
    {
    	unsafe { usart.send("Initializing GATT service: ") };
    	
        let hcicommand: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000001, length: 0, command: [0; 128] };

        //self.bt_send_hci_command(hc);
        let hciresponse = self.send_hci_command(hcicommand);
        
        //The controller will generate a command complete event. BT Core specification Vol 2, Part E, 7.7.14 - Command Complete Event
        if hciresponse[6] == 0x00
        {
        	unsafe { usart.send("Success\n\r") };
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn gap_service_init(&self) 
    {
    	unsafe { usart.send("Initializing GAP service: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0x008A, length: 3, command: [0; 128] };
        hc.command[0] = 0x01;
        hc.command[1] = 0x00;
        hc.command[2] = 0x07;

        let hciresponse = self.send_hci_command(hc);
        
        //The controller will generate a command complete event. BT Core specification Vol 2, Part E, 7.7.14 - Command Complete Event
        if hciresponse[6] == 0x00
        {
        	unsafe { usart.send("Success\n\r") };
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn gap_set_io_capability(&self) 
    {
    	unsafe { usart.send("Setting GAP IO capability: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0x85, length: 1, command: [0; 128] };
        hc.command[0] = 0x03;

        let hciresponse = self.send_hci_command(hc);
        
        //The controller will generate a command complete event. BT Core specification Vol 2, Part E, 7.7.14 - Command Complete Event
        if hciresponse[6] == 0x00
        {
        	unsafe { usart.send("Success\n\r") };
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn gap_set_auth_requirments(&self) 
    {
    	unsafe { usart.send("Setting GAP auth requirments: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0x86, length: 26, command: [0; 128] };
        hc.command[0] = 0x00;
        hc.command[1] = 0x00;
        hc.command[2] = 0x00;
        hc.command[3] = 0x00;
        hc.command[4] = 0x00;
        hc.command[5] = 0x00;
        hc.command[6] = 0x00;
        hc.command[7] = 0x00;
        hc.command[8] = 0x00;
        hc.command[9] = 0x00;
        hc.command[10] = 0x00;
        hc.command[11] = 0x00;
        hc.command[12] = 0x00;
        hc.command[13] = 0x00;
        hc.command[14] = 0x00;
        hc.command[15] = 0x00;
        hc.command[16] = 0x00;
        hc.command[17] = 0x00;
        hc.command[18] = 0x07;
        hc.command[19] = 0x10;
        hc.command[20] = 0x00;
        hc.command[21] = 0x01;
        hc.command[22] = 0x00;
        hc.command[23] = 0x00;
        hc.command[24] = 0x00;
        hc.command[25] = 0x00;

        let hciresponse = self.send_hci_command(hc);
        
        //The controller will generate a command complete event. BT Core specification Vol 2, Part E, 7.7.14 - Command Complete Event
        if hciresponse[6] == 0x00
        {
        	unsafe { usart.send("Success\n\r") };
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn start_hid_service(&self) -> [u8;2]
    {
    	unsafe { usart.send("Starting HID service: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000010, length: 5, command: [0; 128] };
        hc.command[0] = 0x01;
        hc.command[1] = 0x12;
        hc.command[2] = 0x18;
        hc.command[3] = 0x01;
        hc.command[4] = 0x10;
	
        let hciresponse = self.send_hci_command(hc);
        
        if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        	let mut ret: [u8;2] = [0;2];
			ret[0] = hciresponse[7];
			ret[1] = hciresponse[8];
			return ret;
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn hid_add_protocol_mode_characteristic(&self, HIDServiceHandle: [u8;2]) -> [u8;2] 
    {
    	unsafe { usart.send("Adding HID protocol mode characteristic: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000100, length: 12, command: [0; 128] };
        hc.command[0] = HIDServiceHandle[0];
        hc.command[1] = HIDServiceHandle[1];
        hc.command[2] = 0x01;
        hc.command[3] = 0x4e;
        hc.command[4] = 0x2a;
        hc.command[5] = 0x01;
        hc.command[6] = 0x00;
        hc.command[7] = 0x06;
        hc.command[8] = 0x00;
        hc.command[9] = 0x00;
        hc.command[10] = 0x10;
        hc.command[11] = 0x00;

        let hciresponse = self.send_hci_command(hc);
        
        if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        	let mut ret: [u8;2] = [0;2];
			ret[0] = hciresponse[7];
			ret[1] = hciresponse[8];
			return ret;
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn hid_set_protocol_mode_characteristic(&self, HIDServiceHandle: [u8;2], HIDProtocolModeCharactersiticHandle: [u8;2]) 
    {
    	
    	unsafe { usart.send("Setting HID protocol mode characteristic: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000110, length: 7, command: [0; 128] };
                
        hc.command[0] = HIDServiceHandle[0];
        hc.command[1] = HIDServiceHandle[1];
        hc.command[2] = HIDProtocolModeCharactersiticHandle[0];
        hc.command[3] = HIDProtocolModeCharactersiticHandle[1];
        hc.command[4] = 0x00;
        hc.command[5] = 0x01;
        hc.command[6] = 0x01;

        let hciresponse = self.send_hci_command(hc);
        
        if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn hid_add_boot_mouse_input_report_characteristic(&self, HIDServiceHandle: [u8;2]) -> [u8;2]
    {
    	unsafe { usart.send("Adding boot mouse input report characteristic: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000100, length: 12, command: [0; 128] };
        hc.command[0] = HIDServiceHandle[0];
        hc.command[1] = HIDServiceHandle[1];
        hc.command[2] = 0x01;
        hc.command[3] = 0x33;
        hc.command[4] = 0x2a;
        hc.command[5] = 0x03;
        hc.command[6] = 0x00;
        hc.command[7] = 0x1a;
        hc.command[8] = 0x00;
        hc.command[9] = 0x01;
        hc.command[10] = 0x10;
        hc.command[11] = 0x00;

        let hciresponse = self.send_hci_command(hc);
        
        if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        	let mut ret: [u8;2] = [0;2];
			ret[0] = hciresponse[7];
			ret[1] = hciresponse[8];
			return ret;
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn hid_add_information_characteristic(&self, HIDServiceHandle: [u8;2]) -> [u8;2]
    {
    	unsafe { usart.send("Adding HID information characteristic: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000100, length: 12, command: [0; 128] };
        hc.command[0] = HIDServiceHandle[0];
        hc.command[1] = HIDServiceHandle[1];
        hc.command[2] = 0x01;
        hc.command[3] = 0x4a;
        hc.command[4] = 0x2a;
        hc.command[5] = 0x04;
        hc.command[6] = 0x00;
        hc.command[7] = 0x02;
        hc.command[8] = 0x00;
        hc.command[9] = 0x01;
        hc.command[10] = 0x10;
        hc.command[11] = 0x00;

        let hciresponse = self.send_hci_command(hc);
        
        if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        	let mut ret: [u8;2] = [0;2];
			ret[0] = hciresponse[7];
			ret[1] = hciresponse[8];
			return ret;
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn hid_set_information_characteristic(&self, HIDServiceHandle: [u8;2], HIDInformationCharactersiticHandle: [u8;2]) 
    {
    	unsafe { usart.send("Setting HID information characteristic: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000110, length: 10, command: [0; 128] };
        hc.command[0] = HIDServiceHandle[0];
        hc.command[1] = HIDServiceHandle[1];
        hc.command[2] = HIDInformationCharactersiticHandle[0];
        hc.command[3] = HIDInformationCharactersiticHandle[1];
        hc.command[4] = 0x00;
        hc.command[5] = 0x04;
        hc.command[6] = 0x01;
        hc.command[7] = 0x12;
        hc.command[8] = 0x00;
        hc.command[9] = 0x00;

        let hciresponse = self.send_hci_command(hc);
        
        if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn hid_add_control_point_characteristic(&self, HIDServiceHandle: [u8;2]) -> [u8;2]
    {
    	unsafe { usart.send("Adding HID control point characteristic: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000100, length: 12, command: [0; 128] };
        hc.command[0] = HIDServiceHandle[0];
        hc.command[1] = HIDServiceHandle[1];
        hc.command[2] = 0x01;
        hc.command[3] = 0x4c;
        hc.command[4] = 0x2a;
        hc.command[5] = 0x01;
        hc.command[6] = 0x00;
        hc.command[7] = 0x04;
        hc.command[8] = 0x00;
        hc.command[9] = 0x01;
        hc.command[10] = 0x10;
        hc.command[11] = 0x00;

        let hciresponse = self.send_hci_command(hc);
        
        if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        	let mut ret: [u8;2] = [0;2];
			ret[0] = hciresponse[7];
			ret[1] = hciresponse[8];
			return ret;
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn hid_add_report_map_characteristic(&self, HIDServiceHandle: [u8;2]) -> [u8;2]  
    {
    	unsafe { usart.send("Adding HID report map characteristic: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000100, length: 12, command: [0; 128] };
        hc.command[0] = HIDServiceHandle[0];
        hc.command[1] = HIDServiceHandle[1];
        hc.command[2] = 0x01;
        hc.command[3] = 0x4b;
        hc.command[4] = 0x2a;
        hc.command[5] = 0x34;
        hc.command[6] = 0x00;
        hc.command[7] = 0x02;
        hc.command[8] = 0x00;
        hc.command[9] = 0x00;
        hc.command[10] = 0x10;
        hc.command[11] = 0x00;

        let hciresponse = self.send_hci_command(hc);
        
        if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        	let mut ret: [u8;2] = [0;2];
			ret[0] = hciresponse[7];
			ret[1] = hciresponse[8];
			return ret;
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn hid_set_report_map_characteristic(&self, HIDServiceHandle: [u8;2], HIDReportMapCharactersiticHandle: [u8;2]) 
    {
    	unsafe { usart.send("Setting report map characteristic: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000110, length: 58, command: [0; 128] };
        hc.command[0] = HIDServiceHandle[0];
        hc.command[1] = HIDServiceHandle[1];
        hc.command[2] = HIDReportMapCharactersiticHandle[0];
        hc.command[3] = HIDReportMapCharactersiticHandle[1];
		hc.command[4] = 0x00;
		hc.command[5] = 0x34;
		hc.command[6] = 0x05; //
		hc.command[7] = 0x01; //
		hc.command[8] = 0x09; //
		hc.command[9] = 0x02; //
		hc.command[10] = 0xA1;//
		hc.command[11] = 0x01;//
		hc.command[12] = 0x09;//
		hc.command[13] = 0x01;//
		hc.command[14] = 0xA1;//
		hc.command[15] = 0x00;//
		hc.command[16] = 0x85;//
		hc.command[17] = 0x01;//
		hc.command[18] = 0x05;//
		hc.command[19] = 0x09;//
		hc.command[20] = 0x19;//
		hc.command[21] = 0x01;//
		hc.command[22] = 0x29;//
		hc.command[23] = 0x03;//
		hc.command[24] = 0x15;//
		hc.command[25] = 0x00;//
		hc.command[26] = 0x25;//
		hc.command[27] = 0x01;//
		hc.command[28] = 0x95;//
		hc.command[29] = 0x03;//
		hc.command[30] = 0x75;//
		hc.command[31] = 0x01;//
		hc.command[32] = 0x81;//
		hc.command[33] = 0x02;//
		hc.command[34] = 0x95;//
		hc.command[35] = 0x01;//
		hc.command[36] = 0x75;//
		hc.command[37] = 0x05;//
		hc.command[38] = 0x81;//
		hc.command[39] = 0x03;//
		hc.command[40] = 0x05;//
		hc.command[41] = 0x01;//
		hc.command[42] = 0x09;//
		hc.command[43] = 0x30;//
		hc.command[44] = 0x09;//
		hc.command[45] = 0x31;//
		hc.command[46] = 0x15;//
		hc.command[47] = 0x81;//
		hc.command[48] = 0x25;//
		hc.command[49] = 0x7F;//
		hc.command[50] = 0x75;//
		hc.command[51] = 0x08;//
		hc.command[52] = 0x95;//
		hc.command[53] = 0x02;//
		hc.command[54] = 0x81;//
		hc.command[55] = 0x06;//
		hc.command[56] = 0xC0;//
		hc.command[57] = 0xC0;//

        let hciresponse = self.send_hci_command(hc);
        
        if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn hid_add_report_characteristic(&self, HIDServiceHandle: [u8;2]) -> [u8;2]    
    {
    	unsafe { usart.send("Adding HID report characteristic: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000100, length: 12, command: [0; 128] };
        hc.command[0] = HIDServiceHandle[0];
        hc.command[1] = HIDServiceHandle[1];
        hc.command[2] = 0x01;
        hc.command[3] = 0x4d;
        hc.command[4] = 0x2a;
        hc.command[5] = 0x03;
        hc.command[6] = 0x00;
        hc.command[7] = 0x12;
        hc.command[8] = 0x00;
        hc.command[9] = 0x01;
        hc.command[10] = 0x10;
        hc.command[11] = 0x00;

        let hciresponse = self.send_hci_command(hc);
        
        if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        	let mut ret: [u8;2] = [0;2];
			ret[0] = hciresponse[7];
			ret[1] = hciresponse[8];
			return ret;
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn hid_add_report_description(&self, HIDServiceHandle: [u8;2], HIDReportCharactersiticHandle: [u8;2]) -> [u8;2]
	{
		unsafe { usart.send("Adding HID report description: ") };
		
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000101, length: 16, command: [0; 128] };
        hc.command[0] = HIDServiceHandle[0];
        hc.command[1] = HIDServiceHandle[1];
        hc.command[2] = HIDReportCharactersiticHandle[0];
        hc.command[3] = HIDReportCharactersiticHandle[1];
        hc.command[4] = 0x01;
        hc.command[5] = 0x08;
        hc.command[6] = 0x29;
        hc.command[7] = 0x02;
        hc.command[8] = 0x02;
        hc.command[9] = 0x01;
        hc.command[10] = 0x01;
        hc.command[11] = 0x00;
        hc.command[12] = 0x01;
        hc.command[13] = 0x01;
        hc.command[14] = 0x10;
        hc.command[15] = 0x00;

        let hciresponse = self.send_hci_command(hc);
        
	    if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        	let mut ret: [u8;2] = [0;2];
			ret[0] = hciresponse[7];
			ret[1] = hciresponse[8];
			return ret;
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn start_battery_level_service(&self) -> [u8;2]
    {
    	unsafe { usart.send("Starting battery level service: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000010, length: 5, command: [0; 128] };
        hc.command[0] = 0x01;
        hc.command[1] = 0x0F;
        hc.command[2] = 0x18;
        hc.command[3] = 0x01;
        hc.command[4] = 0x04;

        let hciresponse = self.send_hci_command(hc);
        
	    if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        	let mut ret: [u8;2] = [0;2];
			ret[0] = hciresponse[7];
			ret[1] = hciresponse[8];
			return ret;
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn add_battery_level_characteristic(&self, BatteryLevelServiceHandle: [u8;2]) -> [u8;2]
    {
    	unsafe { usart.send("Adding battery level characteristic: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000100, length: 12, command: [0; 128] };
        hc.command[0] = BatteryLevelServiceHandle[0];
        hc.command[1] = BatteryLevelServiceHandle[1];
        hc.command[2] = 0x01;
        hc.command[3] = 0x19;
        hc.command[4] = 0x2a;
        hc.command[5] = 0x01;
        hc.command[6] = 0x00;
        hc.command[7] = 0x02;
        hc.command[8] = 0x00;
        hc.command[9] = 0x01;
        hc.command[10] = 0x10;
        hc.command[11] = 0x00;

        let hciresponse = self.send_hci_command(hc);
        
	    if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        	let mut ret: [u8;2] = [0;2];
			ret[0] = hciresponse[7];
			ret[1] = hciresponse[8];
			return ret;
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn add_presentation_format_description(&self, BatteryLevelServiceHandle: [u8;2], BatteryLevelCharactersiticHandle: [u8;2]) -> [u8;2] 
    {
    	unsafe { usart.send("Adding battery level format description: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000101, length: 21, command: [0; 128] };
        hc.command[0] = BatteryLevelServiceHandle[0];
        hc.command[1] = BatteryLevelServiceHandle[1];
        hc.command[2] = BatteryLevelCharactersiticHandle[0];
        hc.command[3] = BatteryLevelCharactersiticHandle[1];
        hc.command[4] = 0x01;
        hc.command[5] = 0x04;
        hc.command[6] = 0x29;
        hc.command[7] = 0x07;
        hc.command[8] = 0x07;
        hc.command[9] = 0x04;
        hc.command[10] = 0x00;
        hc.command[11] = 0xad;
        hc.command[12] = 0x27;
        hc.command[13] = 0x01;
        hc.command[14] = 0x00;
        hc.command[15] = 0x01;
        hc.command[16] = 0x00;
        hc.command[17] = 0x01;
        hc.command[18] = 0x01;
        hc.command[19] = 0x10;
        hc.command[20] = 0x00;

        let hciresponse = self.send_hci_command(hc);
        
	    if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        	let mut ret: [u8;2] = [0;2];
			ret[0] = hciresponse[7];
			ret[1] = hciresponse[8];
			return ret;
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }


    pub fn start_device_information_service(&self) -> [u8;2]
    {
    	unsafe { usart.send("Starting device information service: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000010, length: 5, command: [0; 128] };
        hc.command[0] = 0x01;
        hc.command[1] = 0x0A;
        hc.command[2] = 0x18;
        hc.command[3] = 0x01;
        hc.command[4] = 0x15;

        let hciresponse = self.send_hci_command(hc);
        
	    if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        	let mut ret: [u8;2] = [0;2];
			ret[0] = hciresponse[7];
			ret[1] = hciresponse[8];
			return ret;
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn add_pnp_id_characteristic(&self, DeviceInformationServiceHandle: [u8;2]) 
    {
    	unsafe { usart.send("Adding device info PNP ID characteristic: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000100, length: 12, command: [0; 128] };
        hc.command[0] = DeviceInformationServiceHandle[0];
        hc.command[1] = DeviceInformationServiceHandle[1];
        hc.command[2] = 0x01;
        hc.command[3] = 0x50;
        hc.command[4] = 0x2a;
        hc.command[5] = 0x07;
        hc.command[6] = 0x00;
        hc.command[7] = 0x02;
        hc.command[8] = 0x00;
        hc.command[9] = 0x01;
        hc.command[10] = 0x10;
        hc.command[11] = 0x00;

        let hciresponse = self.send_hci_command(hc);
        
	    if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }

    pub fn gap_set_discoverable(&self) 
    {
    	unsafe { usart.send("GAP set discoverable: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0x83, length: 16, command: [0; 128] };
        hc.command[0] = 0x00;
        hc.command[1] = 0x00;
        hc.command[2] = 0x00;
        hc.command[3] = 0x00;
        hc.command[4] = 0x00;
        hc.command[5] = 0x00;
        hc.command[6] = 0x00;
        hc.command[7] = 0x03;
        hc.command[8] = 0x09;
        hc.command[9] = 0x42;
        hc.command[10] = 0x50;
        hc.command[11] = 0x00;
        hc.command[12] = 0x06;
        hc.command[13] = 0x00;
        hc.command[14] = 0x00;
        hc.command[15] = 0x0c;

        let hciresponse = self.send_hci_command(hc);
        
	    if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	loop {}
        }
    }
    
    pub fn set_report_characteristic(&self, HIDServiceHandle: [u8;2], HIDReportCharactersiticHandle: [u8;2], x: i16, y: i16)
    {
    	unsafe { usart.send("Sending data: ") };
    	
        let mut hc: HCICommand = HCICommand { ogf: 0x3f, ocf: 0b0100000110, length: 9, command: [0; 128] };
        hc.command[0] = HIDServiceHandle[0];
        hc.command[1] = HIDServiceHandle[1];
        hc.command[2] = HIDReportCharactersiticHandle[0];
        hc.command[3] = HIDReportCharactersiticHandle[1];
        hc.command[4] = 0x00;
        hc.command[5] = 0x03;
        hc.command[6] = 0x00;
        hc.command[7] = -x as u8;
        hc.command[8] = y as u8;

        let hciresponse = self.send_hci_command(hc);
        
	    if hciresponse[6] == 0
        {
        	unsafe { usart.send("Success\n\r") };
        }
        else
        {
        	unsafe { usart.send("Failure\n\r") };
        	//loop {}
        }
    }

    //Brief: Selects the BT device for SPI transfer
    pub fn spi_select_bt(&self) {
        gpioa().bsrr.write(|w| w.br1().bits(1)); // clear ODR1 (BT select)
        //gpioa().odr.write(unsafe { |w| w.odr1().bits(0) } );
    }

    //Brief: Deselects the BT device for SPI transfer
    pub fn spi_deselect_bt(&self) {
        gpioa().bsrr.write(|w| w.bs1().bits(1));
        //gpioa().odr.write(unsafe { |w| w.odr1().bits(1) } );
    }

    //Brief:	Writes to BT device expecting response on how many bytes can be written as a command
    //return:	struct containing info on bt device, if it is ready for receiving command and how many bytes can be written
    pub fn bluetooth_write_status(&self, bws: &mut BluetoothWriteStatus) 
    {
        let mut response: [u8; 128] = [0; 128];
        let mut data = [0; 128];
        data[0] = 0x0a;

        unsafe { spi.spi1sendreceive(data, &mut response, 5) };
        if response[0] == 0x02 
        {
            bws.status = true;
            bws.bytes = response[1];
        }
    }

    pub fn bluetooth_read_status(&self, brs: &mut BluetoothReadStatus)
    {  
        let mut data: [u8; 128] = [0; 128];
        let mut response: [u8; 128] = [0; 128];
        data[0] = 0x0b;

        spi.spi1sendreceive(data, &mut response, 5);

        if response[0] == 0x02 
        {
            brs.status = true;
            brs.bytes = response[3];
        }
    }
}
