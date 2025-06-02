// use std::fs::File;

fn main() {
    // fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    //     if b == 0 {
    //         return Err("Lỗi: Không thể chia cho 0".to_string());
    //     }
    //
    //     Ok(a / b)
    // }
    //
    // match safe_divide(45, 0) {
    //     Ok(n) => println!("{}", n),
    //     Err(msg) => println!("{}", msg),
    // }
    //
    // fn bmi_calculator(height: f32, weight: f32) -> Result<f32, String> {
    //     if height == 0.0 {
    //         return Err("Loi".to_string());
    //     }
    //     Ok(weight / (height * height))
    // }
    //
    // let bmi = bmi_calculator(60.0, 1.8);
    // match bmi {
    //     Ok(n) => println!("{}", n),
    //     Err(msg) => println!("{}", msg),
    // }

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        println!("{} and {}", number, largest);
        if number > largest {
            largest = number;
        }
        println!("Largest number is {}", largest);
        println!("========================");
    }

    println!("{}", largest);
}
