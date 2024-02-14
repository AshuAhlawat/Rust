fn main() {
    let tab_5 = [5, 10, 15, 20, 25];
    // mutable, should have same datatype and size of array immutable

    let days : [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

    let big_arr = [10; 10];


    println!("{}",tab_5[2]);

    for i in tab_5 {
        println!("{i}");
    };

    for i in 0..tab_5.len() {
        println!("{}",(i+1)*5);
    };

    for day in days {
        println!("{}", day)
    }

    for c in days[days.len() - 1].chars() {
        println!("{c}")
    };

    for a in big_arr {
        println!("{}",a)
    }
}
