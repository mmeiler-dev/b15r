pub trait InRegister {
    fn address(&self) -> u8;
}
pub trait OutRegister {
    fn address(&self) -> u8;
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum DdrPin {
    DDRA = 0x21,
    DDA0 = 0,
    DDA1 = 1,
    DDA2 = 2,
    DDA3 = 3,
    DDA4 = 4,
    DDA5 = 5,
    DDA6 = 6,
    DDA7 = 7,
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum PortPin {
    PORTA    = 0x21,
    PORTA0   = 0,
    PORTA1   = 1,
    PORTA2   = 2,
    PORTA3   = 3,
    PORTA4   = 4,
    PORTA5   = 5,
    PORTA6   = 6,
    PORTA7   = 7,
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum PinPin {
    PINA  = 0x20,
    PINA0 = 0,
    PINA1 = 1,
    PINA2 = 2,
    PINA3 = 3,
    PINA4 = 4,
    PINA5 = 5,
    PINA6 = 6,
    PINA7 = 7,
}

impl OutRegister for DdrPin {
    fn address(&self) -> u8 {
        self.clone() as u8
    }
}

impl OutRegister for PortPin {
    fn address(&self) -> u8 {
        self.clone() as u8
    }
}

impl InRegister for PinPin {
    fn address(&self) -> u8 {
        self.clone() as u8
    }
}
