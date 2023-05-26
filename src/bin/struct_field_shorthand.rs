#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: "Bot".to_string(),
            age: 0,
        }
    }
}

fn create_default() {
    let tmp = Person {
        ..Default::default()
    };
    println!("name {} age {}", tmp.name, tmp.age);

    let tmp = Person {
        name: "Sam".to_string(),
        ..Default::default()
    };
    println!("{tmp:?}");
}

fn main() {
    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");

    create_default();

    let pan = Person {
        name: "Pan".to_string(),
        ..peter
    };
    println!("{pan:?}");

    println!("{peter:?}");
}
