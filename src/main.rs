fn main() {
    let mut s = String::from("Hello my name is ");
    let word = first_word(&s);

    s.push_str("eatnows");
    println!("{}", word);

    println!("{}", s);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..index];
        }
    };

    &s[..]
}