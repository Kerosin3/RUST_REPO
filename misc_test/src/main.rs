
/// the is an doc example
///
/// # Examples
///
///```
///
/// let arg1 = 10_i32;
/// let arg2 = 10_i32;
/// let answer = testrust::add_two_numbers(arg1,arg2);
/// assert_eq!(20_i32,answer);
///
///```

fn main() {
    println!("Hello, world!");
    let mut arr0: [i32; 20] = [0; 20];
    for (index, _item) in arr0.iter_mut().enumerate() {
        *_item += index as i32;
        println!("index is {} value is {}", index, _item);
    }
    multiply_array(&mut arr0);
    for _item in arr0.iter() {
        println!("updated value is {}", _item);
    }
    println!("result is {}", analyze_an_array(&arr0));
    multiply_by_2(&mut arr0);
    print_array(& arr0);
    let mut vec0:Vec<u32> = vec![];
    for i in 0..15  {
        vec0.push(i as u32 ) ;
    }
    println!("capacity is {}",vec0.capacity());
    println!("len is {}",vec0.len());

    for i in vec0.iter_mut() {
        *i = 666_u32;
    }
    let x = get_value_from_vector(&mut vec0);
    println!("result getiing value is {}",x);

}


fn get_value_from_vector(to_print:&mut Vec<u32> )->bool{
    let mut ii:usize = 0;
    println!("vec len is {}, ii is {}",to_print.len(),ii);
    let _answ = loop {
        let elem = to_print.pop();
        match elem {
            Some(success) => println!("value matching is {}",success),
            None       => break false,
        }
        ii+=1;
    };
    println!("vec len is {}, ii is {}",to_print.len(),ii);
    if ii < to_print.len()  {
        false
    } else {
        true
    }
}

#[warn(dead_code)]
pub fn add_two_numbers(arg1: i32, arg2: i32) -> i32 {
    arg1 + arg2
}

fn multiply_array(arg1: &mut [i32]) {
    for (index, _item) in arg1.iter_mut().enumerate() {
        *_item *= index as i32;
    }
}
fn analyze_an_array(arr0: &[i32]) -> bool {
    let mut i: usize = 0;
    let answ = loop {
        if i == arr0.len() - 1  { // break if end of array
            break false;
        } else if arr0[i] >= 256_i32 {
            break true;
        }
        i += 1;
    };
    println!("last value is {}", arr0[i]);
    answ // return answer
}
fn print_array(arr_in: & [i32]){
    for item in arr_in.iter(){
        println!("value is {}",item);
    }
}
fn multiply_by_2(arr0:&mut [i32]){
    for item in arr0.iter_mut()  {
        *item <<= 1; 
    }
}

#[test]
fn test_add_two_numbers() {
    let (a, b) = (150_i32, 200_i32);
    assert_eq!(add_two_numbers(a, b), 350_i32);
    let (c, d) = (150_i32, -200_i32);
    assert_eq!(add_two_numbers(c, d), -50_i32);
}

#[test]
fn test_analysis(){
    const ARR_SIZE:usize = 21;
    let mut arr: [i32;ARR_SIZE] = [0;ARR_SIZE];
    for (index,_val) in arr.iter_mut().enumerate()  {
        *_val = index as i32;
        *_val *= index as i32;
        println!("value is {}",_val);
    }
    println!("initialization done!");
    let rez:bool = analyze_an_array(&arr);
    assert_eq!(rez,true);
    //assert_eq!(true,true);
}
