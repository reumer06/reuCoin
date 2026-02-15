use std::fmt::Debug;
trait ConvertTo<T: Debug> {
    // trait can have generic parameters which then can have trait bounds,
    fn print_a_t(t: T) {
        println!("{:?}", t);
    }
    fn convert(&self) -> T;
}

trait Printable: std::fmt::Display {
    // super trait.
    fn print(&self) {
        println!("{}", self);
    }
}

// trait can have their own associated types and const;
trait Vehicle {
    type Energy;
    const WHEELS: u8;

    fn energy_source(&self) -> Self::Energy;

    fn print_wheels() {
        println!("the vehicle has {} wheels", Self::WHEELS); // when const you use Self instead of self;
    }
}

struct FixedArray<T, const N: usize> {
    data: [T; N],
}

trait ArrayOps<T, const N: usize> {
    fn first(&self) -> Option<&T>;
    fn last(&self) -> Option<&T>;

    fn size(&self) -> usize {
        // cannot return self.size() it would be recursive call;
        N
    }
}

impl<T, const N: usize> ArrayOps<T, N> for [T; N] {
    fn first(&self) -> Option<&T> {
        if N > 0 {
            Some(&self[0])
        } else {
            None
        }
    }
    fn last(&self) -> Option<&T> {
        if N > 0 {
            Some(&self[N - 1])
        } else {
            None
        }
    }
}
struct Data {
    age: usize,
}

fn main() {
    let person = Data { age: 25 };

    let list = [person];
    println!("array size is {}", list.size());

    if let Some(item) = list.first() {
        println!("the person age is {}", item.age);
    }
}
