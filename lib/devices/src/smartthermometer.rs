//! Module for Smarthome Thermometer
//! show status
use std::fmt;

/// Smarthome thermometer
///
/// -`text`  - device description
/// -`temp`  - current temperature in °K
#[repr(C)]
pub struct SmartThermometer {
    pub text: String, // device description
    pub temp: u16,    // device temperature in °K
}

impl SmartThermometer {
    /*** interface ***/

    /// Thermometer ctor
    ///
    /// -`thermometer_text`  - thermometer description
    ///
    /// -`return`     - new socket instance
    pub fn new(thermometer_text: &str) -> Self {
        SmartThermometer {
            text: thermometer_text.to_string(),
            temp: 273,
        }
    }

    /// Update thermometer status
    pub fn update(&mut self) {
        /* need to update the current state of thermometer */
        self.temp = 273 + 20;
    }
}

impl Drop for SmartThermometer {
    /// Delete thermometer from network
    fn drop(&mut self) {
        /* here should be radio network
         * actions to infrom the thermometer
         * about fact, that is not used anymore
         */
    }
}

impl fmt::Display for SmartThermometer {
    /// Thermometer print implementation
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let out_s = &format!("Name = {}\tTemp = {}", self.text, self.temp);
        fmt.write_str(out_s)?;
        Ok(())
    }
}

/* linked list have special clone rules */
impl Clone for SmartThermometer {
    /// To copy smartthermometer need to copy it's fields
    /// care! if we add the uid, this will be changed
    fn clone(&self) -> Self {
        SmartThermometer {
            text: self.text.clone(),
            temp: self.temp,
        }
    }
}

impl Default for SmartThermometer {
    /// Default is smartthermometer with name smart thermometer
    fn default() -> Self {
        SmartThermometer {
            text: "smart thermometer".to_string(),
            temp: 273,
        }
    }
}
