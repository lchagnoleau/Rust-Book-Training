fn main() {
    let my_string = String::from("Hello World");

    let first_word_of_string = first_word(&my_string);

    println!("first word = {first_word_of_string}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' ' 
        {
            return &s[..i];
        }
    }

    return &s[..];
}
