fn main() {
    let rect = build_rect(30, 50);
    // println!("The area of the rectangle is {} square pixels", area(&rect));  
    println!("The area of the rectangle is {} square pixels", rect.area());
    // println!("The rect is {:?}", rect); 
    let rect2 = build_rect(25, 45); 
    let rect3 = build_rect(35, 45); 
    println!("can rect hold rect2? {}", rect.can_hold(&rect2)); 
    println!("can rect2 hold rect3? {}", rect.can_hold(&rect3)); 
}

// #[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn build_rect(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
}

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

impl Rectangle {
    // constructor
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}