use std::fmt::{Display, Formatter, Result};

// Trait do ta định nghĩa
pub trait Summary {
    fn summarize(&self) -> String;
}

// Một struct đơn giản
struct News {
    title: String,
    author: String,
}

// Cài trait Summary cho News
impl Summary for News {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

// Cài trait Display cho News để dùng println!("{}", item)
impl Display for News {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "News(title: {}, author: {})", self.title, self.author)
    }
}

// Hàm nhận một item có cả Summary + Display
pub fn notify(item: &(impl Summary + Display)) {
    println!("--- Notify Start ---");
    println!("Summary: {}", item.summarize());
    println!("Display: {}", item);
    println!("--- Notify End ---");
}

fn main() {
    let article = News {
        title: String::from("Rust conquers the world!"),
        author: String::from("Ferris the Crab"),
    };

    notify(&article);
}
