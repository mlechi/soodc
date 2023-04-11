use sha2::{Sha256, Digest};
use std::{env, fs::File, io::Read};
fn main() {
    let args:Vec<String> = env::args().collect();
    let mut checksum :String = String::new();
    let mut file_path: String = String::new();
    let mut verbose: bool = false;
    //Parsing arguments
    if args.len() > 1 {
        for i in 0..args.len() {
            match args[i].as_str() {
                //The supplied checksum
                "-c" => checksum = args[i+1].clone(),
                //The file to be hashed
                "-f" => file_path = args[i+1].clone(),
                "-v" => verbose = true,
                _ => (),
            }
        }
    }
    //Doing the thing that this thing does.
    let hash: String = retrieve_checksum(&checksum).unwrap();
    let result;
    //let result = git_sum_sha256_from_file(&file_path).unwrap();
    if verbose {
        println!("Sha256 hash supplied:");
        println!("{}", hash);
        println!("Sha256 of the file you are checking:");
        result = git_sum_sha256_from_file(&file_path).unwrap();
        println!("{}", result);
    } else {
        result = git_sum_sha256_from_file(&file_path).unwrap();
    }
    if hash == result {
        println!("They match");
    } else {
        println!("They do not match");
    }
    //Below, testing part
    //test();
}

fn git_sum_sha256_from_file( input:&String ) -> Result<String, String> {
    let mut file = match File::open(&input) {
        Ok(x) => x,
        Err(_) => panic!("Couldn't open file {input}"),
    };
    let mut buffer = Vec::new();
    let _x = file.read_to_end(&mut buffer);
    let mut summer = Sha256::new();
    //update method takes forever. Finalize and read_to_end are effectively instant.
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
        Ok(_) => file_string.split(' ').next().unwrap().to_string(),
        //Ok(x) => checksum.split_whitespace().collect()[0],
        Err(_) => panic!("Couldn't read checksum file to string."),
    };
    Ok(output)
}
