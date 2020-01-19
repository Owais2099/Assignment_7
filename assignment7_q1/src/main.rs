mod shop {
    pub mod shopkeeper {
        pub fn greet_customer() {
            println!("Welcome to my shop!");
        }
    }
}

fn main() {
    crate::shop::shopkeeper::greet_customer();
}