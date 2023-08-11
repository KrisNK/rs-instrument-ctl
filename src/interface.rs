//! ## Interface
//! 
//! Module for the InstrumentClient trait.
//! 

use anyhow::Result;
use rs_usbtmc::UsbtmcClient;

/// ### InstrumentClient
///
/// Trait representing a client connecting to an instrument.
///
pub trait InstrumentClient {
    /// ### Set Timeout
    ///
    /// Set a new timeout for the device connection.
    ///
    /// #### Arguments
    /// - `duration` -> the duration of the timeout
    ///
    fn set_timeout(&self, duration: std::time::Duration);

    /// ### Command
    ///
    /// Send a command to the device.
    ///
    /// #### Arguments
    /// - `cmd` -> the command to send
    ///
    fn command(&self, cmd: &str) -> Result<()>;

    /// ### Query Raw
    ///
    /// Send a command and get a response from the device.
    /// The response is a vector of bytes.
    ///
    /// #### Arguments
    /// - `cmd` -> the command to send
    ///
    fn query_raw(&self, cmd: &str) -> Result<Vec<u8>>;

    /// ### Query
    ///
    /// Send a command and get a response from the device.
    /// The response is a utf-8 string.
    ///
    /// #### Arguments
    /// - `cmd` -> the command to send
    ///
    fn query(&self, cmd: &str) -> Result<String>;
}

// IMPLEMENTATIONS OF THE DIFFERENT INTERFACES
// ==========

// USBTMC
// ----------

impl InstrumentClient for UsbtmcClient {
    fn command(&self, cmd: &str) -> Result<()> {
        self.command(cmd)
    }

    fn query(&self, cmd: &str) -> Result<String> {
        self.query(cmd)
    }

    fn query_raw(&self, cmd: &str) -> Result<Vec<u8>> {
        self.query_raw(cmd)
    }

    fn set_timeout(&self, duration: std::time::Duration) {
        self.set_timeout(duration)
    }
}