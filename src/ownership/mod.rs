pub fn ownership_ie() {
    let s = "hello";
    let mut t = String::from(s);
    t.push_str(", world");
    println!("{t}");

    let s1 = String::from("hello");
    // let s2 = s1; // this wont work
    //
    let s2 = s1.clone(); // this will work;

    println!("{}, world", s2);
}

pub fn ownership_n_funcs() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

pub fn return_values_n_scope() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

pub fn take_n_return() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let mut s = String::from("Hello");
    change(&mut s);
    let r3 = &s;
    let r4 = &s;
    // let r1 = &mut s;
    //let r2 = &mut s;
    // println!("{}, {}", r1, r4)
}

fn borrow_errors() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let mut s = String::from("Hello");
    change(&mut s);
    let r3 = &s;
    let r4 = &s;
    // let r1 = &mut s;
    //let r2 = &mut s;
    // println!("{}, {}", r1, r4) // This will give error because of the borrow

}

fn value_ownership_examples() {
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

}

fn last_reference_usage() {
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;

    println!("{}{}", r1, r2);

    let r3 = &mut s;
    // println!("{}", r3); this will also give you errors because of borrow
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn change(some_string: &mut String) {
   some_string.push_str(", world");
}

pub fn dangle_example() {
    // Uncomment this to see a pointer dangling -> pointing to nothing;
    // let dangling_point = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
