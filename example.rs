use std::ffi;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct COption<T> {
    is_valid: bool,
    value: T,
}

struct Book {
    title: String,
}
#[repr(C)]
pub struct CBook {
    title: ffi::CString,
    // ptr_title: *const u8,
    // len_title: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Laptop {
    weight_kg: f64,
}

#[derive(Default)]
struct Backpack {
    laptop: Option<Laptop>,
    // books: Vec<Book>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CBackpack {
    // has_laptop: bool,
    // raw_laptop: Laptop,
    laptop: COption<Laptop>,
    // ptr_books: *mut Book,
    // num_books: usize,
}

impl Backpack{
    fn insert_laptop(&mut self, laptop: Laptop) {
        match self.laptop {
            Some(_) => println!("There's already a laptop here!"),
            None => {
                self.laptop = Some(laptop);
            },
        }
    }
    fn remove_laptop(&mut self) -> Option<Laptop> {
        match self.laptop {
            Some(laptop) => {
                self.laptop = None;
                return Some(laptop)
            },
            None => return None,
        }
    }
    // fn insert_book(&mut self, book: Book) {
    //     self.books.push(book);
    // }
    // fn remove_book_named(&mut self, title: String) -> Option<Book> {
    //     let mut maybe_book = None;
    //     let mut index_remove = 0;
    //     let mut should_remove = false;
    //     let books = &self.books;
    //     for (index, book) in books.iter().enumerate() {
    //         if book.title == title {
    //             index_remove = index;
    //             should_remove = true;
    //             break
    //         }
    //     }
    //     if should_remove {
    //         maybe_book = Some(self.books.remove(index_remove));
    //     }
    //     return maybe_book;
    // }
    fn to_cbackpack(mut self) -> CBackpack {
        let mut has_laptop = false;
        let mut raw_laptop: Laptop = Default::default();
        match self.laptop {
            Some(laptop) => {
                has_laptop = true;
                raw_laptop = laptop;
            },
            None => (),
        }
        return CBackpack {
            // has_laptop: has_laptop,
            // raw_laptop: raw_laptop,
            laptop: COption { is_valid: has_laptop, value: raw_laptop },
            // ptr_books: self.books.as_mut_ptr(),
            // num_books: self.books.len(),
        }
    }
}

impl CBackpack {
    fn to_backpack(self) -> Backpack {
        // unsafe {
        //     let books: Vec<Book> = Vec::from_raw_parts(
        //         self.ptr_books,
        //         self.num_books.into(),
        //         self.num_books.into(),
        //     );
            return Backpack {
                laptop: if self.laptop.is_valid {Some(self.laptop.value)} else {None},
                // books: books,
            }
        // }
    }
}

#[export_name="insert_laptop"]
pub extern "C" fn insert_laptop(cbackpack: &mut CBackpack, laptop: Laptop) {
    let mut backpack = cbackpack.to_backpack();
    backpack.insert_laptop(laptop);
    *cbackpack = backpack.to_cbackpack();
}

#[export_name="remove_laptop"]
pub extern "C" fn remove_laptop(cbackpack: &mut CBackpack) -> COption<Laptop> {
    let mut backpack = cbackpack.to_backpack();
    let maybe_laptop = backpack.remove_laptop();
    *cbackpack = backpack.to_cbackpack();
    match maybe_laptop {
        Some(laptop) => {
            println!("Got a laptop")
            return COption { is_valid: true, value: laptop}
        },
        None => {
            let dummy_laptop: Laptop = Default::default();
            return COption { is_valid: false, value: dummy_laptop }
        },
    }
}

// #[export_name="remove_book_named"]
// pub extern "C" fn remove_book_named(cbackpack: &mut CBackpack, title: ffi::CString) -> COption<CBook> {
//     let mut backpack = cbackpack.to_backpack();
//     let title_str = title.into_string().unwrap();
//     let maybe_book = backpack.remove_book_named(title_str);
//     *cbackpack = backpack.to_cbackpack();
//     match maybe_book {
//         Some(book) => {
//             let cbook = CBook {
//                 title: ffi::CString::new(book.title).unwrap(),
//                 // ptr_title: book.title.as_bytes().as_ptr(),
//                 // len_title: book.title.len(),
//             };
//             return COption{ is_valid: true, value: cbook }
//         },
//         None => {
//             let dummy_cbook = CBook {
//                 title: ffi::CString::new("").unwrap()
//                 // ptr_title: title_str.as_ptr(),
//                 // len_title: title_str.len(),
//             };
//             return COption { is_valid: false, value: dummy_cbook }
//         },
//     }
// }

#[export_name="airport_security_check"]
pub extern "C" fn airport_security_check(cbackpack: CBackpack) -> bool {
    let backpack = cbackpack.to_backpack();
    match backpack.laptop {
        Some(_) => {
            println!("Folks, please remove all laptops from bags and place in the bins!");
            return false
        },
        None => {
            println!("Step through the detector and pick your items up on the other side.");
            let mut all_clear = true;
            // if backpack.books.iter().any(|book| book.title.to_ascii_lowercase().contains("atomic")) {
            //     all_clear = false;
            //     println!("We're going to need you to step over here.");
            // }
            return all_clear
        },
    }
}
