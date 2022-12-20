#![feature(generic_associated_types)]
pub mod smart_house {
    use std::{rc::Rc, any::Any};
    use std::cell::RefCell;

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
                rooms: vec![] // empty vector
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
        pub fn append_dev_to_a_room(&mut self, a_room: &Rc<dyn Roomz>) -> Result<(), ErrorC> {
            if let Some(r_pos) = self.test_whether_room_exits(a_room) {
                let a = self.rooms.iter().nth(r_pos).unwrap().get_name();
                println!("-------------->name is {}",a);
                Ok(())
            } else {
                Err(ErrorC::RoomNotExists(a_room.get_name().to_string()))
            }
        }
    }
    struct Room {
        name: String,
        //devices: Rc<Vec<dyn Device>>,
        devices: Vec<Rc<dyn Device>>,
    }
    impl Room {
        pub fn new_room(name: &str) -> Rc<dyn Roomz> {
            Rc::new(Room {
                name: name.to_owned(),
                devices: vec![],
            })
        }
        pub fn add_a_device(&mut self,a_dev: &Rc<dyn Device>) -> Result<(),ErrorC>{
            if self.test_whether_device_exits_int_the_room(a_dev).is_none() {
                self.devices.push(Rc::clone(a_dev)); // Rc copy
                Ok(())
            } else {
                Err(ErrorC::RoomExists(a_dev.get_dev_name().to_string()))
            }

        }
        fn test_whether_device_exits_int_the_room(&self,a_dev: &Rc<dyn Device>) -> Option<usize>{
            if self.devices.iter().any(|r| r.get_dev_name() == a_dev.get_dev_name()) {
                self.devices
                    .iter()
                    .position(|x| x.get_dev_name() == a_dev.get_dev_name()) // some with pos
            } else {
                None
            }
  
        }
    }

    struct Dev1(String);

    impl Device for Dev1 {
        fn get_dev_name(&self) -> &str {
            self.0.as_str()
        }
    }

    trait Device {
        fn get_dev_name(&self) -> &str;
        //        fn get_dev_info(&self) -> &str;
    }

    trait Roomz {
        fn get_name(&self) -> &str;
        fn get_devices(&self) -> Vec<&str> {
            unimplemented!();
        }
        fn get_state(&self) {
            unimplemented!();
        }
        //fn add_device(&mut self,a_dev: &Rc<dyn Device> ) -> Result<(),ErrorC>;
    }
    impl Roomz for Room {
        fn get_name(&self) -> &str {
            &self.name
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
        fn test_add_device() {
            let mut sh = Shouse::create_new_home();
            let some_room = Room::new_room("test_room1");
            assert!(sh.append_room(&some_room).is_ok());
            let some_dev = Dev1(String::from("device1"));
            assert!(sh.append_dev_to_a_room(&some_room).is_ok());

        }
    }
}
