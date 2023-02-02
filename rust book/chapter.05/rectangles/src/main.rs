fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    ); 
    
    let rect2 = (40,5);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect2)
    );

    let rect3 = Rectangle {
      width: 30,
      height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect3)
    );

    println!("rect3 is {:?}", rect3);
    println!("rect3 is {:#?}", rect3);

    let scale = 3;
    let rect4 = Rectangle{
        width: dbg!(30*scale),
        height: 50,
    };

    dbg!(&rect4);

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}