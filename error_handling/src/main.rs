use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind};

// This function works but is not written as efficently as possible. I rewreite it bellow to be
// more optomized
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username) // I need to remember that i should NOT add a semicolin to the line if the

    fs::read_to_string("hello.txt") // read_to_string does all of this for you, and returns a
                                    // Result<T, E>
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    // let test_str = String::from("New string");
    let test_str = "New string";
    let first_char = last_char_of_first_line(&test_str);
    println!("The first character is: {:?}", first_char);

    let greeting_file_result = File::open("hello.txt"); // The return type of File::open() is a
                                                        // Result<T, E>
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error)
            }
        },
    };

    let _demo_file_1 = File::open("demo_1.txt").unwrap(); // unwrap() will create an Error and
                                                          // panic!() if open() fails

    // expect() works much like unwrap() but you can choose the panic message.
    let _demo_file_2 = File::open("demo_2.txt").expect("demo_2.txt should be included");

    let username_file = read_username_from_file();
    println!("{:?}", username_file);
}
