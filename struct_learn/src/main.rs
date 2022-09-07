struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {
    let user1 = User {
        username: String::from("user1"),
        email: String::from("user1@gmail.com"),
        sign_in_count: 0,
        active: false,
    };



    let u1 = build_user("test".to_string(), "test".to_string());
    let u2 = User{
        username: String::from("user2"),
        ..user1
    };
    println!("{}",u2.username);

    // Tuple struct
    struct Color(i32, i32);
    struct Point(i32, i32);

    // Unit-Like struct 没有任何字段

    // Struct 数据的所有权
}


fn build_user(email: String, username: String) -> User {
    User { 
        // username: username,
        username, // 简写，名称一致
        // email: email,
        email, 
        active: false,
        sign_in_count: 0,
    }    
}