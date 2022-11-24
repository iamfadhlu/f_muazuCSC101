use std::io;


fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();


    println!("Rust Progrom to calculate the area of a 
    	triangle for a given base and height\nEnter base:");
    io::stdin().read_line(&mut input1).expect("Not a valid integer");
    let base:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter height:");
    io::stdin().read_line(&mut input2).expect("Not a valid integer");
    let height:f32 = input2.trim().parse().expect("Not a valid number");
    if base > 0.0 {
    	let area:f32 = (base * height)/2.0;
    	println!("The Area of the triangle is {}", area);

    }




}
