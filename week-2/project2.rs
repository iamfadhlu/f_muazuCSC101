fn main() {
	//t is for toshiba
	let t:f64 = 450_000.0;
	let nt = t * 2.0;
	//m is for mac
	let m:f64 = 1_500_000.0;
	let nm = m * 1.0;
	//hp is for hp :)
	let hp:f64 = 750_000.0;
	let nhp = hp * 3.0;
	//d if for dell
	let d:f64 = 2_850_000.0;
	let nd = d * 3.0;
	//a if for acer
	let a:f64 = 250_00.0;
	let na = a * 1.0;


	let total = nt + nm + nhp +nd + na;
	println!("The sum amount is {}", total );

	// time for average
	let average = total / 10.0;
	println!("The Average amount is {}", average ); 


}