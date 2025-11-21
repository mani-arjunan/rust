# Rust

Unfortunately am into this rust thing now.. there is no going back, lets learn this mf.

## Intro | notes to me

 Rust is a compiled programming language like `go` | `C` | `C++`, to add more in rust memory management is manual unlike
`go` | `java` | `js` etcc, where you will get automatic GC on ur runtime.
 In Rust there is a in house tool called `cargo`, which is the dependency manager, similar to `npm` in js, `cargo new project`
creates a new project with directories `src/main.rs`, `cargo.toml` files, cargo.toml files contains the project
information like project_name, dependencies needed etc.
 * cargo build - builds your program and puts the executable under `target/debuge/*`.
 * cargo check - builds your program and checks if there are any issues, but it won't produce any binary/executable
 so its fast compared to build.
 * cargo run - builds and produce an executable and also runs the executable for us, similar to `go run main.go`

```rust
fn hello_world() {
    println!("Hello World!");
}

fn main() {
    hello_world();
}
```
 Above is the basic `Hello world` program in rust, as you can see in rust function is declared using `fn` keyword,
one interesting thing i found is instead of normal `println` here we are calling `println!`, the `!`
means we are calling a `macro`(kinda c macro) not a normal function.

### Crate
 Crates in rust are kinda of dependencies/npm packages in binary or library, that we can use in our application, these can
be downloaded using cargo(package manager).







