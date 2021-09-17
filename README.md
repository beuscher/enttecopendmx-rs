# enttecopendmx-rs

**DISCLAIMER**: still in testing

Rust bindings for the [Enttec Open DMX USB](https://www.enttec.com/product/lighting-communication-protocols/dmx512/open-dmx-usb/) using the [FTDI D2XX Chip](https://ftdichip.com/product-category/products/ic/).

The code is roughtly based on the programming examples provided by Enttec, from which i extracted the settings for the FTDI Chip. For communication with the Chip the excellent crate [libftd2xx](https://crates.io/crates/libftd2xx) as a safe wrapper of the [FTDI 2DXX driver](https://ftdichip.com/drivers/d2xx-drivers/) c-bindings from the [libftd2xx-ffi](https://crates.io/crates/libftd2xx-ffi) crate.


## TODO:

- [ ] restructure as a crate with example
- [ ] add ` Result<(),Error> ` return values for methods
- [ ] improve and publish to crates.io 
- [ ] Documentation

## Installation/Setup

- install the [FTDI 2DXX driver](https://ftdichip.com/drivers/d2xx-drivers/)
- for the moment: download the repository and import the module using ` mod RustFTDIOpenDMX ` 

## Usage/Examples
*TODO*
