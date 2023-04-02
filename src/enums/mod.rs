pub mod options;
pub mod match_flow;
// #[derive(Debug)]
// enum IpAddrType {
//     V4,
//     V6
// }

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrType,
//     address: String,
// }

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMesasge(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}



#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn enums_main() {
    // let four = IpAddrType::V4;
    // let six = IpAddrType::V6;
  
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // route(home);
    // route(loopback);

    // let home = IpAddr {
    //     kind: IpAddrType::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrType::V6,
    //     address: String::from("::1"),
    // };

    println!("home: \n {:#?}", home);
    println!("loopback: \n {:#?}", loopback);


    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route(ip_type: IpAddr) {

}
