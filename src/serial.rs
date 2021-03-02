// To see the test output on the console, we need to send the data from our kernel to the host system somehow.
// A simple way to send the data is to use the serial port.

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

// use lazy_static to ensure the init method is called exactly once.

/// use the uart_16550 crate to initialize the UART and send data over the serial port.
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}
