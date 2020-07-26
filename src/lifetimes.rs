pub fn lifetimes_simple() {
    let list = vec![100, 34, 72, 55];
    let first_two = return_first_two(&list);

    println!("first two are {:?}", first_two);
    println!("list is {:?}", list);

    println!("again first two are {:?}", first_two);
}

fn return_first_two(list: &[i32]) -> &[i32] {
    &list[..2]
}

// pub fn bad_return_reference_from_function() {
//     let first_two = return_first_two_ex();
//     // borrowed value does not live long enough!
//     println!("first two are {:?}", first_two);
// }
//
// fn return_first_two_ex() -> &[i32] {
//     let list = vec![100, 34, 72, 55];
//     &list[0..2]
// }

pub fn return_list_example() {
    let list = return_list();
    let first_two = &list[0..2];
    println!("first two are {:?}", first_two);
}

fn return_list() -> Vec<i32> {
    vec![100, 34, 72, 55]
}

// struct ListAndRef {
//     list: Vec<i32>,
//     first_two: &[i32],
// }
//
// pub fn return_list_and_first_two() -> ListAndRef {
//     let list_to_use = vec![100, 34, 72, 55];
//
//     ListAndRef {
//         list: list_to_use,
//         first_two: &list_to_use[0..2],
//     }
// }
