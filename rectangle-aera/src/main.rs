use std::io;

struct Rectangle {
    a: u32,
    b: u32,
}

fn main() {
    println!("Enter the 2 side of the rectangle you want to know the area:");

    let rectangle = Rectangle {
        a: get_rectangle_side("a"),
        b: get_rectangle_side("b"),
    };

    let rectangle_area = rectangle_area_calculator(&rectangle);
    println!(
        "Rectangle of {} x {} have a area of {rectangle_area}.",
        rectangle.a, rectangle.b
    );
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

fn rectangle_area_calculator(rec: &Rectangle) -> u32 {
    rec.a * rec.b
}
