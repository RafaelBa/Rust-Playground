// enum and struct
enum IpAddressKind1 {
    V4,
    V6
}

struct IpAddress1 {
    kind: IpAddressKind1,
    address: String
}

// homogenious enum
enum IpAddress2 {
    V4(String),
    V6(String)
}

// heterogenous enum
enum IpAddress3 {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    let home1 = IpAddress1 {
        kind: IpAddressKind1::V4,
        address: String::from("127.0.0.1")
    };

    let loopback1 = IpAddress1 {
        kind: IpAddressKind1::V6,
        address: String::from("::1")
    };

    let home2 = IpAddress2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddress2::V6(String::from("::1"));

    let home3 = IpAddress3::V4(127, 0, 0, 1);
    let loopback = IpAddress3::V6(String::from("::1"));
}
