// Lifetimes are also needed when structs hold references.

// TODO: Fix the compiler errors about the struct.
struct Book <'b, T>{
    author: &'b str,
    title: &'b str,
    some: T
}

fn main() {
    let book: Book<'_, i32> = Book {
        author: "George Orwell",
        title: "1984",
        some: 123
    };

    println!("{} by {}", book.title, book.author);
}
