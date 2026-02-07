// static dispatch

// https://rustc-dev-guide.rust-lang.org/backend/monomorph.html             -> fast runtime at cost of compile time (creating copies of each will take time)

fn peach<T>() {
    // monomorphization collector;
    println!("monomorphization for type: {}", std::any::type_name::<T>());
}

fn banana() {
    peach::<u64>();
    peach::<u32>();
}

struct Duck {
    // fields to store data of duck;
    name: String,
}
trait Quack {
    // shares behaviour that types must implement;
    fn quack(&self);
}
impl Duck {
    // constructor of Duck struct;
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl Quack for Duck {
    // implementation of quack trait for duck;
    fn quack(&self) {
        println!("{} wants to say Quackkkkkkkkkkkkkkkkkkkkk", self.name)
    }
}

// Monomorphization
fn duck_say<T: Quack>(quacker: T) {
    // duck_say uses a "Trait Bound" <T: Quack>
    // This tells the compiler to accept any type T that implements the Quack trait.
    quacker.quack();
}

fn main() {
    let duck = Duck::new("Quill".to_string()); // object for trait;
    duck_say(duck);

    // duck.quack();           // this won't compile duck has been moved above;

    banana();
}
