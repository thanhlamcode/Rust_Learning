fn main() {

    //1. Mỗi giá trị trong Rust có một chủ sở hữu (owner).
    let s = String::from("Hello, world!"); // s là chủ sở hữu của chuỗi "Hello, world!"
    println!("{}", s); // Sử dụng s
    
    // 2. Chỉ có một chủ sở hữu tại một thời điểm.
    let s1 = String::from("Hello, world!");
    let s2 = s1; // s1 chuyển quyền sở hữu sang s2

    println!("{}", s2); // s2 sở hữu chuỗi này, không còn s1
    // println!("{}", s1); // Lỗi biên dịch: `s1` đã bị chuyển nhượng quyền sở hữu, không thể sử dụng lại

    //3. Khi chủ sở hữu ra khỏi phạm vi (scope), giá trị sẽ bị giải phóng (dropped).
    {
        let s = String::from("Hello, world!"); // s là chủ sở hữu của chuỗi này trong phạm vi nhỏ
    } // Khi ra khỏi phạm vi, s bị giải phóng và bộ nhớ bị giải phóng

    // Không thể sử dụng s ngoài phạm vi nó được khai báo
}


// fn condition(){
//     let condition = true;
//
//     let number = if condition { 5 } else { 6 };
//
//     println!("The value of number is: {}", number);
// }

// fn test() {
//     println!("Day la ham test");
// }
//
// fn test2() -> i32 {
//     return 5;
// }
//
// fn ifelse(number: i32) {
//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }
//
// fn div(number: u32) {
//     if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 5 == 0 {
//         println!("number is divisible by 5");
//     } else {
//         println!("number is not divisible by 2, 3 or 5");
//     }
// }
