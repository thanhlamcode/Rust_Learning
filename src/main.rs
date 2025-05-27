use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let sec_ret = rand::thread_rng().gen_range(1..=10);

    println!("Input your guess:");

    loop {
        println!("Please input your guess:");

        let mut guess_str = String::new(); // Tạo mới chuỗi tại mỗi vòng lặp

        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");

        let guess: i32 =  guess_str.trim().parse().expect("Please type a number!");

        match guess.cmp(&sec_ret) {
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
        }
    }
}
