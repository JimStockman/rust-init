mod person;
use person::Person;

fn main() {
    // Declare a new person
    let johnny = Person::new("Jim", "Stockman", 27, "1997-10-16");
    johnny.greet();

    let marie = Person {first_name: "Marie".to_string(), last_name: "Currie".to_string(), age: 50, birthdate: "who knows".to_string()};
    marie.greet();
}
