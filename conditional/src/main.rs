fn main() {
    let x = 5;

    let answer = match x {
        0|1   => "zero or one",
        2..=9 => "two 2 nine",
        _     => "else"
    };

    println!("{answer}");

    for i in 50..61 {
        if i%2 == 0 {
            println!("{i}")
        }
    }

    let y = 10;
    match y {
        2|3 => 
            println!("two")
        ,4
         => println!
         ("three"),
         5..=10 => println!("fo
         ur"),
        _ => println!("else"),
    }


    

}
