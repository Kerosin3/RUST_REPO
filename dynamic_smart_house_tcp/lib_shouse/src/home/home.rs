pub mod home {
    /*    #![feature(associated_type_defaults)]
        #![feature(is_some_and)]
    */
    use std::any::Any;
    use std::cell::RefCell;
    use std::fmt;
    use std::rc::Rc;
    use std::rc::Weak;
    #[derive(thiserror::Error, Debug, Clone)]
    #[non_exhaustive]
    pub enum ErrorC {
        #[error("Internal error.")]
        Internal(String),
        #[error("Invalid argument: {0}")]
        InvalidArgument(String),
        #[error("Such room is already exists {0}")]
        RoomExists(String),
        #[error("Such room is not exists {0}")]
        RoomNotExists(String),
        #[error("This device is already exists in the room {0}")]
        DeviceInRoomExists(String),
        #[error("This device is not exists in the room {0}")]
        DeviceInRoomNotExists(String),
        #[error("Cannot change state")]
        ErrorSettingState,
        #[error("Cannot get state")]
        ErrorGettingState,
        #[error("Some error ocurred")]
        ErrorOther,
        #[error("Error execute handler command")]
        ErrorHandleOperation,
    }

    pub struct SmartHouse {
        rooms: Vec<Rc<dyn RoomObj>>,
    }

    impl SmartHouse {
        pub fn new() -> Self {
            Self { rooms: vec![] }
        }
        fn create_room(room_name: &str) -> Rc<dyn RoomObj> {
            // check whther exists
            Rc::new(Room_Generic {
                name: room_name.to_string(),
                devices: Box::new(RefCell::new(vec![])),
            })
        }
        pub fn append_room(&mut self, a_room: &str) -> Result<(), ErrorC> {
            if self.test_whether_room_exists(a_room).is_some() {
                Err(ErrorC::RoomExists(a_room.to_string()))
            } else {
                self.rooms.push(Rc::new(Room_Generic {
                    name: a_room.to_string(),
                    devices: Box::new(RefCell::new(vec![])), //devices store
                }));
                Ok(())
            }
        }

        fn get_device_number_in_room(&self, a_room: &str) -> Option<usize> {
            if let Some(n_room) = self.test_whether_room_exists(a_room) {
                let room_generic = self.rooms.get(n_room).unwrap().as_any();
                if let Some(obj) = room_generic.downcast_ref::<Room_Generic>() {
                    Some(obj.devices.borrow().len())
                } else {
                    None
                }
            } else {
                None
            }
        }
        pub fn test_whether_a_dev_exists(&self, dev_name: &str) -> Option<()> {
            for (_i, room) in self.rooms.iter().enumerate() {
                if room.find_dev_name(&dev_name).is_some() {
                    return Some(());
                } else {
                    continue;
                }
            }
            None
        }

        pub fn get_dev_report(&self, a_device: &Rc<RefCell<dyn Device>>) -> Option<String> {
            let mut out_str = String::new();
            let dev_name = a_device.borrow().get_name();
            for (_i, room) in self.rooms.iter().enumerate() {
                if room.find_dev_name(&dev_name).is_some() {
                    fmt::write(
                        &mut out_str,
                        format_args!("room: {},device: {} ,", room.get_room_name(), dev_name),
                    )
                    .expect("error whiting string")
                } else {
                    continue;
                }
            }
            if out_str.is_empty() {
                None
            } else {
                Some(out_str)
            }
        }
        pub fn delete_device(&self, a_room: &str, a_device: &str) -> Result<(), ErrorC> {
            if let Some(n_room) = self.test_whether_room_exists(a_room) {
                let generic_room_obj = self.rooms.get(n_room).unwrap().as_any();
                match generic_room_obj.downcast_ref::<Room_Generic>() {
                    // downcast to generic room
                    Some(obj) => {
                        if let Some(dev_num) = obj.find_dev_name(a_device) {
                            // find matched device
                            obj.devices.as_ref().borrow_mut().swap_remove(dev_num); // remove
                            Ok(())
                        } else {
                            Err(ErrorC::DeviceInRoomNotExists(a_device.to_owned()))
                        }
                    }
                    None => Err(ErrorC::ErrorOther),
                }
            } else {
                Err(ErrorC::RoomNotExists(a_room.to_string()))
            }
        }
        pub fn delete_room(&mut self, a_room: &str) -> Result<(usize, usize), ErrorC> {
            if let Some(n_room) = self.test_whether_room_exists(a_room) {
                let prev_size = self.rooms.len();
                self.rooms.swap_remove(n_room);
                let cur_size = self.rooms.len();
                Ok((prev_size, cur_size))
            } else {
                Err(ErrorC::RoomNotExists(a_room.to_string()))
            }
        }
        fn get_all_rooms(&self) -> Option<String> {
            let mut out_string = String::new();
            for (i, room) in self.rooms.iter().enumerate() {
                fmt::write(
                    &mut out_string,
                    format_args!("room {}: {}, ", i, room.get_room_name()),
                )
                .expect("error while writing to_string"); // how return None?
            }
            if out_string.is_empty() {
                None
            } else {
                Some(out_string)
            }
        }
        pub fn test_whether_room_exists(&self, a_room: &str) -> Option<usize> {
            if self.rooms.iter().any(|x| x.get_room_name() == a_room) {
                self.rooms.iter().position(|x| x.get_room_name() == a_room)
            } else {
                None
            }
        }

        pub fn append_dev_to_a_room(
            &mut self,
            a_room: &str,
            a_device: &Rc<RefCell<dyn Device>>,
        ) -> Result<Device_Handler, ErrorC> {
            if let Some(room_pos) = self.test_whether_room_exists(a_room) {
                if self
                    .rooms
                    .get(room_pos)
                    .unwrap()
                    .add_device(a_device)
                    .is_ok()
                {
                    Ok(Device_Handler::new(a_device))
                } else {
                    Err(ErrorC::DeviceInRoomExists(a_device.borrow().get_name()))
                }
            } else {
                Err(ErrorC::RoomNotExists(a_room.to_string()))
            }
        }

        fn get_devices_in_room(&self, a_room: &str) -> Result<String, ErrorC> {
            if let Some(room_pos) = self.test_whether_room_exists(a_room) {
                if let Some(dev_list) = self.rooms.get(room_pos).unwrap().get_all_devices() {
                    Ok(dev_list)
                } else {
                    Err(ErrorC::ErrorOther)
                }
            } else {
                Err(ErrorC::RoomNotExists(a_room.to_string()))
            }
        }

        fn change_dev_state_in_room(
            &mut self,
            a_room: &str,
            dev_name: &str,
            state: bool,
        ) -> Result<(), ErrorC> {
            if let Some(room_pos) = self.test_whether_room_exists(a_room) {
                if let Some(_dev_pos) = self.rooms.get(room_pos).unwrap().find_dev_name(dev_name) {
                    self.rooms
                        .get(room_pos)
                        .unwrap()
                        .change_dev_state(state, dev_name);
                    Ok(())
                } else {
                    Err(ErrorC::DeviceInRoomNotExists(a_room.to_string()))
                }
            } else {
                Err(ErrorC::RoomNotExists(a_room.to_string()))
            }
        }
    }

    trait RoomObj {
        fn get_room_name(&self) -> &str;
        fn add_device(&self, some_dev: &Rc<RefCell<dyn Device>>) -> Result<(), ErrorC>;
        fn find_dev_name(&self, name: &str) -> Option<usize>;
        fn change_dev_state(&self, state: bool, name: &str);
        fn get_all_devices(&self) -> Option<String>;
        fn as_any(&self) -> &dyn Any;
    }

    type Device_wrapper = Box<RefCell<Vec<Rc<RefCell<dyn Device>>>>>;

    struct Room_Generic {
        name: String,
        devices: Device_wrapper,
    }

    impl RoomObj for Room_Generic {
        fn get_room_name(&self) -> &str {
            self.name.as_str()
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn add_device(&self, some_dev: &Rc<RefCell<dyn Device>>) -> Result<(), ErrorC> {
            if self
                .find_dev_name(some_dev.borrow().get_name().as_ref())
                .is_some()
            {
                Err(ErrorC::DeviceInRoomExists(some_dev.borrow().get_name()))
            } else {
                self.devices.as_ref().borrow_mut().push(Rc::clone(some_dev));
                Ok(())
            }
        }
        fn find_dev_name(&self, name: &str) -> Option<usize> {
            if self
                .devices
                .as_ref()
                .borrow()
                .iter()
                .any(|x| x.borrow().get_name() == name)
            {
                Some(
                    self.devices
                        .as_ref()
                        .borrow()
                        .iter()
                        .position(|x| x.borrow().get_name() == name)
                        .unwrap(),
                )
            } else {
                None
            }
        }

        fn get_all_devices(&self) -> Option<String> {
            let mut out_string = String::new();
            for (j, dev) in self.devices.as_ref().borrow().iter().enumerate() {
                fmt::write(
                    &mut out_string,
                    format_args!("dev №{},name: {} ", j, dev.borrow().get_name()),
                )
                .expect("error while writing");
            }
            if out_string.is_empty() {
                None
            } else {
                Some(out_string)
            }
        }
        fn change_dev_state(&self, state: bool, name: &str) {
            if let Some(dev_pos) = self.find_dev_name(name) {
                self.devices
                    .as_ref()
                    .borrow_mut()
                    .get(dev_pos)
                    .unwrap()
                    .borrow_mut()
                    .set_state(state); //;get().set_state(state) ;
            }
        }
    }

    pub trait Device {
        fn get_name(&self) -> String;
        fn set_state(&mut self, state: bool);
        fn get_state(&self) -> bool;
        fn get_property_info(&self) -> String;
        fn set_property_info(&mut self, new_info: &dyn std::fmt::Display);
    }

    pub struct Device_Handler {
        dev: Weak<RefCell<dyn Device>>,
    }

    impl Device_Handler {
        fn new(dev: &Rc<RefCell<dyn Device>>) -> Self {
            Self {
                dev: Rc::downgrade(dev),
            }
        }
        pub fn get_devname(&self) -> Result<String, ErrorC> {
            if let Some(rez) = self.dev.upgrade() {
                Ok(rez.borrow().get_name())
            } else {
                Err(ErrorC::ErrorOther)
            }
        }
        pub fn change_state(&self, new_state: bool) -> Result<(), ErrorC> {
            if let Some(rez) = self.dev.upgrade() {
                rez.borrow_mut().set_state(new_state);
                Ok(())
            } else {
                Err(ErrorC::ErrorSettingState)
            }
        }
        pub fn get_state(&self) -> Result<bool, ErrorC> {
            if let Some(rez) = self.dev.upgrade() {
                Ok(rez.borrow().get_state())
            } else {
                Err(ErrorC::ErrorGettingState)
            }
        }
        pub fn property_change_state(
            &self,
            new_info: impl std::fmt::Display,
        ) -> Result<(), ErrorC> {
            if let Some(rez) = self.dev.upgrade() {
                Ok(rez.borrow_mut().set_property_info(&new_info.to_string()))
            } else {
                Err(ErrorC::ErrorGettingState)
            }
        }
        pub fn get_property_state(&self) -> Result<String, ErrorC> {
            if let Some(rez) = self.dev.upgrade() {
                Ok(rez.borrow().get_property_info())
            } else {
                Err(ErrorC::ErrorGettingState)
            }
        }
    }

    impl Device for Example_Device {
        fn get_name(&self) -> String {
            self.name.clone()
        }
        fn get_state(&self) -> bool {
            self.state
        }
        fn set_state(&mut self, state: bool) {
            self.state = state;
        }
        fn get_property_info(&self) -> String {
            format!("property is {}", self.property)
        }
        fn set_property_info(&mut self, new_info: &dyn std::fmt::Display) {
            self.property = new_info.to_string();
        }
    }

    struct Example_Device {
        name: String,
        state: bool,
        property: String,
    }

    impl Example_Device {
        fn new(name: String) -> Self {
            Self {
                name,
                property: "some_propery".to_string(),
                state: false,
            }
        }
    }

    fn main() {}

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn create_and_append_room() {
            let mut sh = SmartHouse::new();
            assert!(sh.append_room("room1").is_ok());
        }
        #[test]
        fn create_and_append_room_exists() {
            let mut sh = SmartHouse::new();
            assert!(sh.append_room("room1").is_ok());
            assert!(sh.append_room("room1").is_err());
        }
        #[test]
        fn dev_creation() {
            let mut sh = SmartHouse::new();
            assert!(sh.append_room("room1").is_ok());
            let dev: Rc<RefCell<dyn Device>> =
                Rc::new(RefCell::new(Example_Device::new("dev0".to_string())));
            assert!(sh.append_dev_to_a_room("room1", &dev).is_ok());
        }
        #[test]
        fn exists_dev_append() {
            let mut sh = SmartHouse::new();
            assert!(sh.append_room("room1").is_ok());
            let dev: Rc<RefCell<dyn Device>> =
                Rc::new(RefCell::new(Example_Device::new("dev0".to_string())));
            assert!(sh.append_dev_to_a_room("room1", &dev).is_ok());
            assert!(sh.append_dev_to_a_room("room1", &dev).is_err());
        }

        #[test]
        fn add_to_n_exists_room() {
            let mut sh = SmartHouse::new();
            assert!(sh.append_room("room2").is_ok());
            let dev: Rc<RefCell<dyn Device>> =
                Rc::new(RefCell::new(Example_Device::new("dev0".to_string())));
            assert!(sh.append_dev_to_a_room("room1", &dev).is_err());
        }
        #[test]
        fn dev_creation_and_change() {
            let mut sh = SmartHouse::new();
            assert!(sh.append_room("room1").is_ok());
            let dev: Rc<RefCell<dyn Device>> =
                Rc::new(RefCell::new(Example_Device::new("dev0".to_string())));
            let rez_handler1 = sh.append_dev_to_a_room("room1", &dev);
            assert!(rez_handler1.is_ok()); // get handler
            let handler1 = rez_handler1.unwrap(); // unwrap handler
            assert!(handler1.get_state().is_ok_and(|rez| rez == false));
            assert!(handler1.change_state(true).is_ok());
            assert!(handler1.get_state().is_ok_and(|rez| rez == true));
            assert!(sh.change_dev_state_in_room("room1", "dev0", false).is_ok());
            assert!(handler1.get_state().is_ok_and(|rez| rez == false));
        }
        #[test]
        fn dev_creation_and_test_properties() {
            let mut sh = SmartHouse::new();
            assert!(sh.append_room("room1").is_ok());
            let dev: Rc<RefCell<dyn Device>> =
                Rc::new(RefCell::new(Example_Device::new("dev0".to_string())));
            let rez_handler1 = sh.append_dev_to_a_room("room1", &dev);
            assert!(rez_handler1.is_ok()); // get handler
            let handler1 = rez_handler1.unwrap(); // unwrap handler
            assert!(handler1.get_state().is_ok_and(|rez| rez == false));
            assert!(handler1.change_state(true).is_ok());
            assert!(handler1.get_state().is_ok_and(|rez| rez == true));
            assert!(sh.change_dev_state_in_room("room1", "dev0", false).is_ok());
            assert!(handler1.get_state().is_ok_and(|rez| rez == false));
            println!(
                "--------------------------->{}",
                handler1.get_property_state().unwrap()
            );
        }
        #[test]
        fn dev_test_turn_on() {
            let mut sh = SmartHouse::new();
            assert!(sh.append_room("room1").is_ok());
            let dev: Rc<RefCell<dyn Device>> =
                Rc::new(RefCell::new(Example_Device::new("dev0".to_string())));
            let rez_handler1 = sh.append_dev_to_a_room("room1", &dev);
            assert!(rez_handler1.is_ok()); // get handler
            let handler1 = rez_handler1.unwrap(); // unwrap handler
            assert!(handler1.get_state().is_ok_and(|rez| rez == false));
            assert!(handler1.change_state(true).is_ok());
            assert!(handler1.get_state().is_ok_and(|rez| rez == true));
            assert!(sh.change_dev_state_in_room("room1", "dev0", false).is_ok());
            assert!(handler1.get_state().is_ok_and(|rez| rez == false));
        }
        #[test]
        fn test_get_devices_in_room() {
            let mut sh = SmartHouse::new();
            assert!(sh.append_room("room1").is_ok());
            let dev: Rc<RefCell<dyn Device>> =
                Rc::new(RefCell::new(Example_Device::new("dev0".to_string())));

            let dev1: Rc<RefCell<dyn Device>> =
                Rc::new(RefCell::new(Example_Device::new("dev1".to_string())));
            let _rez_handler = sh.append_dev_to_a_room("room1", &dev);
            let _rez_handler1 = sh.append_dev_to_a_room("room1", &dev1);
            let dev_list = sh.get_devices_in_room("room1");
            assert!(dev_list.is_ok());
            println!("------------------->devices: {}", dev_list.unwrap());
        }
        #[test]
        fn test_get_all_rooms() {
            let mut sh = SmartHouse::new();
            let room1 = "room1".to_string();
            let room2 = "room2".to_string();
            assert!(sh.append_room(&room1).is_ok());
            assert!(sh.append_room(&room2).is_ok());
            assert!(sh.get_all_rooms().is_some());
            println!("rooms--------> {}", sh.get_all_rooms().unwrap())
        }
        #[test]
        fn test_delete_room_ne() {
            let mut sh = SmartHouse::new();
            let room1 = "room1".to_string();
            let room2 = "room2".to_string();
            let room3 = "room3".to_string();
            assert!(sh.append_room(&room1).is_ok());
            assert!(sh.append_room(&room2).is_ok());
            assert!(sh.append_room(&room3).is_ok());
            assert!(sh.delete_room("room4").is_err())
        }
        #[test]
        fn test_delete_room() {
            let mut sh = SmartHouse::new();
            let room1 = "room1".to_string();
            let room2 = "room2".to_string();
            let room3 = "room3".to_string();
            assert!(sh.append_room(&room1).is_ok());
            assert!(sh.append_room(&room2).is_ok());
            assert!(sh.append_room(&room3).is_ok());
            assert!(sh.delete_room("room3").is_ok_and(|x| x == (3, 2)));
        }
        #[test]
        fn test_delete_dev() {
            let mut sh = SmartHouse::new();
            let room1 = "room1".to_string();
            let room2 = "room2".to_string();
            assert!(sh.append_room(&room1).is_ok());
            assert!(sh.append_room(&room2).is_ok());

            let dev0: Rc<RefCell<dyn Device>> =
                Rc::new(RefCell::new(Example_Device::new("dev0".to_string())));
            let dev1: Rc<RefCell<dyn Device>> =
                Rc::new(RefCell::new(Example_Device::new("dev1".to_string())));

            let _dev0_handler = sh.append_dev_to_a_room("room1", &dev0);
            let _dev1_handler = sh.append_dev_to_a_room("room1", &dev1);
            assert!(sh
                .get_device_number_in_room("room1")
                .is_some_and(|n| n == 2));
            assert!(sh.delete_device(&room1, "dev1").is_ok());
            assert!(sh
                .get_device_number_in_room("room1")
                .is_some_and(|n| n == 1));
            println!("{}", sh.get_dev_report(&dev0).unwrap());
        }
        #[test]
        fn test_report_dev() {
            let mut sh = SmartHouse::new();
            let room1 = "room1".to_string();
            let room2 = "room2".to_string();
            assert!(sh.append_room(&room1).is_ok());
            assert!(sh.append_room(&room2).is_ok());

            let dev0: Rc<RefCell<dyn Device>> =
                Rc::new(RefCell::new(Example_Device::new("dev0".to_string())));
            let dev1: Rc<RefCell<dyn Device>> =
                Rc::new(RefCell::new(Example_Device::new("dev1".to_string())));

            let _dev0_handler = sh.append_dev_to_a_room("room1", &dev0);
            let _dev1_handler = sh.append_dev_to_a_room("room1", &dev1);

            let dev3: Rc<RefCell<dyn Device>> =
                Rc::new(RefCell::new(Example_Device::new("dev1".to_string())));

            let _dev3_handler = sh.append_dev_to_a_room("room2", &dev3);
            assert!(sh
                .get_device_number_in_room("room1")
                .is_some_and(|n| n == 2));
            assert!(sh
                .get_device_number_in_room("room2")
                .is_some_and(|n| n == 1));
            println!("REPORT IS {}", sh.get_dev_report(&dev1).unwrap());
        }
    }
}
