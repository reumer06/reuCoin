use std::cell::RefCell;

// Refcell is for mutation even through immutable ref.
fn main() {
    let data = RefCell::new(5);

    let borrow = data.borrow();
    println!("{}",*borrow);

    {
        let mut mut_borrowed = data.borrow_mut();
        *mut_borrowed += 1;
    }

    println!("{}",data.borrow());
}