use std::mem;

pub fn run(){
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // Re-assign value
    numbers[2] = 20;

    
    println!("{:?}",numbers);

    // single val
    println!("Single val{:?}",numbers[0]);

    // len
    println!("{}", numbers.len());

    println!(" array bytes: {}", mem::size_of_val(&numbers));

    // get slice

    let slice: &[i32] = &numbers[0..2];

    println!("slice : {:?}", slice);

}