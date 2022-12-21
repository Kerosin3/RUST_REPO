#![feature(generic_associated_types)]
pub mod smart_house {
    use std::cell::RefCell;
    use std::cell::RefMut;
    use std::ops::Deref;
    use std::{any::Any, rc::Rc};

    #[derive(thiserror::Error, Debug, Clone)]
    pub enum ErrorC {
        #[error("Internal error.")]
        Internal(String),
        #[error("Not found.")]
        NotFound,
        #[error("Permission Denied.")]
        PermissionDenied,
        #[error("Invalid argument: {0}")]
        InvalidArgument(String),
        #[error("Such room is already exists {0}")]
        RoomExists(String),
        #[error("Such room is not exists {0}")]
        RoomNotExists(String),
        #[error("This device is already exists in the room {0}")]
        DeviceInRoomExists(String),
    }

    pub fn run_me() {
        println!("run me!");
    }

    struct Shouse {
        rooms: Vec<Rc<dyn Roomz>>,
        //        devices: Vec<Box<dyn Device>>,
    }

    impl Shouse {
        /// create default smart house with default room
        ///
        /// # Examples
        ///  
        pub fn create_new_home() -> Self {
            Shouse {
                rooms: vec![], // empty vector
            }
        }

        fn test_whether_room_exits(&self, a_room: &Rc<dyn Roomz>) -> Option<usize> {
            if self.rooms.iter().any(|r| r.get_name() == a_room.get_name()) {
                self.rooms
                    .iter()
                    .position(|x| x.get_name() == a_room.get_name()) // some with pos
            } else {
                None
            }
        }

        pub fn append_room(&mut self, room: &Rc<dyn Roomz>) -> Result<(), ErrorC> {
            // куда
            // девается
            // и где
            // будет
            // хранится?
            if self.test_whether_room_exits(room).is_none() {
                self.rooms.push(Rc::clone(room)); // Rc copy
                Ok(())
            } else {
                Err(ErrorC::RoomExists(room.get_name().to_string()))
            }
        }

        pub fn test_whether_dev_in_room_exists(
            &self,
            a_room: &Rc<dyn Roomz>,
            a_dev: &Rc<dyn Device>,
        ) -> Option<usize> {
            if let Some(r_pos) = self.test_whether_room_exits(a_room) {
                // room exists
                for (j, dev) in self
                    .rooms
                    .get(r_pos) // room
                    .unwrap()
                    .get_devices_ref()
                    .borrow()
                    .iter()
                    .enumerate()
                {
                    println!("->>device is {}", dev.get_dev_name());
                    if dev.get_dev_name() == a_dev.get_dev_name() {
                        return Some(j);
                    }
                }
                None
            } else {
                None
            }
        }
        pub fn print_dev_in_rooms(&self, a_room: &Rc<dyn Roomz>) {
            for i in self.rooms.get(0).unwrap().get_devices_ref().borrow().iter() {
                //       println!("device:{}", i.deref().get_dev_name()); dgb?
            }
        }
        pub fn append_dev_to_a_room(
            &mut self,
            a_room: &Rc<dyn Roomz>,
            a_device: &Rc<dyn Device>,
        ) -> Result<(), ErrorC> {
            if let Some(r_pos) = self.test_whether_room_exits(a_room) {
                // test is room exists
                if let Some(_) = self.test_whether_dev_in_room_exists(a_room, a_device) {
                    // test if
                    // dev is
                    // already
                    // added
                    return Err(ErrorC::DeviceInRoomExists(a_room.get_name().to_string()));
                } else {
                    // not exists in the room
                    self.rooms
                        .iter()
                        .nth(r_pos)
                        .unwrap()
                        .get_devices_ref()
                        .borrow_mut()
                        .push(Rc::clone(a_device)); // PUSH!!!
                    println!(
                        "-------------->APPENDED DEV:{} TO ROOM {} ",
                        a_device.get_dev_name(),
                        a_room.get_name()
                    );
                    Ok(())
                }
            } else {
                Err(ErrorC::RoomNotExists(a_room.get_name().to_string()))
            }
        }
    }
    struct Room {
        name: String,
        //devices: Rc<RefCell<Vec<Rc<Box<dyn Device>>>>>,
        devices: Rc<RefCell<Vec<Rc<dyn Device>>>>,
    }

    impl Room {
        pub fn new_room(name: &str) -> Rc<dyn Roomz> {
            Rc::new(Self {
                name: name.to_owned(),
                devices: Rc::new(RefCell::new(vec![])),
            })
        }
    }

    struct Dev1 {
        name: String,
        state: bool,
        serial: usize,
    }

    impl Dev1 {
        fn new(name: &str) -> Rc<dyn Device> {
            Rc::new(Self {
                name: (name.to_owned()),
                state: (true),
                serial: (1),
            })
        }
    }

    impl Device for Dev1 {
        fn get_dev_name(&self) -> &str {
            self.name.as_str()
        }
        fn get_dev_state(&self) -> bool {
            true
        }
    }

    trait Device {
        fn get_dev_name(&self) -> &str;
        fn get_dev_state(&self) -> bool;
        //        fn get_serial(&self) -> usize;
        //        fn get_dev_info(&self) -> &str;
    }

    trait Roomz {
        fn get_name(&self) -> &str;
        fn get_devices_ref(&self) -> Rc<RefCell<Vec<Rc<dyn Device>>>>;
        fn get_devices(&self) -> Vec<&str> {
            unimplemented!();
        }
        fn get_state(&self) {
            unimplemented!();
        }
        //fn get_room_store_to_append(&mut self) -> RefCell<Vec<Rc<dyn Device>>>;
        //fn get_room_store_to_append(&mut self) -> RefMut<Vec<Rc<Box<dyn Device>>>>;
        //fn add_device(&mut self,a_dev: &Rc<dyn Device> ) -> Result<(),ErrorC>;
    }
    impl Roomz for Room {
        fn get_name(&self) -> &str {
            &self.name
        }

        fn get_devices_ref(&self) -> Rc<RefCell<Vec<Rc<dyn Device>>>> {
            Rc::clone(&self.devices)
        }
    }

    #[cfg(test)]
    mod test_test {
        use super::*;
        #[test]
        #[should_panic]
        fn test_existing_append() {
            let mut sh = Shouse::create_new_home();
            let some_room = Room::new_room("test_room1");
            assert!(sh.append_room(&some_room).is_ok());
            assert!(sh.append_room(&some_room).is_ok()); // fail
        }
        #[test]
        fn test_room_testing() {
            let mut sh = Shouse::create_new_home();
            let some_room = Room::new_room("test_room1");
            assert!(sh.append_room(&some_room).is_ok());
            assert!(sh.test_whether_room_exits(&some_room).is_some()); // true when exists
                                                                       //let dev1 = Dev1(String::from("device1"));
        }
        #[test]
        #[should_panic]
        fn test_add_device_to_n_exists_room() {
            let mut sh = Shouse::create_new_home();
            let _some_room = Room::new_room("test_room1");
            let some_room2 = Room::new_room("test_room1");
            let dev = Dev1::new("Device1");
            assert!(sh.append_dev_to_a_room(&some_room2, &dev).is_ok());
        }
        #[test]
        fn test_add_device() {
            let mut sh = Shouse::create_new_home();
            let some_room = Room::new_room("test_room1");
            assert!(sh.append_room(&some_room).is_ok());
            let dev = Dev1::new("Device1");
            let dev1 = Dev1::new("Device2");
            assert!(sh.append_dev_to_a_room(&some_room, &dev).is_ok());
            assert!(sh.append_dev_to_a_room(&some_room, &dev1).is_ok());
        }
        #[test]
        fn test_test_dev_in_room() {
            let mut sh = Shouse::create_new_home();
            let some_room = Room::new_room("test_room1");
            assert!(sh.append_room(&some_room).is_ok());
            let dev = Dev1::new("Device1");
            let dev1 = Dev1::new("Device2");
            let dev2 = Dev1::new("Device3");
            assert!(sh.append_dev_to_a_room(&some_room, &dev).is_ok());
            assert!(sh.append_dev_to_a_room(&some_room, &dev1).is_ok());
            assert!(sh.append_dev_to_a_room(&some_room, &dev2).is_ok());
            assert!(sh
                .test_whether_dev_in_room_exists(&some_room, &dev2)
                .is_some());
        }
        #[test]
        fn test_test_dev_in_room_n_exists() {
            let mut sh = Shouse::create_new_home();
            let some_room = Room::new_room("test_room1");
            assert!(sh.append_room(&some_room).is_ok());
            let dev = Dev1::new("Device1");
            let dev1 = Dev1::new("Device2");
            let dev2 = Dev1::new("Device3");
            assert!(sh
                .test_whether_dev_in_room_exists(&some_room, &dev2)
                .is_none());
        }
        #[test]
        #[should_panic]
        fn test_test_dev_append_dev_exists() {
            let mut sh = Shouse::create_new_home();
            let some_room = Room::new_room("test_room1");
            assert!(sh.append_room(&some_room).is_ok());
            let dev = Dev1::new("Device1");
            assert!(sh.append_dev_to_a_room(&some_room, &dev).is_ok());
            assert!(sh.append_dev_to_a_room(&some_room, &dev).is_ok());
        }
        #[test]
        fn test_test_dev_append_dev() {
            let mut sh = Shouse::create_new_home();
            let some_room = Room::new_room("test_room1");
            assert!(sh.append_room(&some_room).is_ok());
            let dev = Dev1::new("Device1");
            let dev1 = Dev1::new("Device2");
            assert!(sh.append_dev_to_a_room(&some_room, &dev).is_ok());
            assert!(sh.append_dev_to_a_room(&some_room, &dev1).is_ok());
            sh.print_dev_in_rooms(&some_room);
        }
    }
}
