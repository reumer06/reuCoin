

fn iter1(iterator: impl Iterator<Item = i32>) { // cannot implement turbofish here.
    // APIT (Argument position impl trait) // lazy way of writing generics.
    // same as the generic type but generic parameter is anonymous; generic type can only be inferred.
    for i in iterator {
        println!("value: {} ", i);
    }
}

fn iter2<I: Iterator<Item = i32>>(iterator: I) {
    // inline generic
    for i in iterator {
        println!("value: {} ", i);
    }
}

fn iter3<I>(iterator: I)    // where clause
where
    I: Iterator<Item = i32>,
{
    for i in iterator {
        println!("value: {} ", i);
    }
}

fn main() {
    // turbofish ::<>
    let numbers = (1..5).collect::<Vec<i32>>();
    let my_vec : Vec<i32> = vec![1,2,3,1];
    iter2::<std::vec::IntoIter<i32>>(my_vec.into_iter());       // cannot do same for iter1 no generic arguments found;
}
