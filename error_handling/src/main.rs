use std::{
    fmt::Error,
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(path);

    let mut username_file = match username_file_result {
        Ok(f) => f,
        Err(err) => return Err(err),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => Err(err),
    }
}

fn read_username_from_file_shorter(path: &str) -> Result<String, io::Error> {
    // let mut username_file = File::open(path)?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // or

    // let mut username = String::new();
    // File::open(path)?.read_to_string(&mut username)?;
    // Ok(username)

    // or 

    fs::read_to_string(path)
}


fn main() {
    let mut file = match File::open("hello.txt") {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(err) => panic!("Can't create file: {:?}", err),
            },
            other_err => {
                panic!("Problem opening file: {:?}", other_err);
            }
        },
    };

    let mut contents = String::new();

    file.read_to_string(&mut contents);

    println!("file contents is : {}", contents);

    let username = read_username_from_file("./username.txt").unwrap();
    let username_other_method = read_username_from_file_shorter("./username.txt").unwrap();

    println!("username = {}", username);
    println!("username = {}", username_other_method);
}
