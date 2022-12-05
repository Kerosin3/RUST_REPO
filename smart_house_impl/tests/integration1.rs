use mylib::home::home::smart_house::SmartHouse;
use mylib::misc::house_creation::house_creation::create_home_instance;
#[test]
fn integration_test_1() {
    let mut house = create_home_instance("some_home");
    println!("{}", house.get_all_rooms());
    let room1 = "room1".to_string();
    let room2 = "room2".to_string();
    assert_eq!(house.append_room(&room1).is_ok(), false); // try append existing room
}
#[test]
fn integration_test_2() {
    let mut house = create_home_instance("some_home");
    println!("{}", house.get_all_rooms());
    let room1 = "room1".to_string();
    let room2 = "room2".to_string();
    let roomNE = "roomNE".to_string();
    let dev0 = "device0".to_string(); //device
    assert_eq!(house.append_a_device(&roomNE, &dev0).is_ok(), false); // try append to non existing room
                                                                      // got error is ok
}
#[test]
fn integration_test_3() {
    // append an existing device to an existing rom
    let mut house = create_home_instance("some_home");
    println!("{}", house.get_all_rooms());
    let room1 = "room1".to_string();
    let room2 = "room2".to_string();
    let dev0 = "device0".to_string(); //device
    assert_eq!(house.append_a_device(&room1, &dev0).is_ok(), false); // try append to non existing room
                                                                     // got error is ok
}
