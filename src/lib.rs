use std::{fmt, io};
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Book {
    title: String,
    author: String,
    available: bool,
}

pub struct Library {
    books: Vec<Book>,
}

// A rulebook for borrowing (Trait)
// It makes your code more flexible, reusable, and easier to extend, especially if you want to add new types of borrowable items later. Think of it as creating a universal lending system in your toy workshop, rather than hardcoding rules just for one type of toy.
pub trait Borrowable {
    fn borrow_item(&mut self, title: &str) -> Result<(), Error>;
    fn return_borrowed_item(&mut self, title: &str) -> Result<(), Error>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    BookNotFound(String),
    BookNotAvailable(String),
    BookAlreadyExists(String),
    BookAlreadyAvailable(String),
    NoAvailableBooks,
    EmptyTitleOrAuthor,
}

impl Display for Book {
    // Overriding the formatter trait for Book to provide a custom string representation
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} by {} - {}",
            self.title,
            self.author,
            if self.available {
                "Available"
            } else {
                "Borrowed"
            }
        )
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::BookNotFound(title) => write!(f, "Book '{}' not found.", title),
            Error::BookNotAvailable(title) => write!(f, "Book '{}' is not available.", title),
            Error::BookAlreadyExists(title) => write!(f, "Book '{}' already exists.", title),
            Error::EmptyTitleOrAuthor => write!(f, "Title or author cannot be empty."),
            Error::BookAlreadyAvailable(title) => write!(f, "Book '{}' is already available.", title),
            Error::NoAvailableBooks => write!(f, "No available books in the library."),
        }
    }
}

impl Library {
    pub fn new() -> Library {
        Library {
            books: Vec::new(),
        }
    }

    pub fn add_book(&mut self, title: &str, author: &str) -> Result<(), Error> {
        if title.is_empty() || author.is_empty() {
            return Err(Error::EmptyTitleOrAuthor);
        }

        let book = Book {
            title: title.to_string(),
            author: author.to_string(),
            available: true,
        };

        if self.books.iter().any(|b| b.title == title) {
            return Err(Error::BookAlreadyExists(title.to_string()));
        }

        self.books.push(book);

        Ok(())
    }

    pub fn list_available_books(&self) -> Result<Vec<&Book>, Error> {
        let available_books: Vec<&Book> = self.books.iter().filter(|book| book.available).collect();

        if available_books.is_empty() {
            Err(Error::NoAvailableBooks)
        } else {
            Ok(
                available_books
            )
        }
    }

    pub fn is_library_empty(&self) -> bool {
        self.books.is_empty()
    }
}

impl Borrowable for Library {
    fn borrow_item(&mut self, title: &str) -> Result<(), Error> {
        let book = self.books.iter_mut().find(|book| book.title.to_lowercase() == title.to_lowercase());

        match book {
            Some(b) if b.available => {
                b.available = false;
                Ok(())
            }
            Some(_) => Err(Error::BookNotAvailable(title.to_string())),
            None => Err(Error::BookNotFound(title.to_string())),
        }
    }

    fn return_borrowed_item(&mut self, title: &str) -> Result<(), Error> {
        let book = self.books.iter_mut().find(|book| book.title.to_lowercase() == title.to_lowercase());

        match book {
            Some(b) if !b.available => {
                b.available = true;
                Ok(())
            }
            Some(_) => Err(Error::BookAlreadyAvailable(title.to_string())),
            None => Err(Error::BookNotFound(title.to_string())),
        }
    }
}

pub fn print_help<W: io::Write>(mut stdout: W) -> io::Result<()> {
    writeln!(stdout, "Commands:")?;
    println!("  Type 'quit' or 'q' to exit the calculator");
    println!("  Type 'help' or 'h' to see this help message");
    println!("  'add book_title by author_name' to add a book");
    println!("  'borrow book_title' to borrow a book");
    println!("  'return book_title' to return a book");
    println!("  'list' or 'ls' to list all books");

    Ok(())
}
