fn main() {
    // s 不可用
    let  s = "Hello "; // s 可用
                    // 可以对 s 进行相关操作


    let mut s = String::from("Hello");

    s.push_str(", World!");

    println!("{}", s);

    // shadow copy => move 
    // 二次释放
    let s1 = String::from("World!");
    let s2 = s1;

    // println!("{}", s1);
    println!("{}", s2);

    // deep copy => clone()
    let s1 = String::from("World!");
    let s2 = s1.clone();
    println!("{}", s1);

    // 对于 int 类型深拷贝与浅拷贝没区别，编译时确定大小，存放在栈上
    // 复制  Copy trait， u32 bool char f64 Tuple(所有字段都是 Copy 就可 Copy)

    let s = String::from("World!");
    take_ownership(s);

    let x = 5;
    make_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();

    let s2 = String::from("World!");

    let s3 = take_and_gives_back(s2);

    // 引用  不可变
    // 把引用作为函数参数的行为叫做借用
    // 可变引用不可以超过一个，避免数据竞争问题
    // 不可以同时拥有可变引用和不可变引用
    let mut s1 = String::from("引用");
    let len = variants_to_stack(&mut s1);
    println!("The length of '{}' is {}", s1, len);

    let mut s = String::from("World!");
    {
        let s1 = &mut s;
    }

    let s2 = &mut s;

    // 悬空引用 

    // 字符串切片

    let s = String::from("Hello World!");
    let hello = &s[..5];
    let world = &s[6..];
    let whole = &s[..];
    println!("{}: {} => {}", hello, world, whole);

    let mut s = String::from("Hello World!");
    let wordIndex = first_word(&s);

    // s.clear();
    println!("{}", wordIndex);

    // &str 使用
    let my_string = String::from("Hello World!");
    let wordIndex = first_word(&my_string[..]);

    let my_string_literal = ("Hello World!");
    let wordIndex = first_word(my_string_literal);
} // s 作用域到此结束， s 不再可用

fn take_ownership(some_string: String) {
    println!("{}", some_string)
}

fn make_copy(some_number: i32) {
    println!("{}", some_number)
}

fn gives_ownership() -> String {
    let some_string = String::from("test");
    some_string
}

fn take_and_gives_back(a_string: String) -> String {
    a_string
}

fn variants_to_stack(s: &mut String) -> usize {
    s.push_str(", 加 mut 可变");
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
   &s[..]
}