//! # Instrument
//! 
//! The main object used for managing and using instruments connected by an interface of choice.
//! 

use std::time::Duration;

mod interface;
use interface::InstrumentClient;
use rs_usbtmc::UsbtmcClient;

use anyhow::{Result, anyhow};

pub struct Instrument {
    client: Box<dyn InstrumentClient>,
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
            let client = Box::new(UsbtmcClient::connect(vid, pid)?);

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