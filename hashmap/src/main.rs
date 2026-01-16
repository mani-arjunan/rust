use std::collections::HashMap;

fn main() {
    let mut persons_age: HashMap<String, i16> = HashMap::new();

    persons_age.insert(String::from("Manikandan"), 29);
    persons_age.insert("test".to_string(), 20);

    let test_2 = String::from("test_2");
    persons_age.insert(test_2, 10); //move test_2 ownership, after this line test_2 is not
    //available

    println!("{:?}", persons_age);

    let test_to_get = String::from("test");
    if let Some(age) = persons_age.get(&test_to_get) { // this is how u can get value by key
        // by default get function accepts reference to any variable, even 
        // for primitive keys u need to pass as reference &, eg below function rank_person_hashmap
        println!("{}", age);
    }

    rank_person_hashmap();
    iterate_over_hashmap(persons_age);
}

fn rank_person_hashmap() {
    let mut rank_person: HashMap<i16, String> = HashMap::new();

    rank_person.insert(1, String::from("Manikandan"));


    if let Some(name) = rank_person.get(&1) { // even though it may seem &1 redundant, coz address
        // of value 1 is not even valid right, coz 1 is value not a variable to get the memory
        // address, but it is what it is that rust is designed to avoid copy/cloning for
        // non-primitive things, even for this rust internally does
        // let temp: i16 = 1;
        // rank_person.get(&temp);
        // free(temp)
        // something like this
        println!("{}", name);
    }
}

fn iterate_over_hashmap(map: HashMap<String, i16>) {
    for (key, value) in map {
        println!("{}: {}", key, value);
    }

    println!("{:?}", map);
}
