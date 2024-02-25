#[derive(Debug)]
enum IPAddressType {
    V4,
    V6,
}
#[derive(Debug)]
enum IPAddr {
    V4(String),
    V6(String),
}
#[derive(Debug)]
struct IPAddress {
    kind: IPAddressType,
    address: String,
}

enum Message {
    Quit,                       // unit struct
    Move { x: i32, y: i32 },    // struct
    Write(String),              // tuple struct with single value
    ChangeColor(i32, i32, i32), // tuple struct with multiple value
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let home = IPAddress {
        kind: IPAddressType::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IPAddress {
        kind: IPAddressType::V6,
        address: String::from("::1"),
    };

    dbg!(&home);
    dbg!(&loopback);

    let home = IPAddr::V4(String::from("127.0.0.1"));
    let loopback = IPAddr::V6(String::from("::1"));

    let x = 5;
    let y = Some(6);

    let mut z = 0;

    match y {
        Some(a) => z = x + a,
        None => println!("y is None"),
    }

    println!("{}", z);
    let penny = Coin::Penny;
    println!("Cents in a Penny are: {}", value_in_cents(penny));
}

fn value_in_cents(coin: Coin) -> usize {
    match coin {
        Coin::Penny => {
            println!("In Penny");
            1
        }
        Coin::Nickel => {
            println!("In Nickel");
            5
        }
        Coin::Dime => 10,
        //Coin::Quarter => 25,
        _ => 0,
    }
}
