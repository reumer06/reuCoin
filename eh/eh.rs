fn main() {
    let a = true;
    assert!(
        a,
        "this will not cause a panic attack as the bool value is true"
    );

    let x = 5;
    let y = 5;
    assert_eq!(x, y, "this will not cause panic attack as x == y");

    let m = 1;
    let n = 2;
    assert_ne!(m, n, "this will not cause panic attack as m!=n");

    if false {
        unreachable!("this code is unreachable, this will never cause panic");
    }

    if false {
        unimplemented!("this is code is unimplemented,this will never cause panic");
    }
    if false {
        todo!("panic will never occur as todo is an alias of unimplemented");
    }

    println!("No panics occurred because all conditions were rolled");
}
