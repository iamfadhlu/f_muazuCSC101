fn main() {
    let city_arr:[&str;5] = ["Abuja","Portharcourt", "Maiduguri", "Kano", "Lagos"];
    println!("Array is {:?}",city_arr);
    println!("Array size is {}",city_arr.len());

    for index in 0..5 {
    	println!("City Index {} is Located in {}",index, city_arr[index]);

    }
}