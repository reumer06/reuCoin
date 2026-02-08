use std::fs::File;
use std::io::Write;     // --> required for writeln!
use std::path::Path;    // --> Require for AsRef<Path>
use std::fmt::{Display, Formatter, Result as FmtResult};


trait Saveable: Display {       // -> Trait with Supertrait; i.e Saveable only be implemented if you implement Display
    fn save<P>(&self, path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,     // --> P: AsRef<Path> allow to pass &str, String or Path objects
    {
        // ? is there for error handling it will stop the entire function when something fails;
        let mut file = File::create(path.as_ref())?;     // --> path.as_ref() converts generic P into a standard &Path;
        writeln!(file, "{}", self.to_string())?;                   // --> converts to String because saveable requires Display;

        Ok(())
    }
}

struct User {
    name: String,
    id: u32,
}
// Without this the compiler will refuse to implement Saveable;
impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "User: {} (ID: {})", self.name, self.id)
    }
}

impl Saveable for User {}       // implement our Trait;
fn main() -> std::io::Result<()> {      // -->  for io errors
    let first_user = User {
        name: String::from("littlejames"),
        id: 42,
    };

    first_user.save("user_data.txt")?;

    println!("User info saved successful");
    Ok(())          // returns okay if everything went well;
}
