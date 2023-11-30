fn main() {
    let number = 0;


    // roll a die
    let mut output = "";

    if number == 1 {
        output = "One"
    } else if number == 2 {
        output = "Two"
    } else {
        output = "Other"
    }

    println!("\n\n{output}")
}
