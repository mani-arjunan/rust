
// here T implementes PartialOrd and Copy trait, means
// types that have copy trait can be passed(primitive types)
fn compare_two_structs<T: PartialOrd + Copy>(a: T, b: T) -> bool {
    if a < b {
        return true;
    }

    return false;
}

fn main() {
    let int1 = 12;
    let int2 = 24;

    let char1 = 'c';
    let char2 = 'd';


    compare_two_structs(int1, int2);
    compare_two_structs(char1, char2);

    // let str1 = String::from("Mani");
    // let str2 = String::from("Aru");
    // below will be error, coz Copy trait is
    // not been implemented by String, which is expected
    // coz String is not primitive type
    // compare_two_structs(str1, str2);
}


