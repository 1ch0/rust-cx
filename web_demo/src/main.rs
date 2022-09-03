#![feature(proc_macro_hygiene, decl_macro)]
// 我们需要使用 rocket 中定义的路由创建宏 `routes!`
#[macro_use]
extern crate rocket;
#[get("/")]
fn index() -> String {
    "Hello, Rocket".to_string()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}