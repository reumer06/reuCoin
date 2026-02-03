#![allow(unused)]

use crate::outer::public_in_super_func;

pub fn public_func() {
    // accessable from everywhere
    println!("Public func");
}

fn private_func() {
    // private func i.e for current file only
    println!("Private func");
}

pub(crate) fn public_in_crate_func() {
    // public to whole project;
    println!("public func from crate");
}

pub(self) fn public_in_self_func() {
    // explicit private func;
    println!("public func from self");
}
mod outer {
    pub(super) fn public_in_super_func() {
        /* visible to 'outer' and its parent
        i.e in this case its main.rs file as mod outer is defined inside main.rs */
        println!("public func from super");
    }
}

fn main() {
    public_func();
    private_func();
    public_in_crate_func();
    public_in_super_func();
}
