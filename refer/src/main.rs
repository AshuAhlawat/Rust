fn main() {
    let mut base1 = (20,30);


    let home = &base1; // immutable reference

    let starter = &mut home;

    
    
    println!("{}", base1.1);

    println!("{}", home.1);

    println!("{}", starter.1);
    
    *starter = &(40,60); // gives error as we can either have many immutable references to same data, or only 1 mutable reference

    println!("{}", base1.1)
    
}
