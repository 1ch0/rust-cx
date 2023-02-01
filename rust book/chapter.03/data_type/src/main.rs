use std::io;

fn main() {
    //let guess: u32 = "42".parse().expect("Not a number!");
    
    // tuple
    let tup: (i32, f64, u8) = (222, 2.3, 11);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (200, 2.3, 11);
    let two_hundred = x.0;
    let two_point_three = x.1;
    let one_one = x.2;

    // array
    
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // let b= [3; 5];

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}
