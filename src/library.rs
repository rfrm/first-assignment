pub struct Person<'a> {
    pub name: &'a str,
    pub age: u32,
}

pub fn show_person(person: &Person) {
    println!("Hi {}!, you are {} years old", person.name, person.age);
}

pub struct Book<'a> {
    pub title: &'a str,
    pub author: &'a str,
    is_available: bool,
    borrowed_by: Option<&'a Person<'a>>,
}

impl<'a> Book<'a> {
    pub fn create(title: &'a str, author: &'a str) -> Self {
        Self {
            title,
            author,
            borrowed_by: None,
            is_available: true,
        }
    }

    fn set_borrowed_by(&mut self, person: &'a Person) {
        self.is_available = false;
        self.borrowed_by = Some(person)
    }

    fn return_book(&mut self) {
        self.is_available = true;
        self.borrowed_by = None;
    }
}

pub struct Library<'a> {
    books: Vec<&'a mut Book<'a>>,
}

impl<'a> Library<'a> {
    pub fn create() -> Self {
        Self { books: Vec::new() }
    }

    pub fn push_book(&mut self, book: &'a mut Book<'a>) {
        self.books.push(book)
    }

    pub fn show_available(&self) {
        println!("+--------------------------------------+");
        println!("+ Title                                +");
        println!("+--------------------------------------+");
        for book in self.books.iter().filter(|book| book.is_available) {
            println!("+ {:<36} +", book.title);
        }
        println!("+--------------------------------------++");
    }

    pub fn show_borrowed(&self) {
        println!("+--------------------------------------+--------------------------------------+");
        println!("+ Title                                + Borrower                             +");
        println!("+--------------------------------------+--------------------------------------+");
        for book in self.books.iter().filter(|book| !book.is_available) {
            if let Some(borrowed_by) = book.borrowed_by {
                println!("+ {:<36} + {:<36} +", book.title, borrowed_by.name);
            }
        }
        println!("+--------------------------------------+--------------------------------------+");
    }

    pub fn chekout_book(&mut self, title: &str, user: &'a Person) {
        if let Some(book) = self
            .books
            .iter_mut()
            .find(|book| book.title == title && book.is_available)
        {
            book.set_borrowed_by(&user)
        } else {
            println!("{} is unnavailable", title);
        }
    }

    pub fn return_book(&mut self, title: &str) {
        if let Some(book) = self.books.iter_mut().find(|person| person.title == title) {
            book.return_book();
        } else {
            println!("{} not found", title);
        }
    }
}
