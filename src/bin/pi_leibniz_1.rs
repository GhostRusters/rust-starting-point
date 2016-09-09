use std::f64;

fn main() {
     let nums = match std::env::args().nth(1) {
         Some(param) => param.parse::<i32>().unwrap(),
         None => 0,
     };

    println!("{:?}", nums);

    let mut current_iteration = 0;
    let mut denom = 1.0;
    let mut signal = 1.0;
    let mut pi_value = 0.0;
    while current_iteration != nums {
        let term: f64 = ( 4.0 / denom ) * signal;
        pi_value += term;
        signal *= -1.0;
        current_iteration += 1;
        denom += 2.0;
    }
    println!("Calculated PI Value is:{:?}", pi_value );
}
