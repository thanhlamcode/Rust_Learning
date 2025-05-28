fn main() {
    

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    println!("Please enter width:");
    let mut width_input = String::new();
    std::io::stdin()
        .read_line(&mut width_input)
        .expect("Failed to read line");
    println!("Please enter height:");
    let mut height_input = String::new();
    std::io::stdin()
        .read_line(&mut height_input)
        .expect("Failed to read line");
    let width: u32 = width_input.trim().parse().expect("Please enter width");
    let height: u32 = height_input.trim().parse().expect("Please type a number");

    let rectangle = Rectangle {
        width,
        height,
    };
    
    println!("Area is {}", area(&rectangle));
    
    println!("{:#?}", rectangle);
    
    
    let circle = Circle{
        radius: 1.4
    };
    
    println!("Area is {}", circle.area());
    println!("Chu vi is {}", circle.cv())
    
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
        self.radius * self.radius*3.14
    }
    
    fn cv(&self)-> f32{
        2.0*3.14*self.radius
    }
}