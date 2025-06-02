use std::fs::File;

fn main() {
    fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            return Err("Lỗi: Không thể chia cho 0".to_string());
        }

        Ok(a / b)
    }

    match safe_divide(45, 0) {
        Ok(n) => println!("{}", n),
        Err(msg) => println!("{}", msg),
    }

    fn bmi_calculator(height: f32, weight: f32) -> Result<f32, String> {
        if height == 0.0 {
            return Err("Loi".to_string());
        }
        Ok(weight / (height * height))
    }

    let bmi = bmi_calculator(60.0, 1.8);
    match bmi {
        Ok(n) => println!("{}", n),
        Err(msg) => println!("{}", msg),
    }
}
