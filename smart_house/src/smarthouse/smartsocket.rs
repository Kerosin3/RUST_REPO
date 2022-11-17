pub struct SmartSocket;

impl SmartSocket {
    /// creates a net socket instance
    ///
    /// # Examples
    ///
    pub fn new() -> SmartSocket {
        SmartSocket
    }
    /// power on the socket
    ///

    pub fn power_on(&mut self) {
        todo!();
    }
    /// power off the socket
    ///
    pub fn power_off(&mut self) {
        todo!()
    }
    /// get current consumed power
    ///
    pub fn get_cur_consum_power(&self) {
        todo!()
    }
    pub fn test_socket(&self) {
        println!("test smart socket");
    }
}
