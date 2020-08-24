use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(123);
    
    numbers.pop();

    
    println!("{:?}",numbers);

    // single val
    println!("Single val{:?}",numbers[0]);

    // len
    println!("Vector length {}", numbers.len());

    println!(" vector bytes: {}", mem::size_of_val(&numbers));

    // get slice

    let slice: &[i32] = &numbers[0..2];

    println!("slice : {:?}", slice);

    // loop
    for number in numbers.iter() {
        println!("{}", number);
    }

    // loop and iterate
    for x in numbers.iter_mut()  {
        *x *= 2;
    }

    println!("{:?}", numbers);


}