struct Cat {
    name : String,
}

trait Animal {
    fn make_sound(&self);
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("name of the car is {} and he does meowwwww",self.name);
    }
}
fn main() {
    let cat = Cat{
        name: String::from("mylo"),
    };
    cat.make_sound();
}