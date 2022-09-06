use std::f32::consts::E;

const MAX_POINTS:u32 = 1000_000000;

/* 
多行注释
*/
// This is main function
fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is {}", x);

    let space = "    ";
    let space = space.len();

    println!("{}", space);

    let guess: u32 = "42".parse().expect("not a number");

    println!("{}", guess);

    let x = 2.0;
    
    let y: f32 = 3.0;

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let reminder = 54 % 5;

    let t = true;
    let f: bool = false;
    // Tuple
    let tup: (i32, f64 ,u8) = (500, 2.3, 10);
    println!("The value of tuple is {} {} {}", tup.0, tup.1, tup.2);
    let (x, y ,z) = tup;
    println!("{}, {}, {}", x, y,z);

    // 数组
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];
    let a1 = a[0];

    another_function(10); // argument

    let y = {
        let x = 1;
        x + 3
    };
    println!("The value of y is {}", y);

    let x = five(4);
    println!("The value of x is {}", x);

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0{
        println!("number is divisible by 2");
    } else {
        println!("number isn's divisible by 4, 3, 2");
    }

    let condition = true;
    let number = if condition {5} else{6};
    println!("The value of number is {}", number);

    loop {
        println!("again");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; 
        }
    };
    println!("The value of result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!");

    let a = [1, 2, 3, 4, 5, 6, 7, 8];
    for element in a.iter() {
        println!("the value is : {}", element);
    }

    // Range  rev
    for number in (1..4).rev() {
        println!("{}", number)
    }
    println!("LIFTOFF");
}

fn another_function(x: i32) { // parameter
    println!("Hello {}", x);
}

fn five(x: i32) -> i32 {
    x + 5
}