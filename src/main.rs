#![allow(unused)]
use sha2::{Sha256, Sha512, Digest};
use std::{env, fs::File, io::Read};
fn main() {
    let args:Vec<String> = env::args().collect();
    let mut checksum :String = String::new();
    let mut file_path: String = String::new();
    //Parsing arguments
    if args.len() > 1 {
        for i in 0..args.len() {
            match args[i].as_str() {
                //The supplied checksum
                "-c" => checksum = args[i+1].clone(),
                //The file to be hashed
                "-f" => file_path = args[i+1].clone(),
                _ => (),
            }
        }
    }
    //Doing the thing that this thing does.
    println!("Sha256 hash supplied:");
    let hash: String = retrieve_checksum(&checksum).unwrap();
    //println!("{}", retrieve_checksum(&checksum).unwrap());
    println!("{}", hash);
    println!("Sha256 of input file:");
    //let result = git_sum_sha256_from_file(&file_path).unwrap();
    println!("{}", git_sum_sha256_from_file(&file_path).unwrap());
    //if
    //Below, testing part
    //test();
}

fn git_sum_sha256_from_file( input:&String ) -> Result<String, String> {
    let mut file = match File::open(&input) {
        Ok(x) => x,
        Err(_) => panic!("Couldn't open file {input}"),
    };
    //let mut file_string = String::new();
    //file.read_to_string(&mut file_string);
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer);
    let mut summer = Sha256::new();
    summer.update(&buffer);
    Ok(format!("{:x}", summer.finalize()))
}

fn retrieve_checksum(input:&String) -> Result<String, String> {
    let mut file_string = String::new();
    let mut file = match File::open(&input) {
        Ok(x) => x,
        Err(_) => panic!("Couldn't open file {input}"),
    };
    let output = match file.read_to_string(&mut file_string) {
        Ok(x) => file_string.split(' ').next().unwrap().to_string(),
        //Ok(x) => checksum.split_whitespace().collect()[0],
        Err(_) => panic!("Couldn't read checksum file to string."),
    };
    Ok(output)
}

//For testing purposes.
fn sum_sha256_str(input:&String) -> Result<String, String> {
    let mut summer = Sha256::new();
    summer.update(&input);
    let result = summer.finalize();
    Ok(format!("{:x}", result))
}

//For testing purposes.
fn test() {
    let string_1 = String::from("Hello, World!\n");
    let result = sum_sha256_str(&string_1);
    println!("{}", result.unwrap());
    let path = String::from("/home/meat/Projects/Rust/summer_of_our_disc_content/src/file");
    let result = git_sum_sha256_from_file(&path);
    println!("{}", result.unwrap());
}
