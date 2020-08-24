pub fn run() {
    let mut count = 0;

    // infinite loop

    loop{
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }

    }

    // while loop (Fizz Buzz)

    while count<=100 {
         if count % 15 == 0{
            println!("FizzzBuzz");
         }else if count % 3 == 0{
             println!("Fizz");
         }else if count % 5 == 0{
             println!("Buzz");
         }else {
             println!("{}", count)
         }

         count +=1;
    }

    for x in 1..100 {
        if x % 15 == 0 {
            println!("FizzzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}
