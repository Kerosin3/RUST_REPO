pub mod server_data {
    use lib_shouse::home::home::home::Device;
    pub static mut SMART_SOCKET_SERIAL: usize = 0;
    pub struct SmartSocket {
        name: String,
        state: bool,
        consum_power: f32,
    }

    impl SmartSocket {
        pub fn new() -> Self {
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
        pub fn set_cons_power(&mut self, temp: f32) {
            self.consum_power = temp;
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
}
