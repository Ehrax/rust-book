use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn run() {
    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let x: Option<u32> = Some(50);
    match x {
        Some(50) => println!("50"),
        Some(i) => println!("i: {}", i),
        None => print!("None"),
    }

    if let Some(50) = x {
        println!("x is 50")
    } else {
        println!("x is not 50")
    }
}
