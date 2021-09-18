# enttecopendmx-rs

**DISCLAIMER**: still in testing

Rust bindings for the [Enttec Open DMX USB](https://www.enttec.com/product/lighting-communication-protocols/dmx512/open-dmx-usb/) and its [FTDI D2XX Chip](https://ftdichip.com/product-category/products/ic/).

The code is roughly based on the programming examples provided by Enttec, from which I extracted the settings for the FTDI Chip. For communication with the chip the excellent crate [libftd2xx](https://crates.io/crates/libftd2xx) as a safe wrapper of the [FTDI 2DXX driver](https://ftdichip.com/drivers/d2xx-drivers/) c-bindings from the [libftd2xx-ffi](https://crates.io/crates/libftd2xx-ffi) crate.

## Usage


First install the [FTDI 2DXX driver](https://ftdichip.com/drivers/d2xx-drivers/)

Then, simply add this crate as a dependency in your `Cargo.toml`.

```toml
[dependencies]
enttecopendmx = "0.1.0"
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
or simply run `cargo run --example rgb-chase` in the root directory of the (github) repository.

## Changelog

The changelog can be found in the [Changelog.md](https://github.com/Beuscher/enttecopendmx-rs/blob/main/Changelog.md) file.


## Issues

If you encounter any problems just open an issue, but a response may take some time.

## TODO:

- [x] restructure as a crate with example
- [ ] add ` Result<T,E> ` return values for all methods
- [x] publish to crates.io 
- [ ] improve Documentation
- [ ] add an alternative version(s) of the `EnttecOpenDMX::new()` method, which allows to specify the interface eg. by serial number
- [ ] add a cli (binary) for simplified testing

## Contributing

This crate is still in an early version, so if you have suggestions for improvements regarding this crate feel free to open a pull request.