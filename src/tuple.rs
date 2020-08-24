pub fn run() {
    let person: (&str, &str, i8) = ("Subash", "Pondy", 25);

    println!("{} is from {} and {} is {}", person.0, person.1, person.0, person.2);
}
