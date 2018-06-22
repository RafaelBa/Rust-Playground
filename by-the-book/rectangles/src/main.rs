#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width &&
            self.height >= other.height
    }
}

impl Rectangle {
    fn i_am_in_a_seperate_impl(&self) -> String {
        format!("I am coming from another block ({:#?})", self)
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!(
        "The area of the reactangle is {} square pixels",
        area(&rect1)
    );

    println!("rect1 is {:?}", rect1);   // rect1 is Rectangle { width: 30, height: 50 }
    println!("rect1 is {:#?}", rect1);  // rect1 is Rectangle {
                                        //   width: 30,
                                        //   height: 50
                                        // }

    println!(
        "Calling area method of rect1: {}",
        rect1.area()
    );

    println!(
        "Can rect1 hold rect2? {}",
        rect1.can_hold(&rect2)
    );

    println!(
        "Can rect1 hold rect3? {}",
        rect1.can_hold(&rect3)
    );

    let square1 = Rectangle::square(400);

    println!(
        "Square via 'Rectangle::square': {:#?}",
        square1
    );

    println!(
        "Calling method from another impl block: {}",
        square1.i_am_in_a_seperate_impl()
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
