use std::io;

fn trapezium(){
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	println!("Let's find the area of the trapezium!\nWhat is the height in cm?");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let ht:i32 = input1.trim().parse().expect("Not a valid input");
	println!("What is the top width in cm?");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let wt:i32 = input2.trim().parse().expect("Not a valid input");
	println!("What is the bottom with in cm?");
	io::stdin().read_line(&mut input3).expect("Failed to read input");
	let bt:i32 = input3.trim().parse().expect("Not a valid input");
	let height_tra = ht / 2;
	let width_tra = wt + bt;
	let tf = height_tra * width_tra;
	println!("The area is {}cm^2",tf);
}
fn rhombus(){
	let mut input_1 = String::new();
	let mut input_2 = String::new();
	println!("Let's find the area of the Rhombus!\nWhat is the length of the 1st diagonal in cm?");
	io::stdin().read_line(&mut input_1).expect("Failed to read input");
	let d1:i32 = input_1.trim().parse().expect("Not a valid input");
	println!("What is the 2nd diagonal in cm?");
	io::stdin().read_line(&mut input_2).expect("Failed to read input");
	let d2:i32 = input_2.trim().parse().expect("Not a valid input");
	let rf = (d1 * d2) / 2;
	println!("The area is {}cm^2",rf);
}
fn parallelogram(){
	let mut input_11 = String::new();
	let mut input_22 = String::new();
	println!("Let's find the area of the Parallelogram\nWhat is the length of the base in cm?");
	io::stdin().read_line(&mut input_11).expect("Failed to read input");
	let base_para:i32 = input_11.trim().parse().expect("Not a valid input");
	println!("What is the length of the height in cm?");
	io::stdin().read_line(&mut input_22).expect("Failed to read input");
	let height_para:i32 = input_22.trim().parse().expect("Not a valid input");
	let pf = height_para * base_para;
	println!("The area is {}cm^2",pf);
}
fn cube(){
	let mut input_111 = String::new();
	println!("Let's find the area of the Cube\nWhat is the length of the cube in cm?");
	io::stdin().read_line(&mut input_111).expect("Failed to read input");
	let lc:i32 = input_111.trim().parse().expect("Not a valid input");
	let l_c = i32::pow(lc, 2);
	let cf = l_c * 6;
	println!("The area is {} cm^2",cf );

}
fn cylinder(){
    let mut input_222 = String::new();
    let mut input_333 = String::new();
	println!("Let's find the Volume of the Cylinder\nWhat is the radius of the cylinder in cm?");
	io::stdin().read_line(&mut input_222).expect("Failed to read input");
	let r:f32 = input_222.trim().parse().expect("Not a valid input");
	println!("What is the height of the cylinder in cm?");
	io::stdin().read_line(&mut input_333).expect("Failed to read input");
	let hcy:f32 = input_333.trim().parse().expect("Not a valid input");
	let r2 = f32::powf(r, 2.0);
	let cyf = r2 * 3.142 * hcy;
	println!("The Volume is {} cm^3",cyf );
}
fn main() {
	let mut option = String::new();
    println!("Welcome to The Calculator\nWhat would you like to calculate?");
    println!("1.)Area of Trapezium\n2.)Area of Rhombus\n3.)Area of Parallelogram\n4.)Area of Cube\n5.)Volume of Cylinder");
    io::stdin().read_line(&mut option).expect("Failed to read input");
    let a:i32 = option.trim().parse().expect("Not a valid input");
    if a == 1{
    	trapezium()
    }else if a == 2{
    	rhombus()
    }else if a == 3{
    	parallelogram()
    }else if a == 4{
    	cube()
    }else if a == 5{
    	cylinder()

    }else{
    	
    } println!("Restart the program and choose from the options available!");

}
