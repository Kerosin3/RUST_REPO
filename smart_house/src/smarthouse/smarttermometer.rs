pub struct SmartTermometer;

impl SmartTermometer {
    /// get new instance of termometer
    ///
    pub fn new() -> SmartTermometer {
        SmartTermometer
    }
    /// get current temperature from instance
    ///
    pub fn get_cur_temp(&self) {
        todo!();
    }
    pub fn test_smart_temp(&self) {
        println!("test smart termometer");
    }
}
