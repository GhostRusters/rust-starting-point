use std::f64;

fn main() {

     let iterations = std::env::args().nth(1);
     println!("{:?}", iterations);

     let nums = match std::env::args().nth(1) {
         Some(param) => param.parse::<i32>().unwrap(),
         None => 0,
     };

    println!("{:?}", nums);

    let mut current_iteration = 0;
    let mut denom = 1;
    let mut signal = 1;
    let mut pi_value:f64 = 0 as f64;
    while current_iteration != nums {
        let term: f64 = ( 4 as f64 / denom as f64 ) * signal as f64;
        pi_value += term;
        //println!("Iteration:{:?}, denom:{:?}, term:{:?}, , pi:{:?}",  current_iteration, denom, term, pi_value );
        signal *= -1;
        current_iteration += 1;
        denom += 2;
    }
    println!("Calculated PI Value is:{:?}", pi_value );
}
