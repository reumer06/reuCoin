
fn max_ref<'qa>(left: &'qa i32,right: &'qa i32) -> &'qa i32 { // lifetime annotations
    if *left < *right{
        right
    } else {
        left
    }
}
fn main(){

    let i = 32; let y = 43;
    println!("{}",max_ref(&i,&y));
}