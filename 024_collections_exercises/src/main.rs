fn main() {
    let v:Vec<f64> = vec![1.0, 2.0, 5.0, 7.0, 2.0, 3.0, 9.0, 2.0, 4.0, 3.0, 2.0, 3.0, 5.0, 9.0, 10.0];
    let mean = get_mean(&v);

    println!("Mean: {:?}", mean);
}

fn get_mean(v: &Vec<f64>) -> f64 {
    let mut sum:f64 = 0.0;

    for element in v.iter() {
        sum += element;
    }

    sum / v.len() as f64
}