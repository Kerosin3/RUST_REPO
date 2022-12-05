use mylib::home::home::smart_house::SmartHouse;
use mylib::misc::house_creation::house_creation::create_home_instance;
#[test]
fn integration_test_2_1() {
    // append an existing device to an existing rom
    let mut house = create_home_instance("some_home");
    println!("{}", house.get_all_rooms());
    let room1 = "room1".to_string();
    let room2 = "room2".to_string();
    let devX = "deviceX".to_string(); //device
    assert_eq!(house.append_a_device(&room1, &devX).is_ok(), true); // OK
    // got error is ok
}