//use std::slice::range;


fn main() {
    
    let mut x = 0;

    loop {
        x += 1;
        println!("{x}");
        break;
    }

    for i in (-10..-5).rev() {
        println!("{i}");
    }

    for i in (-10..-5) {
        println!("{i}");
    }

    let mut j = 0;
    while j < 1 {
        println!("{j}");
        j+=1;
    }

}
