use std::io;

fn main() {
    println!("\nStudent Information Management System");

    println!("\n Please Enter your name");
    let mut name = String::new();
    io::stdin() .read_line(&mut name).expect("Failed to read input");

println!("Your name is {}",name );

println!("\n Please Enter your Age. ");
let mut age = String::new();
io::stdin().read_line(&mut age).expect("Failed to read input");
let age:i32 = age.trim().parse().expect("Input isn't a number");
println!("Your age is: {}",age );




}
