use std::io;
fn main() {
    let mut input = String::new();

    println!("Enter your height(cm):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");


    if height >= 150.0 && height <= 170.0{
    	println!("You're of average height");
    } else if height > 170.0 && height <= 200.0 {
    	println!("You're tall!");    	
    }else if height < 150.0 && height > 100.0 {
    	println!("You're kinda short ;)");
    }else {
    	println!("Your height is abnormal");
    }
}
	