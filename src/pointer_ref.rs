pub fn run() {
    let arr1: [i32; 3] = [1, 2, 3];
    let arr2: [i32; 3] = arr1;
    println!("Values {:?}", (arr1, arr2));

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: &Vec<i32> = &v1;

    println!("{:?}", (&v1, v2));

}
