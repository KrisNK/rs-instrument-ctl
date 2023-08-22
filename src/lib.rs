//! # Instrument Interface
//! 
//! Connect to, command, and query intruments such as oscilloscopes. 
//! 
//! Currently only supports USBTMC connections.
//! 
//! Supported interfaces:
//! - [x] USBTMC
//! - [ ] VXI-11
//! 
//! Considered interfaces:
//! - [ ] GPIB (reason not to: it is old)
//! - [ ] VICP (reason not to: proprietary)
//! - [ ] LSIB (reason not to: way too proprietary)
//! 
//! Eventually, and depending on my motivation, this project will become a pure Rust VISA driver. However, in my current position this seems to be a pipe dream.
//! 
//! ## Usage
//! 
//! To use, add the following line to your project's Cargo.toml dependencies:
//! ```toml
//! rs-instrument-ctl = "0.1"
//! ```
//! 
//! ## Example
//! 
//! The example below demonstrates how to connect to, send commands to and query the device. 
//! 
//! ```rust
//! use rs_instrument_ctl::Instrument;
//! 
//! const VISA_ADDRESS: &str = "USB::0x0000::0x0000::SERIALNUMBER::INSTR"
//! 
//! fn main() {
//!     // connect to the instrument
//!     let instrument = Instrument::connect(VISA_ADDRESS).expect("failed to connect to device");
//! 
//!     // send a command
//!     instrument.command("*IDN").expect("failed to send command");
//! 
//!     // query the device and return the response as a string
//!     let response: String = instrument.query("*IDN?").expect("failed to query");
//! 
//!     // query the device and return the response as a vector of bytes
//!     let response: Vec<u8> = instrument.query("*IDN?").expect("failed to query");
//! }
//! ```
//! 

use std::time::Duration;
use std::sync::Arc;

mod interface;
use interface::InstrumentClient;
use rs_usbtmc::UsbtmcClient;

use anyhow::{Result, anyhow};

pub struct Instrument {
    client: Arc<dyn InstrumentClient + 'static>,
}

impl Instrument {
    /// ## Connect
    /// 
    /// Connect to a compatible device with a VISA address
    /// 
    /// ### Arguments
    /// - `address` -> a valid VISA address
    /// 
    pub fn connect(address: &str) -> Result<Instrument> {
        // parse the address to figure out which interface to use
        let addr: Vec<&str> = address.split("::").collect();

        let interface = addr[0];

        if interface.contains("USB") {
            let vid = match addr[1].strip_prefix("0x") {
                Some(s) => s,
                None => addr[1],
            };
            let pid = match addr[2].strip_prefix("0x") {
                Some(s) => s,
                None => addr[2],
            };
            
            // the device is USB
            let vid = u16::from_str_radix(vid, 16)?;
            let pid = u16::from_str_radix(pid, 16)?;
            let client = Arc::new(UsbtmcClient::connect(vid, pid)?);

            return Ok(Instrument { client })
        } else {
            return Err(anyhow!("unrecognized protocol"))
        }
    }

    /// ### Set Timeout
    ///
    /// Set a new timeout for the device connection.
    ///
    /// #### Arguments
    /// - `duration` -> the duration of the timeout
    ///
    pub fn set_timeout(&self, duration: Duration) {
        self.client.set_timeout(duration);
    }

    /// ### Command
    ///
    /// Send a command to the device.
    ///
    /// #### Arguments
    /// - `cmd` -> the command to send
    ///
    pub fn command(&self, cmd: &str) -> Result<()> {
        self.client.command(cmd)
    }

    /// ### Query
    ///
    /// Send a command and get a response from the device.
    /// The response is a utf-8 string.
    ///
    /// #### Arguments
    /// - `cmd` -> the command to send
    ///
    pub fn query(&self, cmd: &str) -> Result<String> {
        self.client.query(cmd)
    }

    /// ### Query Raw
    ///
    /// Send a command and get a response from the device.
    /// The response is a vector of bytes.
    ///
    /// #### Arguments
    /// - `cmd` -> the command to send
    ///
    pub fn query_raw(&self, cmd: &str) -> Result<Vec<u8>> {
        self.client.query_raw(cmd)
    }
}