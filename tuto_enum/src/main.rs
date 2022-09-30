/*
enum IpAddrKind {
    v4,
    v6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
*/
// Or
enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match &self {
            Self::Penny => 1,
            Self::Nickel => 5,
            Self::Dime => 10,
            Self::Quarter => 25,
        }
    }
}

fn main() {
    println!("Hello, world!");

    // let home = IpAddr {
    //     kind: IpAddrKind::v4,
    //     address: String::from("127.0.0.1")
    // };
    // Or
    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let my_coin = Coin::Nickel;
    println!("My coin is equal to {} cents!", my_coin.value_in_cents());
}
