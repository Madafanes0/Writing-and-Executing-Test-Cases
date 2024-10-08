// src/lib.rs

// Implementation code
pub struct Library {
    books: Vec<Book>,
}

pub struct Book {
    title: String,
    author: String,
    available: bool,
}

impl Library {
    pub fn new() -> Self {
        Library { books: Vec::new() }
    }

    pub fn add_book(&mut self, title: String, author: String) {
        if title.is_empty() || author.is_empty() {
            return;
        }
        let book = Book {
            title,
            author,
            available: true,
        };
        self.books.push(book);
    }

    pub fn contains_book(&self, title: &str) -> bool {
        self.books.iter().any(|book| book.title == title)
    }

    pub fn check_out_book(&mut self, title: String, _user: String) -> bool {
        if let Some(book) = self.books.iter_mut().find(|book| book.title == title && book.available) {
            book.available = false;
            return true;
        }
        false
    }
}

// Test code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_book_success() {
        let mut library = Library::new();
        library.add_book("1984".to_string(), "George Orwell".to_string());
        assert!(library.contains_book("1984"));
    }

    #[test]
    fn test_add_book_missing_title() {
        let mut library = Library::new();
        library.add_book("".to_string(), "George Orwell".to_string());
        assert!(!library.contains_book(""));
    }

    #[test]
    fn test_check_out_book_success() {
        let mut library = Library::new();
        library.add_book("1984".to_string(), "George Orwell".to_string());
        let result = library.check_out_book("1984".to_string(), "User1".to_string());
        assert!(result);
    }

    #[test]
    fn test_check_out_book_not_available() {
        let mut library = Library::new();
        library.add_book("1984".to_string(), "George Orwell".to_string());
        library.check_out_book("1984".to_string(), "User1".to_string());
        let result = library.check_out_book("1984".to_string(), "User2".to_string());
        assert!(!result); // Book is already checked out
    }
    #[test]//book that does not exist
    fn test_check_out_non_existent_book() {
        let mut library = Library::new();
        let result = library.check_out_book("Nonexistent Book".to_string(), "User1".to_string());
        assert!(result, "Should not be able to check out a non-existent book");
    }
}
