pub mod smart_house {
    use std::io::{Error, ErrorKind};
    use std::{fmt, vec};
    pub fn home_fn() {
        println!("home!");
    }

    struct ARoom {
        rname: String,
        dev: Vec<String>,
    }

    pub trait DeviceInfoProvider {
        // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
        fn provide_info(&self) -> Vec<&str>;
    }

    impl ARoom {
        fn new(room_name: &str) -> Self {
            Self {
                rname: room_name.to_string(),
                dev: Default::default(),
            }
        }
        fn extract_room_name(&self) -> String {
            self.rname.clone()
        }

        /// get devices from the room
        ///
        fn get_devices(&self) -> Vec<String> {
            self.dev.clone()
        }

        /// append a device to room, should be used after test_if_dev_exists()
        ///
        /// # Arguments
        ///
        /// * `dev` - &str to add
        ///
        /// # Examples
        ///
        fn append_a_device(&mut self, dev: &str) {
            self.dev.push(dev.to_string());
        }
        fn test_if_dev_exists(&self, dev: &str) -> bool {
            self.dev.iter().any(|x| x == dev) // ворочает строками
        }
    }

    type RoomsList = Vec<ARoom>;

    #[derive(Default)]
    pub struct SmartHouse {
        name: String,
        rooms: RoomsList,
    }

    impl SmartHouse {
        //create new house instance
        pub fn new() -> Self {
            Self {
                name: Default::default(),
                rooms: Default::default(),
            }
        }
        //assign house name
        pub fn assign_name(&mut self, name: &str) {
            self.name = name.to_string();
        }

        pub fn append_room(&mut self, room: &str) -> Result<(), Error> {
            if !self.check_whether_room_exists(room).is_some() {
                // not exists
                let room = ARoom::new(room); // create new room
                self.rooms.push(room);
                Ok(())
            } else {
                Err(Error::from(ErrorKind::AlreadyExists))
            }
        }
        fn check_whether_room_exists(&self, room: &str) -> Option<bool> {
            if self.rooms.iter().any(|x| x.rname == room) {
                Some(true)
            } else {
                None
            }
        }
        pub fn append_a_device(&mut self, room_name: &str, dev: &str) -> Result<(), Error> {
            if self.check_whether_room_exists(room_name).is_none() {
                // not exists
                Err(Error::from(ErrorKind::NotFound))
            } else {
                // exists
                let room_pos = self
                    .rooms
                    .iter()
                    .position(|x| x.rname == room_name)
                    .unwrap(); //get room position
                let spec_room = self.rooms.get_mut(room_pos).unwrap();
                if spec_room.test_if_dev_exists(dev) {
                    // device exists
                    Err(Error::from(ErrorKind::AlreadyExists))
                } else {
                    spec_room.dev.push(dev.to_string());
                    Ok(())
                }
            }
        }
        /// get all room names in the house
        ///
        /// # Examples
        ///
        /// ```
        /// ```
        fn get_rooms(&self) -> Vec<String> {
            let mut room_list: Vec<String> = vec![];
            for i in self.rooms.iter() {
                room_list.push(i.extract_room_name());
            }
            room_list
        }
        /// get room object
        ///
        /// # Arguments
        ///
        /// * `room` - name of a room, shoul be used to process vec after get_rooms()
        ///
        /// # Examples
        ///
        /// ```
        /// ```
        fn get_room(&self, room: &str) -> &ARoom {
            let room_pos = self.rooms.iter().position(|x| x.rname == room).unwrap(); //get room position
            self.rooms.get(room_pos).unwrap()
        }
        /// returns a vec of all devices in a room
        ///
        /// # Arguments
        ///
        /// * `room` - room name
        ///
        /// # Errors
        ///
        /// not found in case romm has not been found
        ///
        /// # Examples
        ///
        /// ```
        /// ```
        fn devices(&self, room: &str) -> Result<Vec<String>, Error> {
            if self.check_whether_room_exists(room).is_none() {
                // no such room
                Err(Error::from(ErrorKind::NotFound))
            } else {
                // go and find a device
                let room_pos = self.rooms.iter().position(|x| x.rname == room).unwrap(); //get room position
                Ok(self.rooms.get(room_pos).unwrap().dev.clone()) //return copy of devices
            }
        }

        /// format to output all rooms in the house
        ///
        /// # Examples
        ///
        /// ```
        /// ```
        pub fn get_all_rooms(&self) -> String {
            let mut string_out = String::new();
            fmt::write(
                &mut string_out,
                format_args!("Home:{}, contains rooms: ", self.name),
            )
            .expect("error writing string");
            for room in self.get_rooms().iter() {
                fmt::write(&mut string_out, format_args!("{}, ", room))
                    .expect("error writing string");
            }
            string_out
        }

        /// format to output all devices in the house
        ///
        /// # Examples
        ///
        /// ```
        /// ```
        pub fn get_all_devices(&self) -> String {
            let mut string_out = String::new();
            let room_list = self.rooms.iter();
            for room in room_list {
                //get all devices
                fmt::write(&mut string_out, format_args!(" room:{}:", room.rname))
                    .expect("error writing string");
                for dev in room.get_devices() {
                    fmt::write(&mut string_out, format_args!(" {},", dev))
                        .expect("error writing string");
                }
            }
            string_out
        }
        pub fn create_report<T: DeviceInfoProvider>(&self, g_room: &T) -> Result<String, Error> {
            let devices = g_room.provide_info(); //get room and devices
            let mut out_devices_info = String::new(); //write out to this str
            let mut flag = 0_i32;
            let rooms = self.get_rooms(); //get list of rooms
            for _a_room in rooms.iter() {
                // iterate over rooms
                for sup_dev in devices.iter() {
                    // iterate over supplied devices
                    let contains = self.get_room(_a_room).dev.contains(&sup_dev.to_string());
                    if contains {
                        flag += 1;
                        fmt::write(
                            &mut out_devices_info,
                            format_args!(" room:{}, device:{},", _a_room, &sup_dev),
                        )
                        .expect("error while writing to buffer");
                    }
                }
            }
            if flag > 0 {
                Ok(format!(
                    "Report for house {}: devices list:{}",
                    self.name, &out_devices_info
                ))
            } else {
                Err(Error::new(
                    ErrorKind::Other,
                    format!(
                        "[Error] Report for house {}: no such device(s) [{:?}] has been found",
                        self.name, devices
                    ),
                ))
            }
        }

        // for testing
        fn create_report_x(&self, devices: Vec<&str>) -> Result<String, Error> {
            //let devices = g_room.provide_info();//get room and devices
            let mut out_devices_info = String::new(); //write out to this str
            let mut flag = 0_i32;
            let rooms = self.get_rooms(); //get list of rooms
            for _a_room in rooms.iter() {
                // iterate over rooms
                for sup_dev in devices.iter() {
                    // iterate over supplied devices
                    let contains = self.get_room(_a_room).dev.contains(&sup_dev.to_string());
                    if contains {
                        flag += 1;
                        fmt::write(
                            &mut out_devices_info,
                            format_args!(" room:{} {},", _a_room, &sup_dev),
                        )
                        .expect("error while writing to buffer");
                    }
                }
            }
            if flag > 0 {
                Ok(format!("Report: devices list:{}", &out_devices_info))
            } else {
                Ok(format!(
                    "Report: no such device(s) [{:?}] has been found",
                    devices
                ))
            }
        }
    }

    #[cfg(test)]
    mod test_report {
        use super::*;
        #[test]
        fn test_get_report_v2() {
            // get report proto func
            let mut s1 = SmartHouse::new();
            let room1 = "room1".to_string();
            s1.append_room(&room1).unwrap();
            let dev0 = "device0".to_string(); //device
            let dev1 = "device1".to_string(); //device
            let dev2 = "device2".to_string(); //device
            s1.append_a_device(&room1, &dev0).unwrap(); // append to room1
            s1.append_a_device(&room1, &dev1).unwrap(); // append to room1
            s1.append_a_device(&room1, &dev2).unwrap(); // append to room1
            let dev_list = vec!["device0", "device1"];

            let out = s1.create_report_x(dev_list);
            assert_eq!(out.is_ok(), true);
            println!("REPORT IS:::::::::::::::::::::::{}", out.unwrap());
        }
    }

    #[cfg(test)]
    mod test_devices {
        use super::*;

        #[test]
        fn append_a_device() {
            // room exists
            let mut s1 = SmartHouse::new();
            let v1 = "room1".to_string();
            s1.append_room(&v1).unwrap();
            let dev0 = "device0".to_string(); //device
            s1.append_a_device(&v1, &dev0).unwrap();
        }

        #[test]
        #[should_panic]
        fn append_a_device_non_existing_room() {
            // room not exists
            let mut s1 = SmartHouse::new();
            let v1 = "room1".to_string();
            s1.append_room(&v1).unwrap();
            let ner = "roomX".to_string();
            let dev0 = "device0".to_string(); //device
            s1.append_a_device(&ner, &dev0).unwrap();
        }

        #[test]
        #[should_panic]
        fn append_a_device_exists() {
            // device exists
            let mut s1 = SmartHouse::new();
            let v1 = "room1".to_string();
            s1.append_room(&v1).unwrap();
            let dev0 = "device0".to_string(); //device
            s1.append_a_device(&v1, &dev0).unwrap();
            s1.append_a_device(&v1, &dev0).unwrap();
        }

        #[test]
        fn append_a_devices() {
            // devices not exists
            let mut s1 = SmartHouse::new();
            let room1 = "room1".to_string();
            let room2 = "room2".to_string();
            s1.append_room(&room1).unwrap();
            s1.append_room(&room2).unwrap();
            let dev0 = "device0".to_string(); //device
            s1.append_a_device(&room1, &dev0).unwrap(); // append to room1
            s1.append_a_device(&room2, &dev0).unwrap(); // append to room2
        }

        #[test]
        fn get_devices_list() {
            //  test devices list
            let mut s1 = SmartHouse::new();
            let room1 = "room1".to_string();
            s1.append_room(&room1).unwrap();
            let dev0 = "device0".to_string(); //device
            let dev1 = "device1".to_string(); //device
            let dev2 = "device2".to_string(); //device
            s1.append_a_device(&room1, &dev0).unwrap(); // append to room1
            s1.append_a_device(&room1, &dev1).unwrap(); // append to room1
            s1.append_a_device(&room1, &dev2).unwrap(); // append to room1
            let dev_list = s1.devices(&room1).unwrap();
            assert_eq!(dev_list.contains(&dev0), true);
            assert_eq!(dev_list.contains(&dev1), true);
            assert_eq!(dev_list.contains(&dev2), true);
        }
    }

    #[cfg(test)]
    mod test_struct_room {
        use super::*;

        #[test]
        fn test_struct_creation() {
            // pushes nothing
            let mut s1 = SmartHouse::new();
            assert_eq!(s1.get_rooms(), Vec::<&str>::new());
        }

        #[test]
        fn test_room_append_n_e() {
            // append not exists
            let mut s1 = SmartHouse::new();
            let v1 = "room1".to_string();
            let v2 = "room2".to_string();
            s1.append_room(&v1).unwrap();
            assert_eq!(s1.check_whether_room_exists(&v1).is_some(), true);
        }

        #[test]
        #[should_panic]
        fn check_n_exists() {
            let mut s1 = SmartHouse::new();
            let v1 = "room1".to_string();
            let v2 = "room2".to_string();
            s1.append_room(&v1).unwrap();
            assert_eq!(s1.check_whether_room_exists(&v1).is_some(), false);
        }

        #[test]
        #[should_panic]
        fn test_room_append_exists() {
            // append exists
            let mut s1 = SmartHouse::new();
            let v1 = "room1".to_string();
            let v2 = "room2".to_string();
            s1.append_room(&v1).unwrap();
            s1.append_room(&v1).unwrap();
            // assert_eq!(s1.check_whether_room_exists(&v1),true);
        }

        #[test]
        fn test_get_rooms() {
            let mut s1 = SmartHouse::new();
            let v1 = "room1".to_string();
            let v2 = "room2".to_string();
            s1.append_room(&v1).unwrap();
            s1.append_room(&v2).unwrap();
            let room_list = s1.get_rooms();
            assert_eq!(room_list.contains(&v1), true);
            assert_eq!(room_list.contains(&v2), true);
        }

        #[test]
        #[should_panic]
        fn test_get_rooms_ne() {
            let mut s1 = SmartHouse::new();
            let v1 = "room1".to_string();
            let v2 = "room2".to_string();
            let v3 = "roomX".to_string();
            s1.append_room(&v1).unwrap();
            s1.append_room(&v2).unwrap();
            let room_list = s1.get_rooms();
            assert_eq!(room_list.contains(&v1), true);
            assert_eq!(room_list.contains(&v3), true);
        }
    }
}
