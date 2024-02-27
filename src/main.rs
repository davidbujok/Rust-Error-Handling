use std::{fs::File, io::{ErrorKind, Read, self}};

fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                    "Tried to create file but there was a problem: {:?}",
                        e)
                },
            }
        },
        Err(error) => {
            panic!("There was a problem opening a file: {:?}",error)
        },
    };
    let _new_file = read_username_from_file();
    let username = read_username_from_file_with_question();
    println!("Username from the file: {}", username.ok().unwrap())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
// THE SAME FUNCTION WITH ? OPERATOR, SHORTUCT FOR HANDLING AND RETURNING ERRORS BACK
// the ? opeartor works only in functions that return Resutl type

fn read_username_from_file_with_question() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    // YOU COULD EVEN CHAIN THOSE TO BE SHORTER
    // let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
}
