use std::fs::File;
use std::io::ErrorKind;
use std::io::Error;
use std::io::Read;
use std::net::IpAddr;

fn read_from_file() -> Result<String, Error> {
    let mut file = File::open("test.txt")?; // ? basically returns error if it encounters error
    // when reading a file, if success it will assign the file to file variable, similar to matchotherwise it will 
    let mut str = String::new();
    
    file.read_to_string(&mut str)?; // same as above if it got any error, it will return coz of ?

    return Ok(str);

}

fn test() {
    let ip_addr = "127.0.0.1".parse::<IpAddr>();

    let ip_addr = match ip_addr {
        Ok(addr) => addr,
        Err(error) => panic!("ERRRR => {}", error),
    };

    println!("{}", ip_addr);
}

fn main() {
    // panic!("App got crashed"); // manually generating unrecoverable error
    let file = File::open("test.txt");

    let file = match file {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(file) => file,
                Err(error) => panic!("{}", error),
            },
            other_error => panic!("{}", other_error),
        },
    };

    println!("{:?}", file); // above is example for recoverable error
    read_from_file();
    test();
}
