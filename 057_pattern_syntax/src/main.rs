fn main() {
    matching_literals();
    matching_named_variables();
    multiple_patterns();
    matching_inclusive_number_range();
    matching_inclusive_char_range();
    destructing_struct();
    destructing_struct_shorthand();
    destructing_struct_matching_literals();
    destructing_enum();
    destructing_nested_struct();
    destructing_complex_structs_and_tuples();
    ignore_unused_parameter(3, 4);
    ignore_part_of_pattern();
    suppress_warning_for_unused_variables();
    ignoring_remaining_part();
    ignoring_remaining_part_of_tuple();
    ignoring_most_values_except_penultimate();
    match_guard();
    match_guard_outer_var();
    combining_multiple_patterns();
    at_bindings();
}

fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_inclusive_number_range() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

fn matching_inclusive_char_range() {
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructing_struct() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

fn destructing_struct_shorthand() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn destructing_struct_matching_literals() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructing_enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}

enum NestedColor {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum NestedMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(NestedColor),
}

fn destructing_nested_struct() {
    let msg = NestedMessage::ChangeColor(NestedColor::Hsv(0, 160, 255));

    match msg {
        NestedMessage::ChangeColor(NestedColor::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        NestedMessage::ChangeColor(NestedColor::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

fn destructing_complex_structs_and_tuples() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("Feet: {}", feet);
    println!("Inches: {}", inches);
    println!("x: {}", x);
    println!("y: {}", y);
}

fn ignore_unused_parameter(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn ignore_part_of_pattern() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

fn ignore_part_of_container() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

fn suppress_warning_for_unused_variables() {
    let _x = 5;
    let _y = 10;
}

fn ignoring_remaining_part() {
    struct LocalPoint {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = LocalPoint { x: 0, y: 0, z: 0 };

    match origin {
        LocalPoint { x, .. } => println!("x is {}", x),
    }
}

fn ignoring_remaining_part_of_tuple() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

fn ignoring_most_values_except_penultimate() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., before_last, _) => {
            println!("Penultimate value: {}", before_last);
        }
    }
}

fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn match_guard_outer_var() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}

fn combining_multiple_patterns() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn at_bindings() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}