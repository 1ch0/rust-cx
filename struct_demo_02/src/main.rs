#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32,
}

// Rust 根据情况自动添加 & 、&mut 或 *,以便 Object 可以匹配方法的签名
impl Reactangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Reactangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Reactangle {
        Reactangle { width: size, height: size }
    }
}

fn main() {
    // let w = 30;
    // let h = 30;

    // println!("{}", area(w, h);   

    let rect = (30, 50);
    println!("{}", area(rect));
    
    let rect = Reactangle{
        width: 30,
        height: 30,
    };

    let s = Reactangle::square(33);
    println!("{}",s.area());
    let rect1 = Reactangle{
        width: 40,
        height: 40,
    };
    
    println!("{}", rect.area());
    println!("{}", rect1.can_hold(&rect));

    println!("{:#?}", rect);
}

fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

fn area1(rect: &Reactangle) -> u32 {
    rect.width * rect.height
} 