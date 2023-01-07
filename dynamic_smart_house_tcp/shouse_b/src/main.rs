use lib_shouse::home::home::home::*;
use std::cell::RefCell;
use std::rc::Rc;

static mut SMART_SOCKET_SERIAL: usize = 0;
static mut TERMOMETER_SERIAL: usize = 0;

struct Termometer {
    name: String,
    state: bool,
    temperature: f32,
}

struct SmartSocket {
    name: String,
    state: bool,
    consum_power: f32,
}

fn main() {
    let mut some_house = SmartHouse::new();
    let room_0 = "room_0".to_string();
    let room_1 = "room_1".to_string();
    assert!(some_house.append_room(&room_0).is_ok()); // append room
    assert!(some_house.append_room(&room_1).is_ok()); // append room
    assert!(some_house.test_whether_room_exists(&room_1).is_some()); // room succ. added
    assert!(some_house.delete_room(&room_1).is_ok()); // deleted!!  comment it to panic!
    assert!(some_house.test_whether_room_exists(&room_1).is_none()); // check not exists room
                                                                     // now!!!!
    assert!(some_house.append_room(&room_1).is_ok()); // append room AFTER DELETE
                                                      //
    let dev0: Rc<RefCell<dyn Device>> = Rc::new(RefCell::new(SmartSocket::new()));
    let dev1: Rc<RefCell<dyn Device>> = Rc::new(RefCell::new(Termometer::new()));

    let dev2: Rc<RefCell<dyn Device>> = Rc::new(RefCell::new(SmartSocket::new()));
    let dev3: Rc<RefCell<dyn Device>> = Rc::new(RefCell::new(Termometer::new()));
    let dev4: Rc<RefCell<dyn Device>> = Rc::new(RefCell::new(Termometer::new()));

    let _dev0_handler = some_house.append_dev_to_a_room(&room_0, &dev0).unwrap(); // append dev to room0
    let _dev1_handler = some_house.append_dev_to_a_room(&room_0, &dev1).unwrap(); // append dev to room0

    let _dev2_handler = some_house.append_dev_to_a_room(&room_1, &dev2).unwrap(); // append dev to room1
    let _dev3_handler = some_house.append_dev_to_a_room(&room_1, &dev3).unwrap(); // append dev to room1
    let _dev4_handler = some_house.append_dev_to_a_room(&room_1, &dev4).unwrap();
    //    let _dev4_handler = some_house.append_dev_to_a_room(&room_1, &dev4).unwrap(); // panic if add
    // existing
    // device
    assert!(some_house // delete dev2 in room1
        .delete_device(&room_1, dev2.borrow().get_name().as_str())
        .is_ok());
    // now we can add dev2 to room1!!!!
    let _dev4_handler = some_house.append_dev_to_a_room(&room_1, &dev2).unwrap();

    // get report
    println!(
        " here is report about {}: {}",
        dev0.borrow().get_name(),
        some_house.get_dev_report(&dev0).unwrap()
    );

    // get state
    println!(
        "current property state: {}",
        _dev0_handler.get_property_state().unwrap()
    );
    println!(
        "current property state: {}",
        _dev1_handler.get_property_state().unwrap()
    );
    _dev1_handler.property_change_state(36.6_f32).unwrap();
    _dev0_handler.property_change_state(9000_f32).unwrap();
    println!(
        "current property state: {}",
        _dev0_handler.get_property_state().unwrap()
    );
    println!(
        "current property state: {}",
        _dev1_handler.get_property_state().unwrap()
    );
}

impl SmartSocket {
    fn new() -> Self {
        unsafe {
            let out = Self {
                name: [
                    "smart_socket_",
                    "#",
                    SMART_SOCKET_SERIAL.to_string().as_str(), // complicated
                ]
                .concat(),
                state: false,
                consum_power: 0.0,
            };
            SMART_SOCKET_SERIAL += 1_usize;
            out
        }
    }
    fn set_cons_power(&mut self, temp: f32) {
        self.consum_power = temp;
    }
}
impl Termometer {
    fn new() -> Self {
        unsafe {
            let out = Self {
                name: [
                    "termometer_",
                    "#",
                    TERMOMETER_SERIAL.to_string().as_str(), // complicated
                ]
                .concat(),
                state: false,
                temperature: 0.0,
            };
            TERMOMETER_SERIAL += 1_usize;
            out
        }
    }
    fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
    }
}

impl Device for Termometer {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn set_state(&mut self, state: bool) {
        self.state = state
    }
    fn get_state(&self) -> bool {
        self.state
    }
    fn get_property_info(&self) -> String {
        format!("current temperature is {}", self.temperature)
    }
    fn set_property_info(&mut self, new_info: &dyn std::fmt::Display) {
        self.temperature = new_info.to_string().parse::<f32>().unwrap();
    }
}
impl Device for SmartSocket {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn set_state(&mut self, state: bool) {
        self.state = state
    }
    fn get_state(&self) -> bool {
        self.state
    }
    fn get_property_info(&self) -> String {
        format!("current power consumption is {}", self.consum_power)
    }
    fn set_property_info(&mut self, new_info: &dyn std::fmt::Display) {
        self.consum_power = new_info.to_string().parse::<f32>().unwrap();
    }
}
