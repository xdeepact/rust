struct Person {
    name: String,
    age: u32,
    hair_color: String,
}

fn main() {
    // 1. Using field access, modify the existing values to replace the information with your own

    let mut me = Person {
        name: String::from("Jake Overall"),
        age: 29,
        hair_color: String::from("brown"),
    };

    // 2. Iterate over the struct to println! the field names
}
