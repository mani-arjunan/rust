mod get;
mod mutability;

use get::get_main;
use mutability::mutability;

fn main() {
    let normal_arr: [i16; 5] = [1, 2, 3, 4, 5]; // normal fixed size array, on stack memory,
    // meaning the size is determined on compile time itself, so this main stack can allocate
    // memory according to that, u cannot shrink or grow this size.

    for i in normal_arr {
        println!("{i} \t");
    }

    let mut vec_arr: Vec<i16> = Vec::new();
    // let vec_arr = vec![1, 2, 3]; this is another way of declaring vector using inbuilt macro
    // vec!

    vec_arr.push(1);
    vec_arr.push(2);
    vec_arr.push(3);
    vec_arr.push(4);


    let first = vec_arr[0];
    println!("First element of vec_arr => {}", first);

    let mut vec_arr_2: Vec<String> = Vec::new();

    vec_arr_2.push(String::from("XXXX"));
    vec_arr_2.push(String::from("YYYY"));
    vec_arr_2.push(String::from("ZZZZ"));

    // let first = vec_arr_2[0]; // this will throw error, coz considering the rust borrow and
    // ownership concept, when we assign vec_arr_2[0] to first, we are basically
    // transferring/moving the ownership which basically means moving out of the vec_arr_2,
    // in the above vec_arr[0] you won't get the same error, it is bcoz above vec_arr is using i16
    // which is a primitive type,
    // to fix this vec_arr_2[0], you either need to read as reference &vec_arr_2[0] or do
    // vec_arr_2[0].clone() which clones a new one and assigns to first
    // NOTE: println!("{}", vec_arr_2[0]) works without & or clone, coz by default
    // the inbuild println! methods does add reference operator & when needed
 
    println!("First element of vec_arr => {}", vec_arr_2[0]);

    get_main();
    mutability();
}
