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

    // change sleeptime here (in ms) to modify the speed of the rgb chase
    const SLEEPTIME: u64 = 100;

    interface.open().unwrap();

    loop {
        for i in 1..4 {
            interface.set_channel(i as usize, 255 as u8);
            //interface.buffer[1] = interface.buffer[1] + 10;
            interface.render().unwrap();
            interface.set_channel(i as usize, 0 as u8);
            thread::sleep(Duration::from_millis(SLEEPTIME));
        }
    }
}