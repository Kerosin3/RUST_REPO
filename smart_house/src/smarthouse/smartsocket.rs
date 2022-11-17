#[allow(dead_code)]
pub struct SmartSocket;

impl SmartSocket {
    /// creates a net socket instance
    ///
    /// # Examples
    ///
    #[allow(dead_code)]
    pub fn new() -> SmartSocket {
        SmartSocket
    }
    /// power on the socket
    ///

    #[allow(dead_code)]
    pub fn power_on(&mut self) {
        todo!();
    }
    /// power off the socket
    ///
    #[allow(dead_code)]
    pub fn power_off(&mut self) {
        todo!()
    }
    #[allow(dead_code)]
    /// get current consumed power
    ///
    #[allow(dead_code)]
    pub fn get_cur_consum_power(&self) {
        todo!()
    }
    #[allow(dead_code)]
    pub fn test_socket(&self) {
        println!("test smart socket");
    }
}
