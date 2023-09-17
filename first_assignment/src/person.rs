#[derive(Debug)]
pub struct Person{
    pub name: String,
    pub age: usize
}

#[derive(Debug, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub is_available: bool
}
#[derive(Debug)]
pub struct Library {
    pub books: Vec<Book>
}