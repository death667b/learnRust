struct IPAddr {
    kind: IPAddrKind,
    address: String
}

enum IPAddrKind {
    V4,
    V6
}

enum IPAddr2 {
    V4(String),
    V6(String)
}

enum IPAddr3 {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

fn main() {
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;

    let home = IPAddr {
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IPAddr {
        kind: IPAddrKind::V6,
        address: String::from("::1")
    };

    let home2 = IPAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IPAddr2::V6(String::from("::1"));

    let home3: IPAddr3 = IPAddr3::V4(127, 0, 0, 1);
    let loopback3: IPAddr3 = IPAddr3::V6(String::from("::1"));

    route(four);
    route(six);
    
    println!("Hello, world!");
}

fn route(ip_kind: IPAddrKind) {}