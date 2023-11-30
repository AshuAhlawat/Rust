fn main() {
    first()
}

fn first() {
    let y = second();

    print!("{y} {}",third())
}

fn second() -> &'static str {
    "Hello"
}

fn third() -> i32 {
    69
}