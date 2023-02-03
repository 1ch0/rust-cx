fn main() {
    // panic!("Hello, world!");

    // 缓冲区溢出
    let v = vec![1, 2, 3];

    v[99];
}
