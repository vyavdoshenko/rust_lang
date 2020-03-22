fn main() {
    different_types()
}

fn different_types() {
    // Different types. They can't be assigned to each other.
    #[derive(Debug)] // This code added for debug printing.
    struct Color(i32, i32, i32);

    #[derive(Debug)] // This code added for debug printing.
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(1, 2, 3);

    println!("Color: {:?}", black);
    println!("Point: {:?}", origin);


    println!("origin.1: {:?}", origin.1);
}
