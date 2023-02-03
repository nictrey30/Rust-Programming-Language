// enum values can only be one of the variants
// enum IpAddrKind {
//     V4,
//     V6,
// }
// struct IpAddr {
//     kind: IpAddr,
//     address: String,
// }
// fn main() {

//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);

//     let home = IpAddr{
//       kind: IpAddrKind::V4,
//       address: String::from("127.0.0.1");
//     };
//     let loopback = IpAddr {
//       kind: IpAddrKind::V6,
//       address: String::from("::1");
//     };
// }
// fn route(ip_kind: IpAddrKind) {}

// we can represent the same concept using just an enum, by putting data into each enum variant
enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    // we attach the data to each variant of the enum directly, so there is no need for an extra struct
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

// There's another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.
// enum IpAddr{
//   V4(u8, u8, u8, u8),
//   V6(String),
// }
//     let home = IpAddr::V4(127,0,0,1);
//     let loopback = IpAddr::V6(String::from("::1"));
