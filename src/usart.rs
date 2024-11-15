use serialport::SerialPort;

pub fn read_sized<const N: usize> (usart: &mut Box<dyn SerialPort>) -> Option<[u8; N]> {
        let mut buf: [u8; N] = [0; N];

        usart.read(&mut buf).expect("Couldn't read buffer!");
        Some(buf)
}
