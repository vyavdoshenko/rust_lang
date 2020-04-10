fn main() {
    monthes();
    fill_array();
    enumerate_and_reverse();
    let s = String::from("Test str!");
    print_str_by_chars(&s);
}

fn monthes() {
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let mut i: usize = 0;
    loop {
        println!("months[{}]: {}", i, months[i]);

        i += 1;

        if months.len() == i {
            break;
        }
    }
}

fn fill_array() {
    let a = [3; 5];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

fn enumerate_and_reverse() {
    for number in (5..12).rev() {
        println!("Reverse number: {}", number);
    }
}

fn print_str_by_chars(s: &String) {
    let bytes = s.as_bytes();

    let mut i: usize = 0;
    while bytes.len() != i {
        println!("Index: {}, char number: {}", i, bytes[i]);
        i += 1;
    }
}