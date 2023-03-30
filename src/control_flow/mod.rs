
pub fn can_drink(age: i32) -> &'static str {
    if age < 21 {
        "You can Drink!"
    } else {
        "Cannot Drink"
    }

}


