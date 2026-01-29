struct MyStruct<'a> {
    remainder: Option<&'a str>, //  stores a pointer to  a string: we'll need mutable reference to shrink it.
}

impl<'a> MyStruct<'a> {
    fn first_char(&mut self) -> Option<&str> {
        let remainder = &mut self.remainder?; // gets a mutable reference from Option, avoiding move.
        let c = &remainder[0..1]; // to read an empty string the program will panic might use remainder.is_empty()
        if remainder.len() != 1 {
            // if one than more char shrinks the slice
            *remainder = &remainder[1..];
            Some(c)
        } else {
            self.remainder.take()
        }
    }
}

fn main() {
    // test run
    let mut broken = MyStruct {
        remainder: Some("Hello"),
    };

    for _ in 0..5 {
        println!("{:?}", broken.first_char());
    }
}
