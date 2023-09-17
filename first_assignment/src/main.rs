mod person;
use crate::person::Library;
use crate::person::Person;
use crate::person::Book;
fn main() {
    
    let person = Person{
        name: String::from("rameez"),
        age: 23
    };

    println!("{:#?}",person);
    let book: Book = Book{
        title: String::from("The girl with a dragon tattoo"),
        author: String::from("Stieg Larrson"),
        is_available: false
    };
    let book1: Book = Book{
        title: String::from("Conspiracy"),
        author: String::from("Ryan Holiday"),
        is_available: true
    };
    let book2: Book = Book{
        title: String::from("Swift"),
        author: String::from("apple"),
        is_available: true
    };


    let library: Library = Library{
        books: [book, book1, book2].to_vec()
    };

    for book in &library.books {
        println!("{:#?}", book);
    }
    
}