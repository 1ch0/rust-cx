fn main() {
    println!("Hello, world!");
    another_function(5, 'h');

    let y = {
        let x = five();
        plus_one(x)
    };
    println!("The value of y is: {}", y);


}

fn another_function(x: i32, h: char) {
    println!("Another function!, value of x: {} h: {}", x, h);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x+1
}