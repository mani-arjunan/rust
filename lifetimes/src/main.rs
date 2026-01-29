// use rand;
//
// // this won't work, coz rust doesn't know which would go away and which
// // memory would be retaining(a | b), so introducing a lifetime like compare_2
// // can explicitly tell rust, that both lifetime could be exist
// // fn compare(a: &str, b: &str) -> &str {
// //     if rand::random() {
// //         return a;
// //     }
// //     return b;
// // }
//
// // fn main() {
// //     let a = "Manikandan";
// //     let b = "Arjunan";
// //
// //     // compare(a, b); // this won't work
// //     compare_2(a, b);
// // }
//
// // declaring explicit lifetime like 'str(this could be anything 'a, 'b etc)
// // returned reference is valid as long as both a and b lives
// fn compare_2<'str>(a: &'str str, b: &'str str) -> &'str str {
//     if rand::random() {
//         return a;
//     }
//     return b;
// }
//
// // fn main() {
// //     let a = String::from("Manikandan");
// //     let result;
// //     {
// //         let b = String::from("Arjunan");
// //         result = compare_2(a.as_str(), b.as_str());
// //         // above will be error because b will goes out of scope after this
// //     }
// //
// //     println!("{}", result);
// // }
// //
// // fn main() {
// //     let a = "Manikandan";
// //     let result;
// //     {
// //         let b = "Arjunan";
// //         result = compare_2(a, b);
// //         // however this will works, coz a and b in this program are string literals
// //         // it will basically be stored as &'static str, means this will be stayed
// //         // in the memory as long as duration for the entire program execution.
// //     }
// //
// //     println!("{}", result);
// // }
// //
//
// fn compare_3<'str>(a: &'str str, b: &str) -> &'str str {
//     return a;
// }
//
// fn main() {
//     let a = String::from("Manikandan");
//     let result;
//     {
//         let b = String::from("Arjunan");
//         result = compare_3(a.as_str(), b.as_str());
//         // this will not be error, coz in compare_3
//         // we only defined the lifetime for a, which is alive untill
//         // the end of the line, atleast where print is executed
//     }
//
//     println!("{}", result);
// }

// lifetime 3 rules
// 1. Each param which has reference gets its own lifetime, a: &str, b: &str etcc all gets its own
//    lifetime
// 2. If there is exactly only one paramater, then its lifetime will be assigned to the output.
// 3. If there are multiple parameters and atleast one of them has `&self` | mut &self, then
//    the lifetime of self is assigned to all the output.
//
// TL;DR If something contains a reference, Rust must prove the data outlives every path that could return it,
// in these cases use an explicit lifetime, something -> function, struct etc.
// EG in struct without explicit lifetimes
struct Employee<'a> {
    name: &'a str,
}

fn main() {
    let emp;

    {
        let name2 = String::from("Manikandan");
        emp = Employee {
            name: name2.as_str(),
        };
    }

    // this will throw error, coz name2 has been removed after the above },
    // this is what lifetime meant, however if u declare let name2 on outer scope(i mean root)
    // and then assign name2 = String:from('...') inside the scope {} this will work,
    // coz name2 lifetime is now known to Emp
    println!("{}", emp.name);
}
