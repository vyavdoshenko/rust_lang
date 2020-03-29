fn main() {
    matching(Some(3));
    matching(Some(4));
    matching(None);

    iflet(Some(3));
    iflet(Some(4));
    iflet(None);
}

fn matching(x: Option<i32>) {
    match x {
        Some(3) => println!("three"),
        _ => (),
    }
}

fn iflet(x: Option<i32>) {
    if let Some(3) = x {
        println!("three");
    } else {
        ();
    }
}