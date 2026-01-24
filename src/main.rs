

struct Hello;

trait SayHi {
    fn say_hi(self);
}

impl SayHi for Hello {
    fn say_hi(self) {
        println!("this is from without reference");
    }
}

impl SayHi for &Hello {
    fn say_hi(self) {
        println!("this is from the reference");
    }
}

impl SayHi for &&Hello {
    fn say_hi(self) {
        println!("this is from the reference reference");
    }
}

fn main() {
    let hello = Hello;
    // hello.say_hi();
    (&hello).say_hi();
    (&&hello).say_hi();
    hello.say_hi();
}