fn main() {
    let name1 = "Muazu Fadhilullahi";
    println!("My name is {}",name1);

    let name2 = name1.replace("Muazu", "Abdul-kadir");
    println!("You can also call me {}",name2);


    let faculty = "Faculty of Science and Technology";
    let school = faculty.replace("Faculty","School");
    println!("I'm a student of the {}",school);
}
