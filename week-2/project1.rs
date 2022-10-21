fn main() {
	//p is for principal which is the mortage loan
	let p:f64 = 520_000_000.0;
	let r:f64 = 10.0;
	let t:f64 = 5.0;


	let a = p * (1.0 + (r/100.0)) * t;
	println!("The Amount is {}", a );

	// time for compound interest
	let ci = a - p;
	println!("The Amount for the Compound Interest is {}", ci ); 


}