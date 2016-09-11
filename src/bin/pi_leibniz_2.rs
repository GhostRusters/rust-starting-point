use std::f64;

fn main() {
     let nums = match std::env::args().nth(1) {
         Some(param) => param.parse::<i32>().unwrap(),
         None => 0,
     };

    println!("{:?}", nums);

    let mut pi_value = 0.0;
    for i in 1..nums {
        let base = 4.0 / ( (2.0 * i as f64) - 1.0 );        
        if i % 2 == 0 {pi_value += base *-1.0;} else {pi_value += base;}
    }
    println!("Calculated PI Value is:{:?}", pi_value );
}
