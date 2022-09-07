#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32,
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
    
    println!("{}", area1(&rect));

    println!("{:#?}", rect);
}

fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

fn area1(rect: &Reactangle) -> u32 {
    rect.width * rect.height
}