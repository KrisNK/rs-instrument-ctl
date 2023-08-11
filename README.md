# Instrument Interface

Connect to, command, and query intruments such as oscilloscopes. 

Currently only supports USBTMC connections.

Supported interfaces:
- [x] USBTMC
- [ ] VXI-11

Considered interfaces:
- [ ] GPIB (reason not to: it is old)
- [ ] VICP (reason not to: proprietary)
- [ ] LSIB (reason not to: way too proprietary)

Eventually, and depending on my motivation, this project will become a pure Rust VISA driver. However, in my current position this seems to be a pipe dream.

## Usage

To use, add the following line to your project's Cargo.toml dependencies:
```toml
rs-instrument-ctl = "0.1"
```

## Example

The example below demonstrates how to connect to, send commands to and query the device. 

```rust
use rs_instrument_ctl::Instrument;

const VISA_ADDRESS: &str = "USB::0x0000::0x0000::SERIALNUMBER::INSTR"

fn main() {
    // connect to the instrument
    let instrument = Instrument::connect(VISA_ADDRESS).expect("failed to connect to device");

    // send a command
    instrument.command("*IDN").expect("failed to send command");

    // query the device and return the response as a string
    let response: String = instrument.query("*IDN?").expect("failed to query");

    // query the device and return the response as a vector of bytes
    let response: Vec<u8> = instrument.query("*IDN?").expect("failed to query");
}
```