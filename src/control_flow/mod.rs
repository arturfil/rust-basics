
pub fn can_drink(age: i32) -> &'static str {
    if age < 21 {
        "You can Drink!"
    } else {
        "Cannot Drink"
    }

}

// let can_he = can_drink(18);
// println!("You... {can_he}");

// let arturos_age = 18;

// let can_drink = if arturos_age >= 21 {"You can drink a cold one"} else {"Sorry pal"};
// println!("{can_drink}")

