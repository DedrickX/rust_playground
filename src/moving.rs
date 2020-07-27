pub fn test() {
    let a = 5;
    let b = a;

    println!("Value a = {}, value b = {}", a, b);

    let arr1 = vec![1, 2, 3];
    let arr2 = arr1; // this is move!!!

    // can not use value arr1, it was moved to arr2.
    // println!("Value arr1 = {:?}, value arr2 = {:?}", arr1, arr2);

    println!("Value arr2 = {:?}", arr2);
}
