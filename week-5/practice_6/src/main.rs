use std::io;
fn main() {
    println!("Enter Lower Bound");
    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let lower_bound:i32 = input1.trim().parse().expect("Failed to input");

    println!("Enter Upper Bound");
    io::stdin().read_line(&mut input2).expect("Failed to input");
    let upper_bound:i32 = input2.trim().parse().expect("Failed to input");

    for x in lower_bound..upper_bound{
    	println!("Count level is {}",x );
    }
}
