pub fn run() {
    // immutable string
    // let hello = "Hello!";

    // Growable mutable string
    let mut string = String::from("Hellowww! ");

    // push char
    string.push('W');

    // push char
    string.push_str("orld!");

    // get length
    println!("Length of {} is {}", string, string.len());

    println!("Is Empty: {}", string.is_empty());

    println!("Contains 'World' {}", string.contains("World"));

    println!("Replace: {}", string.replace("World", "There"));

    // loop with white space

    for word in string.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    println!("{}", s);


    // assertion testing

    assert_eq!(2, s.len());

    assert_eq!(10, s.capacity());


    
    // capacity
    println!("Capacity: {}", string.capacity());
}
