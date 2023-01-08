#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use lib_shouse::home::home::home::Device;
use std::sync::atomic::{AtomicUsize, Ordering};
static TERMOMETER_SERIAL: AtomicUsize = AtomicUsize::new(0);

pub struct Termometer {
    name: String,
    state: bool,
    temperature: f32,
}

impl Termometer {
    pub fn new() -> Self {
        let out = Self {
            name: [
                "termometer_",
                "#",
                TERMOMETER_SERIAL
                    .fetch_add(1, Ordering::SeqCst)
                    .to_string()
                    .as_str(), // complicated
            ]
            .concat(),
            state: false,
            temperature: 0.0,
        };
        out
    }
    fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
    }
}

impl Device for Termometer {
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
        format!("current temperature is {}", self.temperature)
    }
    fn set_property_info(&mut self, new_info: &dyn std::fmt::Display) {
        self.temperature = new_info.to_string().parse::<f32>().unwrap();
    }
}
