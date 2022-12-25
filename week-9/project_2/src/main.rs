use std::fs::File;
use std::io::Write;
use std::io::Read;
fn main() {
    let student_name = vec!["Oluchi Mordi ", "Adams Aliyu  ", "Shania Bolade", "Adekunle Gold", "Blannca Edemoh",];
    let matric_number = vec!["ACC10211111     ", "ECO10110101     ", "CSC10328828     ", "EEE11020202     ", "MEE10202001     ",];
    let department = vec!["Accounting        ", "Economics         ", "Computer          ", "Electrical        ", "Mechanical        "];
    let level = vec!["300", "100", "200","200","100"];
    let mut file = File::create("data.txt").expect("Failed to create");
    file.write_all("PAU uses a Student Management Information System (PAU-SMIS) to manage student-related data.\n This system provides facilities for recording and maintaining personal details of students, maintaining marks scored in assessments and computing\n results of students, keeping track of student attendance, managing many other student-related data. The data is shown as follows
    \n".as_bytes()).expect("write failed");
    file.write_all("Student name   |    Matric Number   |    Deparment    |     Level\n".as_bytes()).expect("write failed");
    file.write_all("_____________________________________________________________\n".as_bytes()).expect("write failed");
    for i in 0..department.len(){
    	file.write_all(student_name[i].as_bytes()).expect("write failed");
    	file.write_all("  |  ".as_bytes()).expect("write failed");
    	file.write_all(matric_number[i].as_bytes()).expect("write failed");
    	file.write_all("  |  ".as_bytes()).expect("write failed");
    	file.write_all(department[i].as_bytes()).expect("write failed");
    	file.write_all("  |  ".as_bytes()).expect("write failed");
    	file.write_all(level[i].as_bytes()).expect("write failed");
    	file.write_all("\n".as_bytes()).expect("write failed");
    }

    let mut file = File::open("data.txt").expect("Failed to create");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); 
    println!("{}", contents );


}
