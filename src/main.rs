//! rust cource part3
extern crate dll;
use dll::Dll;
extern crate devices;
use devices::smartsocket::SmartSocket;
use devices::smartthermometer::SmartThermometer;
use devices::SmartDevice;
use devices::SmartDeviceAccess;

/// Smart room
/// -`text`     - smart room description
/// -`nurse`    - device double linked list sentinel
#[repr(C)]
pub struct SmartRoom {
    text: String,
    nurse: Dll<SmartDevice>,
}

impl SmartRoom {
    /// Room ctor
    ///
    /// -`room_text`  - room description
    ///
    /// -`return`     - new room instance
    pub fn new(room_text: &str) -> Self {
        let mut room: SmartRoom = SmartRoom {
            text: room_text.to_string(),
            nurse: Dll::new(),
        };

        /* nurse is complex type and must be relinked
         * todo: Dll ctor issue, will be reworked later
         */
        room.nurse.relink();

        room
    }

    /// Add device into the room
    /// -`dev`      - device to add
    pub fn dev_add(&mut self, dev: &mut Dll<SmartDevice>) {
        let nurse: &mut Dll<SmartDevice> = &mut self.nurse;
        nurse.addh(dev);
    }

    /// Find device by name
    /// -`dev_name`     - device name
    ///
    /// -`return`       - first finded device reference, or None, if device not present
    pub fn dev_find(&mut self, dev_name: &str) -> Option<&mut SmartDevice> {
        let nurse: &mut Dll<SmartDevice> = &mut self.nurse;
        let mut sel: *mut Dll<SmartDevice> = nurse.next;
        let nurse_addr: *mut Dll<SmartDevice> = nurse;
        unsafe {
            while nurse_addr != sel {
                /* get device */
                let dev: &SmartDevice = &((*sel).data);
                /* get info from device */
                let sel_dev_name: &String = &dev.name();

                if dev_name.eq(sel_dev_name) {
                    /* device with such name found */
                    return Some(&mut ((*sel).data));
                }

                sel = (*sel).next;
            }
        }
        /* device with such name not found */
        Option::None
    }

    /// Room info request
    /// -`return`       - smart room full information  
    pub fn info(&self) -> String {
        let nurse: &Dll<SmartDevice> = &self.nurse;
        let mut rinfo: String = self.text.to_string();
        /* newline for a while will be a delimiter */
        let delm: &'static str = "\n";
        rinfo.push_str(delm);

        /* for every device print device info */
        let mut sel: *mut Dll<SmartDevice> = nurse.next;
        let nurse_addr: *const Dll<SmartDevice> = nurse;

        unsafe {
            while nurse_addr != sel {
                /* get device */
                let dev: &SmartDevice = &((*sel).data);
                /* get info from device */
                let dev_info: String = dev.status();

                rinfo.push_str(&dev_info);
                rinfo.push_str(delm);
                sel = (*sel).next;
            }
        }
        rinfo
    }
}

/* linked list have special clone rules */
impl Clone for SmartRoom {
    fn clone(&self) -> Self {
        let mut new_dll: Self = Self {
            text: self.text.clone(),
            nurse: self.nurse.clone(),
        };

        /* relink smart room anyway */
        new_dll.nurse.relink();

        new_dll
    }
}

impl Default for SmartRoom {
    fn default() -> Self {
        let mut the_dll: Self = Self {
            text: "Room".to_string(),
            nurse: Dll::default(),
        };

        /* relink the data anyway */
        the_dll.nurse.relink();

        the_dll
    }
}

/// Smart house
/// -`text`     - smart house description
/// -`nurse`    - room double linked list sentinel
#[repr(C)]
struct SmartHouse {
    text: String,
    nurse: Dll<SmartRoom>,
}

impl SmartHouse {
    /// House ctor
    ///
    /// -`house_text`  - house description
    ///
    /// -`return`     - new house instance
    pub fn new(house_text: &str) -> Self {
        let mut house: SmartHouse = SmartHouse {
            text: house_text.to_string(),
            nurse: Dll::new(),
        };

        /* nurse is complex type and must be relinked
         * todo: Dll ctor issue, will be reworked later
         */
        house.nurse.relink();

        house
    }

    /// Add room into the house
    /// -`room`      - device to add
    pub fn room_add(&mut self, room: &mut Dll<SmartRoom>) {
        let nurse: &mut Dll<SmartRoom> = &mut self.nurse;
        nurse.addh(room);
    }

    /// Find device by name
    /// -`room_name`     - device name
    ///
    /// -`return`       - first finded device reference, or None, if device not present
    pub fn room_find(&mut self, room_name: &str) -> Option<&mut SmartRoom> {
        let nurse: &mut Dll<SmartRoom> = &mut self.nurse;
        let mut sel: *mut Dll<SmartRoom> = nurse.next;
        let nurse_addr: *mut Dll<SmartRoom> = nurse;
        unsafe {
            while nurse_addr != sel {
                /* get device */
                let room: &SmartRoom = &((*sel).data);
                /* get info from device */
                let sel_room_name: &String = &room.text;

                if room_name.eq(sel_room_name) {
                    /* device with such name found */
                    return Some(&mut ((*sel).data));
                }

                sel = (*sel).next;
            }
        }
        /* device with such name not found */
        Option::None
    }

    /// Find device with selected name in selected room
    /// -`room_name`    - room, where device may be located
    /// -`dev_name`     - target device name
    ///
    /// -`return`       - first finded device reference, or None, if nothing found
    pub fn room_dev_find(&mut self, room_name: &str, dev_name: &str) -> Option<&mut SmartDevice> {
        let maybe_room: Option<&mut SmartRoom> = self.room_find(room_name);
        if let Some(the_room) = maybe_room {
            let maybe_device: Option<&mut SmartDevice> = the_room.dev_find(dev_name);
            /* device may be found */
            return maybe_device;
        }
        /* device with such name not found in room with such name */
        Option::None
    }

    /// Room info request
    /// -`return`       - smart room full information  
    pub fn info(&self) -> String {
        let nurse: &Dll<SmartRoom> = &self.nurse;
        let mut rinfo: String = self.text.to_string();
        /* newline for a while will be a delimiter */
        let delm: &'static str = "\n";
        rinfo.push_str(delm);

        /* for every device print device info */
        let mut sel: *mut Dll<SmartRoom> = nurse.next;
        let nurse_addr: *const Dll<SmartRoom> = nurse;

        unsafe {
            while nurse_addr != sel {
                /* get device */
                let room: &SmartRoom = &((*sel).data);
                /* get info from device */
                let room_info: String = room.info();

                rinfo.push_str(&room_info);
                rinfo.push_str(delm);
                sel = (*sel).next;
            }
        }
        rinfo
    }
}

/// Task3 main routine
fn main() {
    println!("Task3 start\n");

    println!(
        "Generate devices\n
             every room will have one thermometer\n
             and one socket:\n"
    );

    /* generate devices */
    let mut smartsocket0: SmartSocket = SmartSocket::new("socket0");
    let mut smartsocket1: SmartSocket = SmartSocket::new("socket1");
    smartsocket0.en();
    smartsocket1.en();
    let smartthermometer0: SmartThermometer = SmartThermometer::new("thermometer0");
    let smartthermometer1: SmartThermometer = SmartThermometer::new("thermometer1");
    let device0: SmartDevice = SmartDevice::Socket(smartsocket0);
    let device1: SmartDevice = SmartDevice::Thermometer(smartthermometer0);
    let device2: SmartDevice = SmartDevice::Socket(smartsocket1);
    let device3: SmartDevice = SmartDevice::Thermometer(smartthermometer1);

    let name0: String = device0.name();
    let stat0: String = device0.status();
    let name1: String = device1.name();
    let stat1: String = device1.status();
    let name2: String = device2.name();
    let stat2: String = device2.status();
    let name3: String = device3.name();
    let stat3: String = device3.status();

    println!("\t{}", name0);
    println!("\t{}", stat0);
    println!("\t{}", name1);
    println!("\t{}", stat1);
    println!("\t{}", name2);
    println!("\t{}", stat2);
    println!("\t{}", name3);
    println!("\t{}", stat3);

    /* generate rooms with devices */
    println!("\nGenerate rooms: room0 and room1");
    /* wrap devices into double linked list */
    let mut dev0: Dll<SmartDevice> = Dll::from(device0);
    let mut dev1: Dll<SmartDevice> = Dll::from(device1);
    let mut dev2: Dll<SmartDevice> = Dll::from(device2);
    let mut dev3: Dll<SmartDevice> = Dll::from(device3);

    /* room 0 have device0 and device 1 */
    let room0: SmartRoom = SmartRoom::new("room0");
    /* wrap room into linked list */
    let mut rm0: Dll<SmartRoom> = Dll::from(room0);
    /* this relinks need due invesed house construction
     * and will be fixed in future Dll releases
     */
    rm0.data.nurse.relink();

    /* get hangle back, for easy access */
    let room0: &mut SmartRoom = &mut rm0.data;
    room0.dev_add(&mut dev0);
    room0.dev_add(&mut dev1);

    let room1: SmartRoom = SmartRoom::new("room1");
    /* wrap into linked list */
    let mut rm1: Dll<SmartRoom> = Dll::from(room1);
    /* this relinks need due invesed house construction
     * and will be fixed in future Dll releases
     */
    rm1.data.nurse.relink();

    /* get hangle back, for easy access */
    let room1: &mut SmartRoom = &mut rm1.data;
    room1.dev_add(&mut dev2);
    room1.dev_add(&mut dev3);

    /* print room information */
    let room0_info: String = room0.info();
    let room1_info: String = room1.info();

    println!("room0 info: {}\n", room0_info);
    println!("room1 info: {}\n", room1_info);

    println!("\nCheck, find method");

    let check_find0: Option<&mut SmartDevice> = room0.dev_find("socket0");
    if check_find0.is_some() {
        println!("Socket0 finded in room0");
    } else {
        println!("Socket0 not finded in room0");
    }

    /* generate house */
    println!("Generate houses: house0");
    let mut house0: SmartHouse = SmartHouse::new("House0");
    house0.room_add(&mut rm0);
    house0.room_add(&mut rm1);

    /* print house info */
    let house_info: String = house0.info();
    println!("\nHouse info:\n {}", house_info);

    /* find room in the house */
    let maybe_room: Option<&mut SmartRoom> = house0.room_find("room0");
    if maybe_room.is_some() {
        println!("room0 found");
    } else {
        println!("room0 not found");
    }

    /* find device in room in house */
    let maybe_device: Option<&mut SmartDevice> = house0.room_dev_find("room0", "socket0");
    if maybe_device.is_some() {
        println!("socket0 found in room0");
    } else {
        println!("socket0 not found in room0");
    }

    println!("\nTask3 done\n");
}
