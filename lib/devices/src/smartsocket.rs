//! Module for Smarthome Socket
//! can enable/disable
//! show status
use std::fmt;

/// Smarthome socket
///
/// -`text`         - device description
/// -`enabled`      - device is enabled
/// -`power`        - current power consumption
#[allow(non_snake_case)] //to avoid rust erratic warning about milliwatt abbreviation
#[repr(C)]
pub struct SmartSocket {
    pub text: String,  // device description
    pub enabled: bool, // device is enabled
    pub power_mW: u32, // device power consuption in mW
}
#[warn(non_snake_case)]

impl SmartSocket {
    /*** interface ***/

    /// Socket ctor
    ///
    /// -`socket_text`  - socket description
    ///
    /// -`return`     - new socket instance
    pub fn new(socket_text: &str) -> Self {
        SmartSocket {
            text: socket_text.to_string(),
            enabled: false,
            power_mW: 0,
        }
    }

    /// Enable socket
    pub fn en(&mut self) {
        if !self.enabled {
            self.enabled = true;
            self.power_mW = 0;
        }
    }

    /// Disable socket
    pub fn dis(&mut self) {
        if self.enabled {
            self.enabled = false;
            self.power_mW = 0;
        }
    }

    /// Update socket status
    pub fn update(&mut self) {
        /* need to update the current state of soket,
         * if user manually swith its state
         */
        if self.enabled {
            /* need to get current power consumption of socket,
             * if it changed
             */
            self.power_mW = 10000;
        } else {
            self.power_mW = 0;
        }
    }
}

impl Drop for SmartSocket {
    /// Delete socket from network
    fn drop(&mut self) {
        /* here should be radio network
         * actions to inform the socket
         * about fact, that is not used anymore
         */
    }
}

impl fmt::Display for SmartSocket {
    /// Socket print implementation
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let state_string: &'static str = if self.enabled { "On" } else { "Off" };
        let out_s = &format!(
            "Name = {}\tState = {}\tPower = {} mW",
            self.text, state_string, self.power_mW
        );
        fmt.write_str(out_s)?;
        Ok(())
    }
}

/* linked list have special clone rules */
impl Clone for SmartSocket {
    /// To copy smartsocket need to copy it's fields
    /// care! if we add the uid, this will be changed
    fn clone(&self) -> Self {
        SmartSocket {
            text: self.text.clone(),
            enabled: self.enabled,
            power_mW: self.power_mW,
        }
    }
}

impl Default for SmartSocket {
    /// Default is disabled socket with name smart socket
    fn default() -> Self {
        SmartSocket {
            text: "smart socket".to_string(),
            enabled: false,
            power_mW: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::SmartSocket;

    #[test]
    fn test_socket() {
        let mut socket0 = SmartSocket::new("Socket in kitchen");
        socket0.en();
        assert!(socket0.enabled);
        socket0.dis();
        assert!(!socket0.enabled);
        socket0.update();
        assert_eq!(0, socket0.power_mW);
    }
}
