fn main() {
    println!("Hello, world!");
}

fn bar() -> ! {
    print!("forever ");

    loop {
        print!("and ever ");
    }
}