pub fn run()  {
    let name = "Subash";

    let mut age = 23;
    println!("My name is {} and i am {}", name, age);

    age = 24;
    println!("My name is {} and i am {}", name, age);

    // define constant
    const ID :i32 = 001;
    println!("ID: {}", ID );

    // assign multible var at once
    let (my_name, my_age) = ("Subash", "25");
    println!("{} is {}", my_name, my_age );

}