#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.height <= self.height && rect.width <= self.width 
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self { height: size, width: size }
    }
}

pub fn main_structs_example() {
    // let width_1 = 30;
    // let height_1 = 50;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect_2 = Rectangle {
       width: 50,
       height: 50,
    };

    let rect3 = Rectangle {
       width: 60,
       height: 45,
    };

    let rect_square = Rectangle::square(58);

    // let rectangle_1 = (40, 90);

    // println!("The area of the rectangle is {} square pixels.", area(width_1, height_1));
    // println!("The area of the rectangle is {} square pixels.", area_imprvd(rectangle_1));
    // println!("The area of the rectangle is {} square pixels.", area_struct(&rect_2));
    // println!("The area of the rectangle is {} square pixels.", rect_2.area());
    println!("Can rect2 hold rect1 ? {}", rect_2.can_hold(&rect1));
    println!("Can rect3 hold rect2 ? {}", rect3.can_hold(&rect_2));
    // println!("rect_2 is {:#?}", rect_2);
    // dbg!(&rect_2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_imprvd(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
