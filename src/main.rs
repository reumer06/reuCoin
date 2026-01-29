struct MyStruct<'a> {
    remainder: Option<&'a str>,
}

impl<'a> MyStruct<'a> {
    fn first_char(&mut self) -> Option<&str> {
        let remainder = &mut self.remainder?;
        let c = &remainder[0..1];
        if remainder.len() != 1 {
            *remainder = &remainder[1..];
            Some(c)
        } else {
            self.remainder.take()
        }
    }
}

fn main() {
    let mut broken = MyStruct {
        remainder: Some("Hello"),
    };

    for _ in 0..5 {
        println!("{:?}", broken.first_char());
    }
}
