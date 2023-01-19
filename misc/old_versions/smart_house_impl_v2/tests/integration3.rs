use mylib::home::home::smart_house::{DeviceInfoProvider, SmartHouse};
use mylib::misc::house_creation::house_creation::create_home_instance;

struct SmartSocket {}
struct SmartThermometer {}

struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}
impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn provide_info(&self) -> Vec<&str> {
        vec![SMART_SOCKET_STR]
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn provide_info(&self) -> Vec<&str> {
        vec![SMART_SOCKET_STR, SMART_TERMOMETER_STR]
    }
}

const SMART_SOCKET_STR: &str = "smart socket";
const SMART_TERMOMETER_STR: &str = "smart termometer";

#[test]
fn integration_test_1_1() {
    let mut s1 = SmartHouse::new(); //create house
    s1.assign_name("house_one"); // assign a name
    let room1 = "room1".to_string(); //add room1
    let room2 = "room2".to_string(); // add room2
    s1.append_room(&room1).unwrap(); //append room
    s1.append_room(&room2).unwrap(); //append room
    let dev0 = "device0".to_string(); //device
    let dev1 = "device1".to_string(); //device
    let dev2 = "device2".to_string(); //device
                                      //    let dev_smart_socket = SMART_SOCKET_STR.to_string(); // append smart socket not here!
    s1.append_a_device(&room1, &dev0).unwrap(); // append to room1
    s1.append_a_device(&room1, &dev1).unwrap(); // append to room1
    s1.append_a_device(&room1, &dev2).unwrap(); // append to room1
                                                //    s1.append_a_device(&room1, &dev_smart_socket).unwrap(); // append socket to room1
                                                //    s1.append_a_device(&room2, &dev_smart_socket).unwrap(); // append socket to room2

    let socket1 = SmartSocket {};
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report_sockets = s1.create_report(&info_provider_1);
    assert_eq!(report_sockets.is_err(), true); // get ane rror here
    if let Err(e) = report_sockets {
        println!("error report");
        println!("an error while processing... {}", e);
    }
    println!("all devices:{}", s1.get_all_devices());
    println!("all rooms in {}", s1.get_all_rooms());
}
