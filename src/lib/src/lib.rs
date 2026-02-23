use uint::construct_uint;

construct_uint! {
    // construct an unsigned 256-bit integer
    pub struct U256(4);
}
pub mod crypto;
pub mod sha256;
pub mod types;
pub mod util;
