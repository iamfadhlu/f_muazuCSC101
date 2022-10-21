fn main() {

	let p:f64 = 210_000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//formula for depriciation
	let c = 1.0 - (r/100.00);
	let a = f64::powf(c, n);
	let b = p * a;
	println!("The depreciation amount is {}", b);



}