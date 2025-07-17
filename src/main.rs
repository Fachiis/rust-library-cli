use library_cli::{print_help, Borrowable, Library};
use std::io::{self, Write};
use colored::Colorize;

pub fn run<R: io::Read, W: io::Write>(mut stdin: R, mut stdout: W) -> io::Result<()> {
    writeln!(stdout, "\nWelcome to the Book Library CLI project\n")?;
    print_help(&mut stdout)?;
    let mut library = Library::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut user_input = String::new(); // Define a mutable String to hold user input
        let lines = io::stdin().read_line(&mut user_input);

        match lines {
            Ok(_) => {
                let args: Vec<&str> = user_input.trim().split_whitespace().collect();

                if args.is_empty() {
                    continue;
                }

                match args[0].to_lowercase().as_str() {
                    "quit" | "q" => {
                        println!("Goodbye!");
                        break;
                    }
                    "help" | "h" => {
                        print_help(&mut stdout)?;
                    }
                    "add" => {
                        if args.len() < 3 {
                            println!("{}", "Usage: add <title> by <author>".yellow());
                            continue;
                        }

                        let by_index = args.iter().position(|&x| x == "by");

                        if let Some(index) = by_index {
                            let title = args[1..index].join(" ");
                            let author = args[index + 1..].join(" ");

                            match library.add_book(&title, &author) {
                                Ok(_) => {
                                    println!("{}", format!("Book '{}' by '{}' added successfully.", title, author).green());
                                }
                                Err(e) => {
                                    eprintln!("{}", format!("Error adding book: {}", e).red());
                                }
                            }

                        } else {
                            println!("Usage: add <title> by <author>");
                            continue;
                        }

                    }
                    "borrow" => {
                        if args.len() < 2 {
                            println!("{}", "Usage: borrow <title>".yellow());
                            continue;
                        }

                        let title = args[1..].join(" ");

                        match library.borrow_item(&title) {
                            Ok(_) => {
                                println!("{}", format!("You have borrowed '{}'.", title).green());
                            }
                            Err(e) => {
                                eprintln!("{}", format!("Error borrowing book: {}", e).red());
                            }
                        }
                    }
                    "return" => {
                        if args.len() < 2 {
                            println!("{}", "Usage: return <title>".yellow());
                            continue;
                        }

                        let title = args[1..].join(" ");

                        match library.return_borrowed_item(&title) {
                            Ok(_) => {
                                println!("{}", format!("You have returned '{}'.", title).green());
                            }
                            Err(e) => {
                                eprintln!("{}", format!("Error returning book: {}", e).red());
                            }
                        }
                    }
                    "list" | "ls" => {
                        let books = library.list_available_books();
                        match books {
                            Ok(available_books) => {

                                    println!("Available books:");
                                    for book in available_books {
                                        println!(" - {}", book);
                                    }

                            }
                            Err(e) => {
                                eprintln!("{}", format!("{}", e).bright_blue());
                            }
                        }
                    }
                    _ => {
                        eprintln!("{}", format!("Unknown command: '{}'", args[0]).red());
                        eprintln!("{}", "Type 'help' for a list of commands.".yellow());
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", format!("Error reading input: {}", e).red());
                continue;
            }
        }
    }

    Ok(())
}

fn main() {
    run(io::stdin(), io::stdout()).unwrap_or_else(|e| eprintln!("Error: {}", e));
}
