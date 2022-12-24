use std::io;
fn main() {
	let mut input1 = String::new();
    println!("Welcome\nHow many siblings do you have?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
	let number_of_siblings:i32 = input1.trim().parse().expect("Not a valid input");
	if number_of_siblings == 0{
		println!("You don't have any siblings\nThe program is for people with siblings only\nNo offense!");
	}else if number_of_siblings >= 1{
		println!("Okayyy!");
		for x in 1..number_of_siblings+1{
			let mut input2 = String::new();
			println!("What is the name of the {}", x);
			io::stdin().read_line(&mut input2).expect("Failed to read input");
	        let name_of_siblings:&str = input1.trim();
	        println!("What is the age of {}",x );
	        let mut input3 = String::new();
	        io::stdin().read_line(&mut input3).expect("Failed to read input");
	        let age_of_siblings:i32 = input3.trim().parse().expect("Not a valid input");
	        if age_of_siblings >= 18{
	        	println!("What is the marital status?\n1.)Single\n2.)Married");
	        	let mut input4 = String::new();
	        	io::stdin().read_line(&mut input4).expect("Failed to read input");
	            let marital_status:i32 = input4.trim().parse().expect("Not a valid input");
	            if marital_status == 1{
	            	println!("Is the sibling a :\n1.)Student\n2.)Worker");
	            	let mut input5 = String::new();
	        	    io::stdin().read_line(&mut input5).expect("Failed to read input");
	                let job_status:i32 = input5.trim().parse().expect("Not a valid input");
	                if job_status == 1{
	                	println!("What is the name of the University and the course of study?"); 
	                	let mut input6 = String::new();
	        	        io::stdin().read_line(&mut input6).expect("Failed to read input");
	                    let university:&str = input5.trim();

	                }

	            }else if marital_status == 2{
	            	println!("Does the sibling have children?\n1.)Yes\n2.)No");
	            	let mut input7 = String::new();
	        	    io::stdin().read_line(&mut input7).expect("Failed to read input");
	                let offspring:i32 = input7.trim().parse().expect("Not a valid input");	
	                println!("What city does the sibling live in?");
	                let mut input8 = String::new();
	        	    io::stdin().read_line(&mut input8).expect("Failed to read input");
	                let city:&str = input8.trim();

                }

	        }else if age_of_siblings < 18{
	        	println!("Has the sibling written WAEC?\n1.)Yes\n2.)No");
	        	let mut input9 = String::new();
	        	io::stdin().read_line(&mut input9).expect("Failed to read input");
	            let waec_status:i32 = input9.trim().parse().expect("Not a valid input");
	            if waec_status == 1{
	            	println!("What secondary school did the sibling attend?");
	            	let mut input10 = String::new();
	        	    io::stdin().read_line(&mut input10).expect("Failed to read input");
	                let sec_sch:&str = input10.trim();
	            }else if waec_status == 2{
	            	let status_str = "Not done";
	            	println!("What is the current class of the sibling?");
	            	let mut input11 = String::new();
	        	    io::stdin().read_line(&mut input11).expect("Failed to read input");
	                let class:&str = input11.trim();

	            }
	        }

		}
	}
}
