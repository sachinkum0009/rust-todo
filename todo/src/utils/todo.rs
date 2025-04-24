#[derive(Debug)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub priority: i32,
}

impl Todo {
    pub fn new(name: String, description: String, priority: i32) -> Todo {
        Todo {
            id: 0,
            name,
            description,
            priority,
        }
    }
    pub fn print_details(&self) {
        println!("Name: {}", self.name);
        println!("Description: {}", self.description);
        println!("Priority: {}", self.priority);
    }
}
