struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // {
    //     let r;                // ---------+-- 'a
    //     //          |
    //     {                     //          |
    //         let x = 5;        // -+-- 'b  |
    //         r = &x;           //  |       |
    //     }                     // -+       |
    //     //          |
    //     println!("r: {}", r); //          |
    // }                         // ---------+

    {
        let x = 5; // ----------+-- 'b
                   //           |
        let r = &x; // --+-- 'a  |
                    //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    } // ----------+

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    // &i32        // 引用
    // &'a i32     // 带有显式生命周期的引用
    // &'a mut i32 // 带有显式生命周期的可变引用

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result1 = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result1);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
