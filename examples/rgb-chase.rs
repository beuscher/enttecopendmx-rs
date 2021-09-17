#![deny(unsafe_code)]
use enttecopendmx;
use std::time::Duration;
use std::thread;

//
// Simple loop chasing through the first 3 channels of the universe
//

fn main() {
    // new interface
    let mut interface = enttecopendmx::EnttecOpenDMX::new().unwrap();

    interface.open().unwrap();

    loop {
        interface.set_channel(1 as usize, 255 as u8);
        for i in 1..2 {
            interface.set_channel(i as usize, 0 as u8);
            interface.set_channel(i+1 as usize, 255 as u8);
            //interface.buffer[1] = interface.buffer[1] + 10;
            interface.render().unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    }
}