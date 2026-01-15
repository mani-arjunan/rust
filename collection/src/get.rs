pub fn get_main() {
    let vec_arr: Vec<i16> = vec![1,2,3,4];

    // println!("{}", vec_arr[20]); // this will result array out of index panicing(u know the
    // reason), this is usual in every languages, but in order to access an invalid index 
    // value without crashing the program rust provides a method called "get" where it will
    // return Option<T>, so u would end up in None if that index is not accessible
    match vec_arr.get(20) {
        Some(twenty) => println!("{}", twenty),
        _ => println!("Index is out of range")
    }
    match vec_arr.get(1) {
        Some(one) => println!("{}", one),
        _ => println!("Index is out of range")
    }

    // NOTE: by default [Vec::].get() will returned referenced value(&)
}
