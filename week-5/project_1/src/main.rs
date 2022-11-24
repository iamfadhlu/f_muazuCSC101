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
    let b:f32 = b.trim().parse().expect("Not a valid integer");
    println!("Successfully inputed!");


    println!("Input C");
    io::stdin().read_line(&mut c).expect("Not a valid integer");
    let c:f32 = c.trim().parse().expect("Not a valid integer");
    println!("Successfully inputed!");

    let b_square= f32::powf(b,2.0);
    let determinant = b_square + (4.0*a*c);
    let determinant_2 = b_square - (4.0*a*c);

    println!("The determinants are {} and {}",determinant, determinant_2);
    let formula_1 = determinant.sqrt();
    let formula_3 = -b + formula_1;
    let formula_4 = -b - formula_1;
    let final_1 = formula_3 / (2.0*a);
    let final_2 = formula_4 / (2.0*a);
    println!("The roots of the equation are {} and {}", final_1, final_2);




    if determinant > 0.0{
        println!("There are two distinct roots ");
    }else if determinant == 0.0{
        println!("There is exactly one real root");
    }else if determinant < 0.0{
        println!("There're no real roots");
    }if determinant > 0.0{
        println!("There are two distinct roots ");
    }


    










}
