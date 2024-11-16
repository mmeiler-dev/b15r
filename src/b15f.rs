use std::process::Command;
use serialport::SerialPort;
use std::time::Duration;
use std::thread;
use rand::Rng;
use crate::{
    constants::*,
    ports::*,
    usart::*,
    registers::*,
    commands::B15FCommand,
    command_buffer,
    error,
    info
};


pub struct B15F {
    port: Box<dyn SerialPort>
}

impl B15F {
    pub fn get_instance() -> Self {
        let port = B15F::init().expect("There was no port found!");
        let mut drv = Self {
            port 
        };

        let mut tries = 3;
        while tries > 0 {
            drv.discard();

            match drv.test_connection() {
                true => break,
                _ => {}
            }

            tries -= 1;
        }

        info!("Connection to B15F established!");
        //info!("{}", drv.board_info())
        drv
    }

    fn init() -> Option<Box<dyn SerialPort>> {
        let devices = B15F::get_devices();

        let device = match devices.first() {
            Some(item) => item,
            None => {
                error!("Failed to find adapter for B15F!"; "Is the USB connected properly?");
            } 
        };

        let port = serialport::new(device, BAUD)
            .timeout(Duration::from_millis(1000))
            .open().expect("No connection!");

        Some(port)
    }

    fn get_devices() -> Vec<String> {
        let mut serial_port = "ls /dev/ttyUSB*";
        if cfg!(any(target_arch = "arm", target_arch = "aarch64")) {
            serial_port = "ls /dev/ttyAMA*";
        }

        let output = Command::new("bash")
            .args(["-c", serial_port])
            .output()
            .expect("Failed to get serial interface!");

        String::from_utf8(output.stdout)
            .expect("Failed to convert to string!")
            .split_ascii_whitespace()
            .map(|item| item.into())
            .collect()
    }

    fn discard(&mut self) {
        self.port.clear(serialport::ClearBuffer::Output).expect("Couldn't clear output buffer!");

        for _ in 0..16 {
            self.port.write(command_buffer![B15FCommand::Discard]).expect("Couldn't write to port!");
            thread::sleep(Duration::from_millis(4));
        }

        self.port.clear(serialport::ClearBuffer::Input).expect("Couldn't clear input buffer!");
    }

    fn test_connection(&mut self) -> bool {
        let dummy: u8 = rand::thread_rng().gen_range(0x00..=0xFF);

        self.port.write(command_buffer![
            B15FCommand::Test, 
            dummy
        ]).expect("Failed to write to port!");
        
        let mut buffer: [u8; 2] = [0;2];
        self.port.read(&mut buffer).expect("Couldn't write to buffer!");
        
        if buffer[0] != OK || buffer[1] != dummy {
            return false;
        }

        true
    }

    /*fn board_info(&self) -> Vec<String> {
        self.port.write(command_buffer![B15FCommand::Info]).expect("Failed to write to port!");

        let size: u8 = read_sized::<1>(&mut self.port).expect("Couldn't read from port!");

        while n >= 0 {
            let len: u8 = read_sized::<1>(&mut self.port).expect("Coudln't read from port!");

        }
    }*/

    ////////////////////////////////////////////////////
    //              CONTROL FUNCTIONS                 //
    ////////////////////////////////////////////////////

    pub fn digital_write(&mut self, port: B15FPort, value: u8) {
        let reversed = value.reverse_bits();
        let command = match port {
            B15FPort::Port0 => B15FCommand::DigitalWrite0,
            B15FPort::Port1 => B15FCommand::DigitalWrite1,
        };

        self.port.write(command_buffer![
            command, 
            reversed
        ]).expect("Couldn't write to port!");
        
        let response = read_sized::<1>(&mut self.port).expect("Couldn't read response!");
        if response[0] != OK {
            error!("Digital write to port {:?} failed!", port);
        }
    }

    pub fn digital_read(&mut self, port: B15FPort) -> u8 {
        self.port.clear(serialport::ClearBuffer::Input).expect("Couldn't clear input buffer!");
 
        let command = match port {
            B15FPort::Port0 => B15FCommand::DigitalRead0,
            B15FPort::Port1 => B15FCommand::DigitalRead1,
        };

        self.port.write(command_buffer![command]).expect("Couldn't write to port!");

        let response = read_sized::<1>(&mut self.port).expect("Couldn't read response!");
        response[0].reverse_bits()
    }

    pub fn read_dip_switch(&mut self) -> u8 {
        self.port.clear(serialport::ClearBuffer::Input).expect("Couldn't clear input buffer!");
        self.port.write(command_buffer![B15FCommand::ReadDipSwitch]).expect("Failed to write to port!");

        let response = read_sized::<1>(&mut self.port).expect("Couldn't read response!");
        response[0].reverse_bits()
    }

    pub fn analog_write(&mut self, port: B15FPort, value: u16) {
        let command = match port {
            B15FPort::Port0 => B15FCommand::AnalogWrite0,
            B15FPort::Port1 => B15FCommand::AnalogWrite1,
        };

        self.port.write(command_buffer![
            command, 
            value & 0xFF, 
            value >> (8 as u8)
        ]).expect("Failed to write to port!");

        let response = read_sized::<1>(&mut self.port).expect("Couldn't get a response!");
        if response[0] != OK {
            error!("Analog write to port {:?} failed!", port);
        }
    }

    pub fn analog_read(&mut self, channel: u8) -> u16 {
        assert!(channel <= 7, "Channel number should be in the range of 0..7. Given channel: {}", channel);

        self.port.clear(serialport::ClearBuffer::Input).expect("Couldn't clear input buffer!");
        self.port.write(command_buffer![
            B15FCommand::AnalogRead, 
            channel
        ]).expect("Failed to write to port!");

        let mut buffer: [u8;2] = [0;2];
        self.port.read(&mut buffer).expect("Couldn't read buffer!");

        let response = u16::from_le_bytes(buffer);

        if response > 1023 {
            error!("Bad ADC data!");
        }
        
        response
    }

    fn set_mem8(&mut self, address: u8, value: u8) {
        self.port.clear(serialport::ClearBuffer::Input).expect("Couldn't clear input buffer!");
        self.port.write(command_buffer![
            B15FCommand::SetMem8, 
            ((address) & 0xFF) as u8, 
            ((address as usize) >> 8) as u8, 
            value
        ]).expect("Failed to write to port!");

        let response = read_sized::<1>(&mut self.port).expect("Couldn't get a response!");
        if response[0] != OK {
            error!("Bad value!");
        }
    }

    pub fn set_register<R: InRegister>(&mut self, register: R, value: u8) {
        let address = register.address();
        self.set_mem8(address, value);
    }
}
