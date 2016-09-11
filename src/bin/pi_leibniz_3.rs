use std::f64;
use std::time::Instant;

fn main() {
     let nums = match std::env::args().nth(1) {
         Some(param) => param.parse::<i32>().unwrap(),
         None => 200000,
     };

    println!("{:?}", &nums);

    let s_timer = Instant::now();
    let pi_final = calculate_pi(&nums);
    let e_timer = s_timer.elapsed();

    println!("Time was: {:?} ({:?})to calculate PI Value as:{:?}", &e_timer.subsec_nanos(), &e_timer, &pi_final );
}

fn calculate_pi(nums: &i32) -> f64 {
    let mut pi_value = 0.0;
    for i in 1..*nums {
        let base = 4.0 / ( (2.0 * i as f64) - 1.0 );
        if i % 2 == 0 {pi_value += base *-1.0;} else {pi_value += base;}
    }
    return pi_value;
}
