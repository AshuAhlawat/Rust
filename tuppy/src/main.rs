fn main() {
    let mut x = 0;
    {
        println!("{x}");
        let tup1 = (2,45,-3.14, false, (5, 3));
        println!("{} {}", tup1.4.1, tup1.2);
        x = 1;
    }
    
    // println!("{}", tup1.1)

    println!("{}", x);

    {
        let x = true;
    }

    println!("{}", x)


}
