fn main() {
    init_empty_vec();
    use_macro_creating_vec();
    fill_vec();
    get_element();
    iterate_immutable_vec();
    change_vec_elements();
    enum_in_vec();
}

fn init_empty_vec() {
    let v: Vec<i32> = Vec::new();
}

fn use_macro_creating_vec() {
    let v = vec![1, 2, 3];
}

fn fill_vec() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn get_element() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn iterate_immutable_vec() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

fn change_vec_elements() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

fn enum_in_vec() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}