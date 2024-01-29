struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct Colour(u8, u8, u8);


fn intensity(color: &Color) -> f32 {
    let i = (color.red as f32 + color.blue as f32 + color.green as f32)/3.0;

    i / 255.0
}

fn intensity2(color: &Colour) -> f32 {
    let i = (color.0 as f32 + color.1 as f32 + color.2 as f32)/3.0;

    i / 255.0
}

fn main() {
    let mut bg = Color { red: 23, blue: 200, green: 100 };

    println!("{}", intensity(&bg) );

    bg.red = 50;

    println!("{}", intensity(&bg));

    let mut bg2 = Colour(23, 200, 100);

    println!("{}", intensity2(&bg2) );
}