#[repr(C)]
struct Book {
    title: String,
}

#[repr(C)]
struct Laptop {}

#[repr(C)]
struct Backpack {
    laptop: Option<Laptop>,
    books: Vec<Book>,
}

impl Backpack{
    fn insert_laptop(&mut self, laptop: Laptop) {
        match self.laptop {
            Some(_) => println!("There's already a laptop here!"),
            None => self.laptop = Some(laptop),
        }
    }
    fn remove_laptop(&mut self) -> Option<Laptop> {
        match self.laptop {
            Some(laptop) => {
                self.laptop = None;
                return laptop;
            },
            None => return None,
        }
    }
    fn insert_book(&mut self, book: Book) {
        self.books.push(book);
    }
    fn remove_books_named(&mut self, title: String) -> Vec<Book> {
        let books = Vec<Book>;
        for (i, book) in self.books.iter().enumerate() {
            if book.title == title {
                books.push(self.books.remove(i));
            }
        }
        return books;
    }
}


pub fn insert_laptop(backpack: Backpack, laptop: Laptop) {
    backpack.insert_laptop(laptop);
}
pub fn remove_laptop(backpack: Backpack) -> Laptop {
    backpack.remove_laptop();
}
pub fn airport_security_check(backpack: Backpack) -> bool {
    match backpack.laptop {
        Some(laptop) => {
            println!("Folks, please remove all laptops from bags and place in the bins!");
            return false
        },
        None => return true,
    }
}
pub fn get_some_work_done(maybe_laptop: Option<Laptop>) {
    match maybe_laptop {
        Some(laptop) => println!("Clickity-clack-click-click"),
        None => println!("Um... where's my laptop?"),
    }
}
