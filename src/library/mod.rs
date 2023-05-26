pub struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// Implement the methods below. Update the `self` parameter to
// indicate the method's required level of ownership over the object:
//
// - `&self` for shared read-only access,
// - `&mut self` for unique and mutable access,
// - `self` for unique access by value.
impl Library {
    pub fn new() -> Library {
        todo!("Initialize and return a `Library` value")
    }

    //fn len(self) -> usize {
    //    todo!("Return the length of `self.books`")
    //}

    //fn is_empty(self) -> bool {
    //    todo!("Return `true` if `self.books` is empty")
    //}

    //fn add_book(self, book: Book) {
    //    todo!("Add a new book to `self.books`")
    //}

    //fn print_books(self) {
    //    todo!("Iterate over `self.books` and each book's title and year")
    //}

    //fn oldest_book(self) -> Option<&Book> {
    //    todo!("Return a reference to the oldest book (if any)")
    //}
}
