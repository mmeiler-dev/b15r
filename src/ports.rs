#[derive(Debug)]
pub enum B15FPort {
    Port0,
    Port1
}

/// For ease of use so that you don't have to import the ports all the time (which would be more efficient) 
impl From<u8> for B15FPort {
    fn from(port_num: u8) -> Self {
        assert!(port_num == 0 || port_num == 1, );
        match port_num {
            0 => B15FPort::Port0,
            1 => B15FPort::Port1,
            _ => {
                panic!("You have given a false port! Given port: {}", port_num);
            }
        }
    }
}
