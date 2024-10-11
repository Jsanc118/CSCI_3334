use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
   let mut file = File::create("books.txt").unwrap(); // TODO: Implement this function
  for Book in books
    { 
         writeln!(file, "{},{},{}", Book.title, Book.author, Book.year).unwrap(); 
    }// Hint: Use File::create() and write!() macro
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).expect("Failed to open file");// TODO: Implement this function
    let reader = BufReader::new(file);
    
    let mut books : Vec<Book> = vec![];

    for line in reader.lines()
    {
        let line = line.unwrap();
   let mut parts = line.split(',');
   let title = parts.next().unwrap().to_string();
   let author = parts.next().unwrap().to_string();
   let year_str = parts.next().expect("Missing Year").trim();
   let year = year_str.parse::<u16>().expect("Failed to parse year");
   books.push(Book {title, author, year});         
}
return books;
 // Hint: Use File::open() and BufReader
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}