fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index:");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Failed to parse integer");
    
    println!("Value is: {}", a[index]);
}
