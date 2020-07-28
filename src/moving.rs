pub fn test() {
    let a = 5;
    let b = a;

    println!("Value a = {}, value b = {}", a, b);

    let s1 = "hello";
    let s2 = s1;

    println!("Value s1 = {}, value s2 = {}", s1, s2);

    let s3 = String::from("ahoy");
    let s4 = s3; // this is move!!!

    println!("Value s4 = {:?}", s4);
}
