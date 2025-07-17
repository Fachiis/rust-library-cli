use library_cli::Library;

#[test]
fn test_new_library() {
    let library = Library::new();
    assert!(library.is_library_empty(), "Library should be empty on creation");
}

#[test]
fn test_add_book() {
    let mut library = Library::new();
    let result = library.add_book("The Rust Programming Language", "Steve Klabnik and Carol Nichols");
    assert!(result.is_ok(), "Book should be added successfully");

    // Check if the book is in the library
    assert!(!library.is_library_empty(), "Library should not be empty after adding a book");
}

#[test]
fn test_list_available_books() {
    let mut library = Library::new();
    library.add_book("The Rust Programming Language", "Steve Klabnik and Carol Nichols");
    
    let output = library.list_available_books();
    assert_eq!(output.unwrap().len(), 1, "There should be one available book listed");
}