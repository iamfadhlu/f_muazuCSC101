use std::fs::File;
use std::io::Write;
fn main() {
    let lager = vec!["33 Export   ", "Desperados  ", "Goldberg    ", "Gulder      ", "Heineken    ", "Star        "];
    let stout = vec!["Legend      ", "Turbo King  ", "Williams    ", "     -      ", "     -      ", "     -      "];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz", "     -      ", "     -      "];
    let mut file = File::create("data.txt").expect("Failed to create");
    file.write_all("Nigerian Breweries Plc, the pioneer and largest brewing Company in Nigeria was\n incorporated in 1946 as Nigerian Brewery Limited. Their rich portfolio of high-quality Lager, Stout, Non-alcoholics and Spirit are uniquely outstanding which is why they are Nigeria’s number one choice, as shown below.
\n".as_bytes()).expect("write failed");
    file.write_all("LAGER         |    STOUT   |    NON-ALCOHOLIC\n".as_bytes()).expect("write failed");
    file.write_all("_________________________________________________\n".as_bytes()).expect("write failed");
    for i in 0..lager.len(){
    	file.write_all(lager[i].as_bytes()).expect("write failed");
    	file.write_all("  |  ".as_bytes()).expect("write failed");
    	file.write_all(stout[i].as_bytes()).expect("write failed");
    	file.write_all("  |  ".as_bytes()).expect("write failed");
    	file.write_all(non_alcoholic[i].as_bytes()).expect("write failed");
    	file.write_all("\n".as_bytes()).expect("write failed");




    }
}
