fn no_param<T>(_: &T) {} /* this won't compile ?Sized mentioned cause even generic parameters without any written trait have hidden trait bound which is
                          T Sized the type sized is known at compile time we have to use T: ?Sized for writing this unique trait.*/
fn main() {
    let my_str = "Banana";
    no_param(my_str);       // ERROR: doesn't have a size known at compile-time
}
