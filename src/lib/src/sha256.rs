use crate::U256;
use serde::Serialize;
use sha256::digest;
#[derive(Clone, Copy, Serialize)]

pub struct Hash(U256);
