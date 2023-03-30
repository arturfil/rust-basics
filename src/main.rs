use ownership::ownership_ie;

use crate::{functions::{secondary_function, print_x, calc_area}, control_flow::can_drink};

mod data_types;
mod functions;
mod control_flow;
mod ownership;


fn main() {
    // variables: mutable and imutable
    // let name: &str = "arturo filio";
    // println!("hello, world!");
    // let mut x = 5;
    // println!("the value of x is: {x}");
    // x = 6;
    // println!("the value of x is: {x}");
    // println!("the value of name is: {name}");

    // const three_hours: u32 = 60 * 60 * 60 * 3;

    // print!("these are no. of seconds in an hours {three_hours}" );
    
    // let x = 5;
    // let x = x + 1;

    {
        // let x = x * 2;
        // println!("the value of x in the inner scope is {x}");
    }

    // println!("the value of x in the outer scope is {x}");

    // return_data();
    // tuples();
    // arrays();
    // access_array();

    // secondary_function();
    // print_x(x);
    // let area = calc_area(5, 5);
    // println!("area of a square: {area}");

    // let can_he = can_drink(18);
    // println!("You... {can_he}");

    // let arturos_age = 18;

    // let can_drink = if arturos_age >= 21 {"You can drink a cold one"} else {"Sorry pal"};
    // println!("{can_drink}")

    ownership_ie(); 
}
