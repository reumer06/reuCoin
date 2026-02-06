trait Quack {
    fn quack(&self);
}
struct Duck;

impl Quack for Duck {
    fn quack(&self) {
        println!("quack");
    }
}
struct Human;

impl Human {
    fn quack(&self){
        println!("Humans do not quack");
    }
}

fn main() {
    Duck.quack();
    Human.quack();
}