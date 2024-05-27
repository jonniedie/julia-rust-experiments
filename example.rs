
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct COption<T> {
    is_valid: bool,
    value: T,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CVector<T> {
    ptr: *mut T,
    len: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Laptop {
    mass_kg: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Pen {
    mass_kg: f64,
}

#[derive(Default)]
struct Backpack {
    laptop: Option<Laptop>,
    // pens: Vec<Pen>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CBackpack {
    laptop: COption<Laptop>,
    // pens: CVector<Pen>,
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
    // fn to_cbackpack(mut self) -> CBackpack { // uncomment when pens
    fn to_cbackpack(self) -> CBackpack {
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
            laptop: COption { is_valid: has_laptop, value: raw_laptop },
            // pens: CVector { ptr: self.pens.as_mut_ptr(), len: self.pens.len() },
        }
    }
}

impl CBackpack {
    fn to_backpack(self) -> Backpack {
        return Backpack {
            laptop: if self.laptop.is_valid {Some(self.laptop.value)} else {None},
            // pens: unsafe { Vec::from_raw_parts(self.pens.ptr, self.pens.len, self.pens.len) },
        }
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
            return COption { is_valid: true, value: laptop}
        },
        None => {
            let dummy_laptop: Laptop = Default::default();
            return COption { is_valid: false, value: dummy_laptop }
        },
    }
}

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
            let all_clear = true;
            return all_clear
        },
    }
}
