fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("Tuple: (i32, f64, u8): {}, {}, {}", tup.0, tup.1, tup.2);

    destructure_tuple();
}

fn destructure_tuple() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);
}