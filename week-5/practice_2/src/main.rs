use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the first edge of the triange: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");
    println!("The first edge is {}cm",a );

    println!("Enter the second edge of the triangle: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");
    println!("The second edge is {}cm",b );


    println!("Enter the second edge of the triangle: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid integer");
    println!("The second edge is {}cm",c );


    let s:f32 = (a + b + c) / 2.0;
    let mut area:f32 = s * (s - a) * (s-b) * (s - c);
    area = area.sqrt();

    println!("Area of a triangle: {}cm^2",area );

}
