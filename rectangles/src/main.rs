fn main() {
    let rect = Rectangle {
        width: 50,
        height: 50
    };
    println!("The area is {} square pixels, rect is {:#?}", rect.area(), rect);
}

impl Rectangle {
 fn area(&self) -> u32 {
     self.width * self.height
 }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}