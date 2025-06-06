use std::fmt::Display;
use std::ops::Deref;

// Định nghĩa smart pointer MyBox<T>
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Triển khai trait Deref để có thể dùng toán tử *
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    // Dùng *y để truy cập giá trị bên trong MyBox
    assert_eq!(5, *y);

    println!("Giá trị y trỏ đến là: {}", *y); // In ra: Giá trị y trỏ đến là: 5

}
