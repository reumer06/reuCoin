// strongly typed language, all values need to be well-defined.rust also not like implicit conversion.
// rustc --explain E0384
// read only is default in rust.
fn myfunc(mut x :  i32){
    x *= 2;
    println!("{x}");
}
fn main(){
    // let mut x = 10;
    // x = 5;
    // println!("value of x is {x}");
    //
    // myfunc(12);

    // you can do this:
    let x = 10;
    println!("value is {x}");

    let mut x = x;
    x +=1;
    println!("value is {x}");
    let mut m = String::from("bob");
    let n = String::from("alice");

    let mut m_ref = &m;
    println!("value of string is {m_ref}");
    m_ref = &n;
    println!("the value of string is {m_ref}");
    let mut string_mut = &mut m;
    string_mut.push_str(" says hello");
    println!("{m} \n{n}");

}

