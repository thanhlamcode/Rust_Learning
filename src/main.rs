fn main() {
    // fn area(rectangle: &Rectangle) -> u32 {
    //     rectangle.width * rectangle.height
    // }
    // 
    // println!("Please enter width:");
    // let mut width_input = String::new();
    // std::io::stdin()
    //     .read_line(&mut width_input)
    //     .expect("Failed to read line");
    // println!("Please enter height:");
    // let mut height_input = String::new();
    // std::io::stdin()
    //     .read_line(&mut height_input)
    //     .expect("Failed to read line");
    // let width: u32 = width_input.trim().parse().expect("Please enter width");
    // let height: u32 = height_input.trim().parse().expect("Please type a number");
    // 
    // let rectangle = Rectangle { width, height };
    // 
    // println!("Area is {}", area(&rectangle));
    // 
    // println!("{:#?}", rectangle);
    // 
    // let circle = Circle { radius: 1.4 };
    // 
    // println!("Area is {}", circle.area());
    // println!("Chu vi is {}", circle.cv());
    // 
    // let rectangle1 = Rectangle {
    //     height: 4,
    //     width: 4,
    // };
    // let rectangle2 = Rectangle {
    //     height: 4,
    //     width: 6,
    // };
    // let rectangle3 = Rectangle {
    //     height: 3,
    //     width: 7,
    // };
    // let rectangle4 = Rectangle {
    //     height: 5,
    //     width: 9,
    // };
    // 
    // println!(
    //     "Can rectangle1 hold rectangle2: {}",
    //     rectangle1.can_hold(&rectangle2)
    // );
    // 
    // println!(
    //     "Can rectangle1 hold rectangle2: {}",
    //     rectangle3.can_hold(&rectangle4)
    // );
    // 
    // let rectangle5 = Rectangle::square(4);
    // 
    // println!("Rectangle5 is {:#?}", rectangle5);
    
    
    
    let merchant1 = MerchantSetting{
        ip: IpAdress::V4,
        name: String::from("Day la may 1"),
        price: 546
    };
    
    println!("Merchant1: {:?}", merchant1)
}

#[derive(Debug)]
enum IpAdress {
    V4,
    V6,
}

#[derive(Debug)]
struct MerchantSetting{
    ip: IpAdress,
    name: String,
    price: u32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct Circle {
    radius: f32,
}

impl Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * 3.14
    }

    fn cv(&self) -> f32 {
        2.0 * 3.14 * self.radius
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Self {
            width: size,
            height: size,
        }
    }
}
