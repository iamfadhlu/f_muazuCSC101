fn main() {
	//p is for principal which is the mortage loan
	let p:f64 = 520_000_000.0;
	let r:f64 = 10.0;
	let t:f64 = 5.0;


	let b = (1.0 + (r/100.0));
	let a = f32::powf(b, t);
	println!("The Amount is {}", a );

	// time for compound interest
	let ci = a;
	println!("The Amount for the Compound Interest is {}", a ); 


}