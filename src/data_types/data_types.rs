use std::io;


pub fn return_data() -> u32 {
    let guess:u32 = "42".parse().expect("Not a number");
    return guess;
}

pub fn tuples() -> (i32, f64, u8) {
    let tup: (i32, f64, u8) = (500, 5.3, 1);
    print!("This is a tuple {:?}", tup);
    return tup;
}

pub fn arrays() -> [&'static str; 12]{
    let months: [&str; 12] = [
        "Jan", "Feb", "Mar", "April", "May", "June",
        "July", "Aug", "Sept", "Oct", "Nov", "Dec"
    ];
    return months;

}

pub fn access_array() -> () {
    let a = [1,2,3,4,5];
    println!("\nEnter an array index from 0 - 4");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the elemtnat index {index} is: {element}");

}
