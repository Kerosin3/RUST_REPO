pub mod house_creation {
    use crate::home::home::smart_house::SmartHouse; // import public api
    pub fn create_home_instance(h_name: &str) -> SmartHouse {
        let mut s1 = SmartHouse::new();
        s1.assign_name(h_name);
        let room1 = "room1".to_string();
        s1.append_room(&room1).unwrap(); //add room1
        let room2 = "room2".to_string();
        s1.append_room(&room2).unwrap(); // add room2

        let dev0 = "device0".to_string(); //device
        let dev1 = "device1".to_string(); //device
        let dev2 = "device2".to_string(); //device
        s1.append_a_device(&room1, &dev0).unwrap(); // append to room1
        s1.append_a_device(&room1, &dev1).unwrap(); // append to room1
        s1.append_a_device(&room1, &dev2).unwrap(); // append to room1
        s1.append_a_device(&room2, &dev0).unwrap(); // append to room2
        s1.append_a_device(&room2, &dev1).unwrap(); // append to room2
        s1.append_a_device(&room2, &dev2).unwrap(); // append to room2
        s1
    }
}
