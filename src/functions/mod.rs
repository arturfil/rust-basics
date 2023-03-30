
pub fn secondary_function() {
    println!("This is a secondary example");
}

pub fn print_x(x: i32) {
    println!("The value of the arguemnt x is :{x}");
}

// as you can see, this has no ';' because without it, it
// is implied that it returns that value
pub fn calc_area(height: i32, width: i32) -> i32 {
    height * width
}
