pub mod catalog;
pub mod user;
mod house;
// Re-export module để người dùng bên ngoài có thể gọi: guessing_game::books::list_books()
pub use catalog::books;
pub use catalog::authors;

pub use house::machine;
pub use house::phone;



pub fn run_system() {
    books::list_books();
    authors::list_authors();
}
