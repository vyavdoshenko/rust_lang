use std::collections::HashMap;

pub use ordered_float::*;

fn main() {
    let v: Vec<f64> = vec![1.0, 2.0, 5.0, 7.0, 2.0, 3.0, 9.0, 2.0, 4.0, 3.0, 2.0, 3.0, 5.0, 9.0, 10.0];
    let mean = get_mean(&v);
    let median = get_median(&v);
    let mode = get_mode(&v);

    println!("Mean: {:?}", mean);
    println!("Median: {:?}", median);
    println!("Mode: {:?}", mode);

    println!("Pig latin: {:?}", pig_latin(&String::from("first")));
    println!("Pig latin: {:?}", pig_latin(&String::from("apple")));

    let mut collection: HashMap<String, Vec<String>> = HashMap::new();
    add_employee(&String::from("Add Sally to Engineering"), &mut collection);
    add_employee(&String::from("Add Amir to Sales"), &mut collection);
    add_employee(&String::from("Add Bobby to Engineering"), &mut collection);
    add_employee(&String::from("Add Ahmad to Sales"), &mut collection);
    add_employee(&String::from("Add Bob to Tech"), &mut collection);
    add_employee(&String::from("Add Alisa to Sales"), &mut collection);

    print_employees_by_department(&String::from("Engineering"), &collection);
}

fn get_mean(v: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;

    for element in v.iter() {
        sum += *element;
    }

    sum / v.len() as f64
}

fn get_median(v: &Vec<f64>) -> f64 {
    let mut vec = v.clone();

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vec[vec.len() / 2]
}

fn get_mode(v: &Vec<f64>) -> f64 {
    let mut collection: HashMap<OrderedFloat<f64>, u32> = HashMap::new();

    for element in v.iter() {
        let count = collection.entry(OrderedFloat::<f64>(*element)).or_insert(0);
        *count += 1;
    }

    collection
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
        .into_inner()
}

// English vowels: A, E, I, O, U, and sometimes Y
fn pig_latin(s: &String) -> String {
    let vowels = "aeioy".to_string();
    if s.chars().count() > 0 {
        let first_letter = s.chars().next().unwrap();
        if vowels.find(first_letter.to_ascii_lowercase()) != None {
            return s.clone() + "-hay";
        }

        let mut ms: String = s.chars().skip(1).take(s.chars().count() - 1).collect();
        ms.push('-');
        ms.push(first_letter);
        ms.push_str("ay");

        return ms;
    }

    "".to_string()
}

fn add_employee(s: &String, collection: &mut HashMap<String, Vec<String>>) {
    let mut words = s.split_whitespace();
    if Some("Add") == words.next() {
        let name = words.next();
        if name != None {
            if Some("to") == words.next() {
                let department = words.next();
                if department != None && words.next() == None {
                    collection.entry(String::from(department.unwrap()))
                        .or_insert(Vec::new())
                        .push(String::from(name.unwrap()));
                }
            }
        }
    }
}

fn print_employees_by_department(dep: &String, collection: & HashMap<String, Vec<String>>) {
    println!("Department: {:?}", dep);
    let employees= collection.get(dep);
    if employees != None {
        let mut sorted = employees.unwrap().clone();
        sorted.sort();
        println!("Employees: {:?}", &sorted);
    }
}
