trait Area {
    fn area(&self) -> u32;
}

struct Square(u32);

impl Area for Square {
    fn area(&self) -> u32 {
        self.0.pow(2)
    }
}

impl Square {
    fn new(side: u32) -> Self {
        Self(side)
    }
}

struct Rectangle(u32, u32);

impl Area for Rectangle {
    fn area(&self) -> u32 {
        self.0 * self.1
    }
}

impl Rectangle {
    fn new(a: u32, b: u32) -> Self {
        Self(a, b)
    }
}

fn comparison_area(a: impl Area, b: impl Area) -> bool {
    a.area() == b.area()
}

fn main() {
    let my_square = Square::new(5);
    let my_rectangle = Rectangle::new(5, 5);
    if comparison_area(my_square, my_rectangle) {
        println!("面積は等しい");
    } else {
        println!("面積は異なる");
    }
}
