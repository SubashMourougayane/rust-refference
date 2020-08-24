use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();
    // println!("Command => {:?}", command)
    let name = "Subash";
    let status = "100%";
    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status"{
        println!("Status is {}", status);
    } else{
        println!("That's not a valid command");
    }


}