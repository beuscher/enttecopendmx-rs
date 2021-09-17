# enttecopendmx-rs

**DISCLAIMER**: still in testing

Rust bindings for the [Enttec Open DMX USB](https://www.enttec.com/product/lighting-communication-protocols/dmx512/open-dmx-usb/) and its [FTDI D2XX Chip](https://ftdichip.com/product-category/products/ic/).

The code is roughly based on the programming examples provided by Enttec, from which I extracted the settings for the FTDI Chip. For communication with the chip the excellent crate [libftd2xx](https://crates.io/crates/libftd2xx) as a safe wrapper of the [FTDI 2DXX driver](https://ftdichip.com/drivers/d2xx-drivers/) c-bindings from the [libftd2xx-ffi](https://crates.io/crates/libftd2xx-ffi) crate.


## TODO:

- [ ] restructure as a crate with example
- [ ] add ` Result<T,E> ` return values for all methods
- [ ] publish to crates.io 
- [ ] improve Documentation

## Usage


First install the [FTDI 2DXX driver](https://ftdichip.com/drivers/d2xx-drivers/)

Then, simply add this crate as a dependency in your `Cargo.toml`.

```toml
[dependencies]
version = "~0.1.0"
``` 
### Basic Example

This basic example sets the output of channel 1 to the maximum value of 255.

```rust
use enttecopendmx;

let mut interface = enttecopendmx::EnttecOpenDMX::new();
interface.open();
interface.set_channel(1 as usize, 255 as u8);
interface.render();
```

## Issues

If you encounter any problems just open an issue, but a response may take some time.

## Contributing

This crate is still in an early version, so if you have suggestions for improvements regarding this crate feel free to open a pull request.