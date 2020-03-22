fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("First part: {}", hello);
    println!("Second part: {}", world);

    full_slice();
    array_slice();
}

fn full_slice() {
    let s = String::from("hello");

    let len = s.len();

    let slice1 = &s[0..len];
    let slice2 = &s[..];

    println!("Original: {}", s);
    println!("First: {}", slice1);
    println!("Second: {}", slice2);
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[2..4];

    for element in slice.iter() {
        println!("The value is: {}", element);
    }
}
