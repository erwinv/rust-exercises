pub struct Library {
    books: Vec<Book>,
}

pub struct Book {
    pub title: String,
    year: u16,
}

impl Book {
    pub fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

impl Library {
    pub fn new() -> Library {
        Library{ books: vec![] }
    }

    pub fn len(&self) -> usize {
       self.books.len()
    }

    pub fn is_empty(&self) -> bool {
       self.books.len() == 0
    }

    pub fn add_book(&mut self, book: Book) {
       self.books.push(book);
    }

    pub fn print_books(&self) {
       for book in &self.books {
            println!("title: {:?}, year: {:?}", book.title, book.year);
       }
    }

    pub fn oldest_book(&self) -> Option<&Book> {
        if self.books.len() == 0 {
            return None;
        }

        let mut oldest: &Book = &self.books[0];
        for book in &self.books[1..] {
            if book.year < oldest.year {
                oldest = book
            }
        }

        Some(oldest)
    }
}
