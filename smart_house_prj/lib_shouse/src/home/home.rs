#![feature(generic_associated_types)]
pub mod smart_house {
    use std::rc::Rc;

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

    }

    pub fn run_me() {
        println!("run me!");
    }

    struct Stol {}
    struct Shulp {}

    trait Device {
        fn get_info(&self) -> Result<String, ErrorC>;
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
        fn create_new_home() -> Self {
            Shouse {  
                rooms: vec![Room::new_room("default_room")],
            }  
        }  


        fn test_whether_room_exits(&self,a_room: Rc<dyn Roomz>) -> bool{
            self.rooms.iter().any(|r| r.get_name() == a_room.get_name())
        } 

       pub fn append_room(&mut self, room: Rc<dyn Roomz>) -> Result<(), ErrorC> { // куда
                                                                                          // девается
                                                                                          // и где
                                                                                          // будет
                                                                                          // хранится?
            if ! self.rooms.iter().any(|r| r.get_name() == room.get_name()) {
                self.rooms.push(Rc::clone(&room)); // move from stack to heap
                Ok(())
            } else {  
                Err(ErrorC::RoomExists(room.get_name().to_string()))
            }    
        }    
    }    
    #[derive(Clone)]
    struct Room {    
        name: String,  
    }    
    impl Room {   
        fn new_room(name: &str) -> Rc<Room> {
            Rc::new(Room{ 
                name: name.to_owned(),
            })
        }
    }

    trait Roomz {
        fn get_name(&self) -> &str;
        fn get_devices(&self) -> Vec<&str> {
            unimplemented!();
        }
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
            sh.append_room(Room::new_room("room1")).unwrap();
            sh.append_room(Room::new_room("room1")).unwrap();// should panic
        }
        #[test]
        fn test_room_testing() {
            let mut sh = Shouse::create_new_home();
            let some_room = Room::new_room("test_room1");
            sh.append_room(some_room);
            sh.test_whether_room_exits(some_room);
        }


    }
}
