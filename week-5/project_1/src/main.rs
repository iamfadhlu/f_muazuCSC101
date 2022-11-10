use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();


    println!("Program To Find The Roots Of A Quadratic Equation");
    println!("Format: ax^2 +bx +c");


    println!("Input A");
    io::stdin().read_line(&mut a).expect("Not a valid integer");
    let a:f32 = a.trim().parse().expect("Not a valid integer");
    println!("Successfully inputed!");


    println!("Input B");
    io::stdin().read_line(&mut b).expect("Not a valid integer");
    let a:f32 = b.trim().parse().expect("Not a valid integer");
    println!("Successfully inputed!");


    println!("Input C");
    io::stdin().read_line(&mut c).expect("Not a valid integer");
    let a:f32 = c.trim().parse().expect("Not a valid integer");
    println!("Successfully inputed!");


    let formula = 


}
