use rand::Rng;
// trait
use std::io; // preclude
use std::cmp::Ordering; // trait

fn main() {
    println!("猜树");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {

        println!("猜测一个数:");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("无法读取行");
    
        // shadow
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("你猜测的数是： {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"), // arm
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!"); 
                break;
            },
        }
    }

}