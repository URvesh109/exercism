#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    fn new(name: String, age: u32, weight: f32) -> Self {
        Self { name, age, weight }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn weight(&self) -> f32 {
        self.weight
    }

    fn set_age(&mut self, age: u32) {
        self.age = age;
    }
}

fn main() {
    let default: User = Default::default();
    println!("Default {default:?}");
    let mut user = User::new("Dhoni".to_owned(), 25, 86.0);
    println!("User weight {}", user.weight());
    println!("User name {}", user.name());
    user.set_age(45);
    println!("User data {user:?}");
}
