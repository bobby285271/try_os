use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

/// use lazy_static to ensure the init method is called exactly once.
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}
