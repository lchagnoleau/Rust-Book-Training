use std::io;

struct Rectangle {
    a: u32,
    b: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            a: size,
            b: size,
        }
        /* That alos works:
        Rectangle {
            a: size,
            b: size,
        }        
         */
    }

    fn area(&self) -> u32 {
        self.a * self.b
    }

    fn can_hold(&self, rec: &Rectangle) -> bool {
        self.area() > rec.area()
    }
}

fn main() {
    println!("Enter the 2 side of the rectangle you want to know the area:");

    let rec1 = Rectangle {
        a: get_rectangle_side("a"),
        b: get_rectangle_side("b"),
    };

    let rec2 = Rectangle {
        a: get_rectangle_side("a"),
        b: get_rectangle_side("b"),
    };

    println!("Can rect1 hold rect2? {}", rec1.can_hold(&rec2));

    let rec_square = Rectangle::square(10);
    println!("New rectangle square with area of {}", rec_square.area());
}

fn get_rectangle_side(side_name: &str) -> u32 {
    loop {
        println!("Enter size for side {side_name}:");

        let mut size_str = String::new();
        io::stdin()
            .read_line(&mut size_str)
            .expect("Fail to read line");

        match size_str.trim().parse::<u32>() {
            Ok(size) => return size,
            Err(_) => continue,
        };
    }
}
