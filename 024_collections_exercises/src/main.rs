fn main() {
    let v:Vec<f64> = vec![1.0, 2.0, 5.0, 7.0, 2.0, 3.0, 9.0, 2.0, 4.0, 3.0, 2.0, 3.0, 5.0, 9.0, 10.0];
    let mean = get_mean(&v);
    let median = get_median(&v);
    // let mode = get_mode(&v);

    println!("Mean: {:?}", mean);
    println!("Median: {:?}", median);
    // println!("Mode: {:?}", mode);
}

fn get_mean(v: &Vec<f64>) -> f64 {
    let mut sum:f64 = 0.0;

    for element in v.iter() {
        sum += element;
    }

    sum / v.len() as f64
}

fn get_median(v: &Vec<f64>) -> f64 {
    let mut vec = v.clone();
    
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vec[vec.len() / 2]
}

// fn get_mode(v: &Vec<f64>) -> f64 {
//     use std::collections::HashMap;

//     let mut map:HashMap<OrderedFloat, u32> = HashMap::new();

//     for element in v.iter() {
//         let count = map.entry(element).or_insert(0);
//         *count += 1;
//     }

// }
