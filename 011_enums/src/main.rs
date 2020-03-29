fn main() {
    ip_enum();
}

fn ip_enum() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let v4 = IpAddr::V4(127, 0, 0, 1);

    let v6 = IpAddr::V6(String::from("::1"));

    println!("v4: {:?}", v4);
    println!("v6: {:?}", v6);
}
