fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("The first word is: {}", word);    

    let a = [1,2,3,4,5,6,7,8,9];
    let slice = &a[1..3];
    println!("The slice is: {:?}", slice);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
