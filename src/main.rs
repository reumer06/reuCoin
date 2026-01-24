struct  Holder <'a> {
    reference : &'a str
}
static GREETING: &'static str  = "hello world";
fn make_holder() -> Holder<'static>  {
    // let text = String::from("the code");
    Holder {reference : &GREETING}
}
fn main(){
    println!("{}",make_holder().reference);
}