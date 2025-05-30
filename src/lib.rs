mod catalog;

// Re-export module để người dùng bên ngoài có thể gọi: library::books::list_books()
pub use catalog::books;
pub use catalog::authors;

pub fn run_system() {
    books::list_books();
    authors::list_authors();
}
