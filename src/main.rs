fn main() {
    println!("Hello, world!");

    let mut number = String::new();
    
    println!("Please enter a number:");

    std::io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    
    let number: u32 = number.trim().parse().expect("Please type a number!");
    
    div(number);
}

fn test() {
    println!("Day la ham test");
}

fn test2() -> i32 {
    return 5;
}

fn ifelse(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn div(number: u32) {
    if number % 2 == 0 {
        println!("number is divisible by 2");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 5 == 0 {
        println!("number is divisible by 5");
    } else {
        println!("number is not divisible by 2, 3 or 5");
    }
}
