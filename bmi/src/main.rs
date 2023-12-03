use std::io;

fn main() {
    let mut weight= String::new();

    println!("Enter your weight (kg): ");

    io::stdin().read_line(&mut weight).expect("Some error");

    

    println!("\nUr Fat {weight}");

}
