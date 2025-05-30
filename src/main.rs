use guessing_game::phone;
use unicode_segmentation::UnicodeSegmentation;


fn main() {
    // phone::list_phone();

    let s = String::from("Hôm nay là thứ ");

    let x = s.chars().nth(6).unwrap();
    let y = s.bytes().nth(6).unwrap();
    println!("{}", x);
    println!("{}", y);
}
