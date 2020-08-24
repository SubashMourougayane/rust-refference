pub fn run() {
    // print to console
    println!("Hello World!");

    // Basic Formating
    println!("Number : {}", 1);
    println!("{} is from {}", "Subash", "Pondicherry");

    //Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Subash", "Pondicherry", "code"
    );


    // named arguments
    println!("{name} likes to play {activity}", name = "Subash", activity = "baseball" );


    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10 );

    // palceholder for debug trait
    println!("{:?}", (12, true, "hello") );

    // math

    println!("10 + 10 = {}", 10+10);
}
