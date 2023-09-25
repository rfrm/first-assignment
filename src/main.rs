mod library;
use library::{show_person, Book, Library, Person};

fn main() {
    let robinson = Person {
        name: "Robinson Rodriguez",
        age: 31,
    };
    let nayelis = Person {
        name: "Nayelis Bolivar",
        age: 21,
    };
    show_person(&robinson);

    let mut book1 = Book::create("Cien años de soledad", "Gabriel Garcia Marquez");
    let mut book2 = Book::create(
        "The Rust programming language",
        "Steve Klabnik & Carol Nichols",
    );

    let mut library = Library::create();
    library.push_book(&mut book1);
    library.push_book(&mut book2);
    library.show_available();

    library.chekout_book("Building Microservices", &robinson);
    library.chekout_book("The Rust programming language", &robinson);
    library.chekout_book("Cien años de soledad", &nayelis);
    library.chekout_book("Cien años de soledad", &robinson);

    library.show_borrowed();
    library.return_book("The Rust programming language");
    library.show_available();
}
