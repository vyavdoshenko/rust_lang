fn main() {
    new_string();
    creating_strings();
    another_way_to_create_string();
    unicode_hello();
    updating_a_string();
    pushing_char();
    concat_with_plus();
    concat_with_format();
    str_len_in_bytes();
    slicing_string();
    iterating_over_string_as_chars();
    iterating_over_string_as_bytes();
}

fn new_string() {
    let mut s = String::new();
}

fn creating_strings() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}

fn another_way_to_create_string() {
    let s = String::from("initial contents");
}

fn unicode_hello() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn updating_a_string() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}

fn pushing_char() {
    let mut s = String::from("lo");
    s.push('l');
}

fn concat_with_plus() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}

fn concat_with_format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
}

fn str_len_in_bytes() {
    let len = String::from("Здравствуйте").len();
}

fn slicing_string() {
    let hello = "Здравствуйте";

    let s = &hello[0..4]; // 0..4 - indices in bytes
}

fn iterating_over_string_as_chars() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}

fn iterating_over_string_as_bytes() {
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

