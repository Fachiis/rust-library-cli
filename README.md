# Library CLI

A command-line interface application for managing a virtual library written in Rust.

## Overview

Library CLI is a simple yet powerful command-line tool that allows users to manage a collection of books. The application provides functionality for adding books to the library, borrowing and returning books, and listing available books.

## Features

- Add new books to the library with title and author information
- Borrow books from the library
- Return previously borrowed books
- List all available books in the library
- Simple and intuitive command-line interface with colored output

## Installation

### Prerequisites

- Rust and Cargo (install from [rustup.rs](https://rustup.rs/))

### Building from Source

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/library-cli.git
   cd library-cli
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the executable:
   ```
   ./target/release/library-cli
   ```

## Usage

Once you launch the application, you'll be presented with a prompt `>` where you can enter commands.

### Available Commands

- `add <title> by <author>` - Add a new book to the library
- `borrow <title>` - Borrow a book from the library
- `return <title>` - Return a previously borrowed book
- `list` or `ls` - List all available books in the library
- `help` or `h` - Display the help message
- `quit` or `q` - Exit the application

### Examples

```
> add The Rust Programming Language by Steve Klabnik and Carol Nichols
Book 'The Rust Programming Language' by 'Steve Klabnik and Carol Nichols' added successfully.

> add 1984 by George Orwell
Book '1984' by 'George Orwell' added successfully.

> list
Available books:
 - The Rust Programming Language by Steve Klabnik and Carol Nichols - Available
 - 1984 by George Orwell - Available

> borrow 1984
You have borrowed '1984'.

> list
Available books:
 - The Rust Programming Language by Steve Klabnik and Carol Nichols - Available

> return 1984
You have returned '1984'.
```

## Project Structure

- `src/lib.rs` - Core library functionality including the `Book` and `Library` structs and the `Borrowable` trait
- `src/main.rs` - CLI application entry point and command handling
- `tests/` - Unit tests for the library

## Error Handling

The application provides informative error messages for various scenarios:

- Attempting to borrow a book that doesn't exist
- Attempting to borrow a book that is already borrowed
- Attempting to return a book that is already available
- Attempting to add a book that already exists
- Attempting to add a book with an empty title or author

## Testing

Run the unit tests with:

```
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

