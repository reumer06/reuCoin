mod my_module {
    pub mod child_module {
        pub mod grand_child_module {
            pub(super) fn public_in_grandchild() {  // visibility to parent; my_module
                println!("public in granchild");
            }

            pub fn public_for_everyone(){   // for everyone;
                println!("public for everyone");
                public_in_self();
                public_in_grandchild();
                public_in_module();
            }
            pub(self) fn public_in_self() { // to current module only; grand_child_module
                println!("public in self");
            }
            pub(in crate::my_module) fn public_in_module() {    //inside my_module everyone can access it.
                println!("public from this module");
            }
        }
    }
    pub mod other_child {
        pub(super) fn public_in_my_module() { // visibility to parent; my_module
            println!("accessible from my module");
            // super::child_module::grand_child_module::public_in_module(); --> can access it;
        }
    }
}

fn main() {
    use my_module::child_module::grand_child_module;
    // use my_module::other_child;
    // other_child::public_in_my_module(); --> this is private under other_child;
    grand_child_module::public_for_everyone();
}
