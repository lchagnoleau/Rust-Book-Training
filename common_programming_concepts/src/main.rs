fn main() {
    println!("Hello, world!");

    let guess = get_u32_from_str("adad");

    println!("guess = {guess}");
}

fn get_u32_from_str(buffer: &str) -> u32 {
    match buffer.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}
