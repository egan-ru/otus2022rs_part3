//! Container for devices
pub mod smartsocket;
pub mod smartthermometer;
use crate::smartsocket::SmartSocket;
use crate::smartthermometer::SmartThermometer;

/// Smarthome device, for a while can be:
/// 0. `Nodev`
/// 1. `Socket`
/// 2. `Thermometer`
pub enum SmartDevice {
    Nodev,
    Socket(SmartSocket),
    Thermometer(SmartThermometer),
}

/// Contain methods to easy access
/// to smart device
pub trait SmartDeviceAccess {
    /// Get device name
    /// -`return` device name
    fn name(&self) -> String;
    /// Get device status
    /// -`return' device status
    fn status(&self) -> String;
}

impl SmartDeviceAccess for SmartDevice {
    /// Get device name
    /// -`return` device name
    fn name(&self) -> String {
        match self {
            SmartDevice::Socket(the_socket) => the_socket.text.clone(),
            SmartDevice::Thermometer(the_thermometer) => the_thermometer.text.clone(),
            SmartDevice::Nodev => "not device".to_string(),
        }
    }

    /// Get device status
    /// -`return' device status
    fn status(&self) -> String {
        match self {
            SmartDevice::Socket(the_socket) => {
                /* todo: here should be lang translation table access */
                let name_str: &'static str = "name: ";
                let status_str: &'static str = "\nstatus: ";
                let on_str: &'static str = "on";
                let off_str: &'static str = "off";
                let power_str: &'static str = "\npower: ";
                #[allow(non_snake_case)]
                //to avoid rust erratic warning about milliwatt abbreviation
                let mW_str: &'static str = " mW ";
                #[warn(non_snake_case)] //to avoid rust erratic warning about milliwatt abbreviation
                let curr_status_str: &'static str =
                    if the_socket.enabled { on_str } else { off_str };

                /* construct status */
                let mut dev_status: String = name_str.to_string();
                dev_status.push_str(&the_socket.text);
                dev_status.push_str(status_str);
                dev_status.push_str(curr_status_str);
                if the_socket.enabled {
                    dev_status.push_str(power_str);
                    dev_status.push_str(&the_socket.power_mW.to_string());
                    dev_status.push_str(mW_str);
                };

                dev_status
            }

            SmartDevice::Thermometer(the_thermometer) => {
                /* todo: here should be lang translation table access */
                let name_str: &'static str = "name: ";
                let temp_str: &'static str = "\ntemp: ";

                /* construct status */
                let mut dev_status: String = name_str.to_string();
                dev_status.push_str(&the_thermometer.text);
                dev_status.push_str(temp_str);
                dev_status.push_str(&the_thermometer.temp.to_string());

                dev_status
            }

            SmartDevice::Nodev => "not device".to_string(),
        }
    }
}

impl Default for SmartDevice {
    /// Defauld smart device is not a device
    fn default() -> Self {
        SmartDevice::Nodev
    }
}

/* clone for smart device have special rules */
impl Clone for SmartDevice {
    fn clone(&self) -> Self {
        match self {
            SmartDevice::Socket(the_socket) => SmartDevice::Socket(the_socket.clone()),
            SmartDevice::Thermometer(the_thermometer) => {
                SmartDevice::Thermometer(the_thermometer.clone())
            }
            SmartDevice::Nodev => SmartDevice::Nodev,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SmartDevice;
    use crate::SmartDeviceAccess;
    use crate::SmartSocket;
    use crate::SmartThermometer;
    #[test]
    fn test_smartdevice_access() {
        let mut smartsocket0: SmartSocket = SmartSocket::new("test socket0");
        smartsocket0.en();
        let smartthermometer0: SmartThermometer = SmartThermometer::new("test thermometer");
        let device0: SmartDevice = SmartDevice::Socket(smartsocket0);
        let device1: SmartDevice = SmartDevice::Thermometer(smartthermometer0);

        let name0: String = device0.name();
        let stat0: String = device0.status();
        let name1: String = device1.name();
        let stat1: String = device1.status();
        println!("{}", name0);
        println!("{}", stat0);
        println!("{}", name1);
        println!("{}", stat1);
    }
}
