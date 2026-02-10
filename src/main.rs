struct Duck {}
struct FormalDuck {
    name: String,
}

impl FormalDuck {
    fn new(name: String) -> Self {
        Self { name }
    }
}

trait Quack {
    fn quack(&self);
}

impl Quack for Duck {
    fn quack(&self) {
        println!("Duck Quacks");
    }
}

impl Quack for FormalDuck {
    fn quack(&self) {
        println!("{} Quacks", self.name);
    }
}

fn duck_say(quacker: &dyn Quack) {
    // &dyn is used to create trait object;
    // call the function instead of using <T: Quack> i.e static dispatch;
    quacker.quack()
}
fn main() {
    let duck = Duck {};
    let formal_duck = FormalDuck::new("Dodo".to_string());
    duck_say(&formal_duck);
    duck_say(&duck);
}
