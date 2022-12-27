#![feature(associated_type_defaults)]
#![feature(is_some_and)]

use std::cell::RefCell;
use std::process::Output;
use std::rc::Rc;
use std::rc::Weak;

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
    #[error("This device is not exists in the room {0}")]
    DeviceInRoomNotExists(String),
    #[error("Cannot change state")]
    ErrorSettingState,
    #[error("Cannot get state")]
    ErrorGettingState,
}

struct SmartHouse {
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
    fn append_room(&mut self, a_room: &str) -> Result<(), ErrorC> {
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

    fn test_whether_room_exists(&self, a_room: &str) -> Option<usize> {
        if self.rooms.iter().any(|x| x.get_room_name() == a_room) {
            self.rooms.iter().position(|x| x.get_room_name() == a_room)
        } else {
            None
        }
    }

    fn append_dev_to_a_room(
        &mut self,
        a_room: &str,
        a_device: &Rc<RefCell<dyn Device>>,
    ) -> Result<Device_Handler, ErrorC> {
        if let Some(room_pos) = self.test_whether_room_exists(a_room) {
            let x = self.rooms.get(room_pos).unwrap().add_device(a_device);
            Ok(Device_Handler::new(a_device))
        } else {
            Err(ErrorC::RoomNotExists(a_room.to_string()))
        }
    }
    fn change_dev_state_in_room(
        &mut self,
        a_room: &str,
        dev_name: &str,
        state: bool,
    ) -> Result<(()), ErrorC> {
        if let Some(room_pos) = self.test_whether_room_exists(a_room) {
            if let Some(dev_pos) = self.rooms.get(room_pos).unwrap().find_dev_name(dev_name) {
                // dev
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
    fn add_device(&self, some_dev: &Rc<RefCell<dyn Device>>);
    fn find_dev_name(&self, name: &str) -> Option<usize>;
    fn change_dev_state(&self, state: bool, name: &str);
}

struct Room_Generic {
    name: String,
    devices: Box<RefCell<Vec<Rc<RefCell<dyn Device>>>>>,
}

impl RoomObj for Room_Generic {
    fn get_room_name(&self) -> &str {
        self.name.as_str()
    }
    fn add_device(&self, some_dev: &Rc<RefCell<dyn Device>>) {
        self.devices.as_ref().borrow_mut().push(Rc::clone(some_dev));
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

trait Device {
    fn get_name(&self) -> String;
    fn set_state(&mut self, state: bool);
    fn get_state(&self) -> bool;
}

struct Device_Handler {
    dev: Weak<RefCell<dyn Device>>,
}

impl Device_Handler {
    fn new(dev: &Rc<RefCell<dyn Device>>) -> Self {
        Self {
            dev: Rc::downgrade(dev),
        }
    }
    fn change_state(&self, new_state: bool) -> Result<(), ErrorC> {
        if let Some(rez) = self.dev.upgrade() {
            rez.borrow_mut().set_state(new_state);
            Ok(())
        } else {
            Err(ErrorC::ErrorSettingState)
        }
    }
    fn get_state(&self) -> Result<bool, ErrorC> {
        if let Some(rez) = self.dev.upgrade() {
            Ok(rez.borrow().get_state())
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
}

struct Example_Device {
    name: String,
    state: bool,
}

impl Example_Device {
    fn new(name: String) -> Self {
        Self { name, state: false }
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
}
