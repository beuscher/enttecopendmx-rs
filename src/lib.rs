//! Rust bindings for the [Enttec Open DMX USB] and its FTDI D2XX Chip.
//! 
//! #Usage
//! 
//! Simply add this crate as a dependency in your `Cargo.toml`.
//! 
//! ```toml
//! [dependencies]
//! enttecopendmx = "0.1.0"
//! ```
//! 
//! Further the corresponding driver for the Interface needs to be installed on the system. For further informations 
//! check the information's provided by [Enttec] and the 
//! documentation for the [libftd2xx] crate this one is based on.
//! 
//! The most stripped down way to generate an output signal is to just send the maximum value `255` on Channel one:
//! 
//! ```no_run
//! use enttecopendmx;
//!
//! let mut interface = enttecopendmx::EnttecOpenDMX::new();
//! interface.open();
//! interface.set_channel(1 as usize, 255 as u8);
//! interface.render();
//! ```
//! 
//! 
//! 
//! [Enttec]: https://www.enttec.com/product/lighting-communication-protocols/dmx512/open-dmx-usb
//! [Enttec Open DMX USB]: https://www.enttec.com/product/lighting-communication-protocols/dmx512/open-dmx-usb
//! [libftd2xx]: https://crates.io/crates/libftd2xx
//! 



use libftd2xx::{DeviceStatus, Ftdi, FtdiCommon, StopBits};
use libftd2xx::DeviceInfo;
use libftd2xx::FtStatus;
use std::time::Duration;

//Code structure inspired by C# example code
// -> Class that interfaces with the OPEN DMX USB 
//      - found at Enttecs website: https://www.enttec.com.au/product/lighting-communication-protocols/open-dmx-usb/


// constants

const BUF_SIZE: usize = 513;
const BAUDRATE: u32 = 250000;
const BITS_8: libftd2xx::BitsPerWord = libftd2xx::BitsPerWord::Bits8;
const STOP_BITS_2: libftd2xx::StopBits = StopBits::Bits2;
const PARITY_NONE: libftd2xx::Parity = libftd2xx::Parity::No;
const READ_TIMEOUT: Duration = Duration::from_millis(1000);
const WRITE_TIMEOUT: Duration = Duration::from_millis(1000);


/// This struct represents an Enttec Open DMX Interface. To create a new instance use the `new()` method or construct it "by hand". 
#[allow(dead_code)]
#[derive(Debug)]
pub struct EnttecOpenDMX {
    /// FTDI device 
    ftdi: Ftdi,
    /// buffer which can be written 
    buffer: [u8; BUF_SIZE], 
    /// defaults to "EnttecOpenDMX"
    name: String,
    /// initial device status
    status: DeviceStatus,
    /// initial device info
    info: DeviceInfo
}


impl EnttecOpenDMX {

    /// Creates a new instance representing the Open DMX Interface, it uses the auto discovery provided by the [libftd2xx] crate.
    /// 
    /// To select a specific device check the documentation of the [libftd2xx] crate and then create the struct.
    ///
    /// [libftd2xx]: https://crates.io/crates/libftd2xx
    pub fn new() -> Result<EnttecOpenDMX,FtStatus> {
        let device_name = String::from("EnttecOpenDMX");

        // new device by auto discovery
        let mut ft = Ftdi::new()?;

        // initial device info and status (closed connection)
        let device_info = ft.device_info()?;
        let ftdi_status = ft.status()?;

        // return interface
        Ok(EnttecOpenDMX {
            ftdi: ft,
            buffer: [0; BUF_SIZE],
            name: device_name,
            status: ftdi_status,
            info: device_info,
        })
    }

    /// Opens the connection with the Interface.
    /// 
    pub fn open(&mut self) -> Result<(),FtStatus>{
        self.ftdi.reset()?;
        self.ftdi.set_baud_rate(BAUDRATE)?;
        self.ftdi.set_data_characteristics(BITS_8, STOP_BITS_2, PARITY_NONE)?;
        self.ftdi.set_timeouts(READ_TIMEOUT, WRITE_TIMEOUT)?;
        self.ftdi.set_flow_control_none()?;
        self.ftdi.clear_rts()?;
        self.ftdi.purge_rx()?;
        self.ftdi.purge_tx()?;
        Ok(())
    }


    /// Allows to set the value of a specific DMX channel. For the channel only values lower than 513 are allowed or the code will `panic!`
    pub fn set_channel(&mut self, channel: usize, value: u8) {
        if channel < 513 {
            self.buffer[channel] = value;
        } else {
            panic!("invalid channel: {}", channel);
        }
    }

    /// Allows too set the whole state of the universe at once.
    pub fn set_buffer(&mut self, buf: [u8;BUF_SIZE]) {
        self.buffer = buf;
    }

    /// Allows to set the whole state of the universe at once to 0.
    pub fn blackout(&mut self) {
        self.buffer = [0;BUF_SIZE];
    }


    /// Renders the current buffer 
    pub fn render(&mut self) -> Result<(),FtStatus> {
        self.ftdi.set_break_on()?;
        self.ftdi.set_break_off()?;
        self.ftdi.write_all(&self.buffer).unwrap();
        Ok(())
    }

    /// Closes an open connection.
    pub fn close(&mut self) -> Result<(),FtStatus> {
        self.ftdi.close()?;
        Ok(())
    }

}