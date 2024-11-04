pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    // Creates a new User with the given name, age, and weight
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        User { name, age, weight }
    }

    // Returns the name of the User
    pub fn name(&self) -> &str {
        &self.name
    }

    // Returns the age of the User
    pub fn age(&self) -> u32 {
        self.age
    }

    // Returns the weight of the User
    pub fn weight(&self) -> f32 {
        self.weight
    }

    // Sets a new age for the User
    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    // Sets a new weight for the User
    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}
