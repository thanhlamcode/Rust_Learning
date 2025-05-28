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
    
    println!("{:#?}", rectangle)
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
