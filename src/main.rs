// Uninitialized var in rust
// more about this : https://doc.rust-lang.org/std/mem/union.MaybeUninit.html

use std::mem::{self, MaybeUninit};

fn main() {
    let x: i32 = unsafe { mem::uninitialized() };
    let y: i32 = unsafe { MaybeUninit::uninit().assume_init() };

    let mut m = MaybeUninit::<&i32>::uninit();
    m.write(&0);
    let m = unsafe { m.assume_init() };

    unsafe fn make_vector(out: *mut Vec<i32>){
        unsafe {out.write(vec![1,2,3])};
    }

    let mut v = MaybeUninit::uninit();
    unsafe {make_vector(v.as_mut_ptr());}

    let v  = unsafe { v.assume_init()};
    assert_eq!(&v,&[1,2,3]);

    println!("Vector: {:?}",v);
    println!("{x} and {y} and {m}");
}
