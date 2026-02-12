// generics in rust are mostly traits.
use serde::Serialize; // to convert into desired format.
use std::marker::PhantomData; // zero-sized data, disappears at compilation;
struct Json;
struct Toml;
struct Cbor;
struct Yaml;
trait Encode {
    fn encode<T: Serialize>(val: T) -> String;
}

impl Encode for Json {
    fn encode<T: Serialize>(val: T) -> String {
        serde_json::to_string(&val).unwrap()
    }
}

impl Encode for Toml {
    fn encode<T: Serialize>(val: T) -> String {
        toml::to_string(&val).unwrap()
    }
}

impl Encode for Cbor {
    fn encode<T: Serialize>(val: T) -> String {
        let bytes = serde_cbor::to_vec(&val).unwrap();
        hex::encode(bytes)
    }
}

impl Encode for Yaml {
    fn encode<T: Serialize>(val: T) -> String {
        serde_yaml::to_string(&val).unwrap()
    }
}

struct User<T: Encode> {
    name: String,
    age: u32,
    _marker: PhantomData<T>, // to let know compiler T is associated with User. Otherwise we will get a compilation error;
}
fn main() {
    let user: User<Json> = User {
        name: "Alice".to_string(),
        age: 19,
        _marker: PhantomData,
    };

    println!(
        "user name is {} \nthe age of the user is {}",
        user.name, user.age
    );
}
