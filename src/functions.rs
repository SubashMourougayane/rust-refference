pub fn run() {
    greeting("Hello", "Jane");

    // bind func val to var
    let sum = add(5, 5);

    println!("Sum: {}", sum);

    //  clousure
    let num3: i32 = 10;
    let add_nums = |num1: i32, num2: i32| num1 + num2 + num3;

    println!("C Sum: {}", add_nums(2, 2))
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
