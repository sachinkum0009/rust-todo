const DB_PATH: &str = "todo.db";

pub struct Config {
    pub db_path: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            db_path: String::from(DB_PATH),
        }
    }
}