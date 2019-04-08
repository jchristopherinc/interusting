struct Rectangle {
    width: u32,
    height: u32,
}

fn area_of_rectange(rectange: &Rectangle) -> u32 {
    rectange.height * rectange.width
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("Area of rectange: {}", area_of_rectange(&rect1));
}
