struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2*(self.length+self.width)
    }

    fn area(&self) -> u32 {
        self.length*self.width
    }
    
    fn is_square(&self) -> bool {
        self.length == self.width
    }
}


fn main() {
    let a = Rectangle{ length: 10, width: 5 };
    
    if a.is_square() {
        println!("Its a Square")
    } else {
        println!("Area: {} \nPerimeter: {}",a.area(), a.perimeter());
    }


}
