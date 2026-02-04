pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn new(list: Vec<i32>) -> Self {
        // to create a user instance or public constructor.
        let mut fool = AveragedCollection { list, average: 0.0 };
        fool.update_average();
        fool
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }
    pub fn average(&self) -> f64 {
        self.average
    }
}

fn main() {
    let mut collection = AveragedCollection::new(vec![1, 2, 3, 4]); // creates a object
    println!("average: {}", collection.average());
    collection.add(12);
    println!("updated Average: {}", collection.average());

    match collection.remove() {
        // removes 12
        Some(_val) => println!("removed: {_val}, new average: {}", collection.average()),
        None => println!("empty list"),
    }
    // can do it both ways;
    if let Some(_val) = collection.remove() {
        // removes 4
        println!("average: {}", collection.average());
    }
}
