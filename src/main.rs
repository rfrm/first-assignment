struct Person {
    name: String,
    age: u32,
}

struct Book {
    title: String,
    author: String,
    is_available: bool,
}

struct Library {
    books: Vec<Book>,
}

fn main() {
    let robinson = Person {
        name: "Robinson".to_string(),
        age: 31,
    };
    println!("Hi {}!, you are {} years old", robinson.name, robinson.age);

    let book1 = Book {
        title: String::from("Cien anios de soledad"),
        author: String::from("Gabriel Garcia Marquez"),
        is_available: true,
    };
    let book2 = Book {
        title: "The Rust programming language".to_string(),
        author: "Steve Klabnik & Carol Nichols".to_string(),
        is_available: true,
    };
    let mut books: Vec<Book> = Vec::new();
    books.push(book1);
    books.push(book2);
    let library = Library { books };
    for book in library.books {
        if book.is_available {
            println!("{}'s {} is available", book.author, book.title)
        } else {
            println!("{}'s {} is unavailable", book.author, book.title)
        }
    }
}
