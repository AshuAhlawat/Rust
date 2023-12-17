//use std::slice::range;


fn main() {
    
    let mut x = 0;

    loop {
        x += 1;
        println!("{x}");
        break;
    }
    
    let rand_rangje = -10..-5;

    for i in rand_rangje.clone() {
        println!("{i}");
    }

    for (i, val) in rand_rangje.rev().enumerate() {
        println!("{i}, {val}");
    }

    let mut j = 0;
    while j < 1 {
        println!("{j}");
        j+=1;
    }

    for j in "hello".chars() {
        println!("{j}")
    }

    for i in 5..10 {
        println!("{i}")
    }

}
