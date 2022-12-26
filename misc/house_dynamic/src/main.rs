#![feature(associated_type_defaults)]
use std::process::Output;
use std::rc::Rc;
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
        #[error("This device is already exists in the room {0}")]
        DeviceInRoomExists(String),
        #[error("This device is not exists in the room {0}")]
        DeviceInRoomNotExists(String),
    }

struct SmartHouse{
    rooms: Vec<Rc<dyn RoomObj>>,
}


impl SmartHouse{
    pub fn new()-> Self {
        Self {
            rooms: vec![]
        }
    }
    fn create_room(room_name:&str) -> Rc<dyn RoomObj> {
        // check whther exists
        Rc::new( Room_Generic{
            name: room_name.to_string(),
            devices: Box::new(RefCell::new(vec![]))
        } )
    }
    fn append_room(&mut self, a_room: &str) -> Result<(),ErrorC> {
        if self.test_whether_room_exists(a_room).is_some(){
            Err(ErrorC::RoomExists(a_room.to_string()))
        } else {
        self.rooms.push(Rc::new(Room_Generic {
            name: a_room.to_string(),
            devices: Box::new(RefCell::new(vec![])) //devices store
        }));
                Ok(())
        }
    }
    
    fn test_whether_room_exists(&self,a_room: &str) -> Option<usize> {
        if self.rooms.iter().any(|x| x.get_room_name() == a_room )
        {
            self.rooms.iter().position(|x| x.get_room_name() == a_room )
        } else {
            None
        }
    }

    fn append_dev_to_a_room(&mut self, a_room: &str, a_device: &Rc<dyn Device>)-> Result<(),ErrorC>{
        if let Some(room_pos) = self.test_whether_room_exists(a_room){
            let x =self.rooms.get(room_pos).unwrap().add_device( Rc::clone(a_device)  );
            Ok(())
        } else {
            Err(ErrorC::RoomNotExists(a_room.to_string()))
        }
    }
}


trait RoomObj{
    fn get_room_name(&self) -> &str;
    fn add_device(&self,some_dev: Rc<dyn Device>);
    //fn get_all_devices(&self);
}

struct Room_Generic{
    name: String,
    devices: Box<RefCell<Vec<Rc<dyn Device>>>>
}

impl RoomObj for Room_Generic{
    fn get_room_name(&self) ->&str{
        self.name.as_str()
    }
    fn add_device(&self,some_dev: Rc<dyn Device>){
        self.devices.as_ref().borrow_mut().push(Rc::clone(&some_dev));
    }
}

trait Device {
    fn get_name(&self)-> String;
    fn set_state(&mut self,state: bool) ;
    fn get_state(&self) -> bool;
}

struct Device_Handler{
}


struct Example_Device{
    name: String,
    state: bool,
}

impl Example_Device{
    fn new(name: String) -> Self{
        Self{name,state: false}
    }
}

impl Device for Example_Device {
    fn get_name(&self) -> String{
        self.name.clone()
    }
    fn get_state(&self) -> bool{
        self.state 
        }
    fn set_state(&mut self,state: bool){
        self.state = state;
    }
}


fn main() {
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn create_and_append_room(){
        let mut sh = SmartHouse::new();
        assert!(sh.append_room("room1").is_ok());
    }
    #[test]
    fn create_and_append_room_exists(){
        let mut sh = SmartHouse::new();
        assert!(sh.append_room("room1").is_ok());
        assert!(sh.append_room("room1").is_err());
    }
    #[test]
    fn dev_creation(){
        let mut sh = SmartHouse::new();
        assert!(sh.append_room("room1").is_ok());
        let dev: Rc <dyn Device>  = Rc::new(Example_Device::new("dev0".to_string()));        
        //let mut g_dev: Rc<dyn Device> = Rc::new(Generic_Dvice(String::from("some_device")));
        //let mut g_dev: Rc<dyn Device> = Rc::new(Generic_Device::new(dev, name, state))
        assert!(sh.append_dev_to_a_room("room1",&dev).is_ok());
    }
    #[test]
    fn add_to_n_exists_room(){
        let mut sh = SmartHouse::new();
        assert!(sh.append_room("room2").is_ok());
        let dev: Rc <dyn Device>  = Rc::new(Example_Device::new("dev0".to_string()));        
       // let mut g_dev: Rc<dyn Device> = Rc::new(Generic_Dvice(String::from("some_device")));
        assert!(sh.append_dev_to_a_room("room1",&dev).is_err());
    }
    #[test]
    fn dev_creation_and_change(){
        let mut sh = SmartHouse::new();
        assert!(sh.append_room("room1").is_ok());
        let dev: Rc<RefCell<dyn Device>> = Rc::new(RefCell::new(  Example_Device::new("dev0".to_string())));    
        // как процесс оборачивания завернуть для юзера? все эти обертки - лишнее,
        // юзер создал девайс и должен через этот объект умтеь им управлять, в свою очередь его
        // должн уметь втянуть "умный дом" с помощью  функции append_dev_to_a_room, и в случае чего
        // и внутри сам поменять состояние объекта девайс
        //  то же самое ниже, юзер не должен приставлять борровы, как обернуть все это?
        //  возвращать юзеру какой то хандлер или что? 
//      assert!(sh.append_dev_to_a_room("room1",dev.borrow()).is_ok());     
        assert_eq!(dev.borrow().get_state(),false);
        dev.borrow_mut().set_state(true);
        assert_eq!(dev.borrow().get_state(),true);
    }


}
