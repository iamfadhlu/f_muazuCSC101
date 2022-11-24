use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid String");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid number");
    let age:i32 = input2.trim().parse().expect("Not a valid number");


    if age >= 18 {
    	println!("Welcome to the party");
    } else{
    	let age:i32 = 18 - age;
    	println!("Ooops! Come when you're older :)\nYou're {} years too young!", age);
    }
}
