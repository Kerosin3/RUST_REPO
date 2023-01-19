mod smarthouse;

#[test]
fn test_smart_socket() {
    let ssocket = smarthouse::smartsocket::SmartSocket::new();
    ssocket.test_socket();
}
#[test]
fn test_smart_termometer() {
    let smart_termometer = smarthouse::smarttermometer::SmartTermometer::new();
    smart_termometer.test_smart_temp();
}
