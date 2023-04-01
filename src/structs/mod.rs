
pub mod example;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Color(i32, i32, i32); // this is an rgb color
struct Point(i32, i32, i32); 

// unit structs
struct AlwaysEqual;

fn create_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

pub fn struct_main() {

    let user1 = User {
        active: true,
        username: String::from("arturfil"),
        email: String::from("arturo@test.com"),
        sign_in_count: 1,
    };

    println!("{}, has email {}", user1.username, user1.email);
    let user2:User = create_user(String::from("arturo2@test.com"), String::from("artufil"));
   
    // there is also spred operator like in js
    let user3 = User {
        email: String::from("adri@test.com"),
        username: String::from("adri"), 
        ..user1
    };
    
    let purple = Color(234,200,183);
    let x = Point(100,8,2);

    let subject = AlwaysEqual;
}


