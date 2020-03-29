fn main() {
    let five = Some(5);
    print_opt(five);

    let six = plus_one(five);
    print_opt(six);

    let none = plus_one(None);
    print_opt(none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_opt(x: Option<i32>) {
    match x {
        None => println!("None", ),
        Some(i) => println!("i: {:?}", i),
    }
}