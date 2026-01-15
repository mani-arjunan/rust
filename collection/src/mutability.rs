pub fn mutability() {
    let mut vec_arr: Vec<String> = vec![String::from("XXXX")];
    let first = &vec_arr[0];

    vec_arr.push(String::from("YYYY")); // this won't work coz we already created one immutable
    // reference vec_arr[0] and assigned to first, now when we do vec_arr.push
    // the push underlyingly creates mutable reference to this vec, coz it may needed to increase
    // the array length if its already full or delete coz of this it may need to shift all the
    // elements the indexes can change, the immutable reference above can become invalid.
    // to make this work either create copy .clone() and assign to first, or move this print
    // to above the line, so that the immutable scope destroyed.

    println!("{}", first);
}
