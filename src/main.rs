use sha2::{Sha224, Sha256, Sha384, Sha512, Digest};
use std::{env, fs::File, io::Read};
fn main() {
    let args:Vec<String> = env::args().collect();
    let mut hash :String = String::new();
    let mut file_path: String = String::new();
    let mut verbose: bool = false;
    let mut summer: Sha = Sha::Sha256;
    //Parsing arguments
    if args.len() > 1 {
        for i in 0..args.len() {
            match args[i].as_str() {
                //The supplied hash
                "-h" => hash = args[i+1].clone(),
                //The file to be hashed
                "-f" => file_path = args[i+1].clone(),
                "-v" => verbose = true,
                "-sha224" => summer = Sha::Sha224,
                "-sha384" => summer = Sha::Sha384,
                "-sha512" => summer = Sha::Sha512,
                _ => (),
            }
        }
    } else { help_message(); return }
    //Doing the thing that this thing does.
    let hash: String = retrieve_hash(&hash).unwrap();
    let result;
    if verbose {
        println!("Hash supplied:");
        println!("{}", hash);
        println!("Hash of the file you are checking:");
        result = git_sum_from_file(&file_path, summer).unwrap();
        println!("{}", result);
    } else {
        result = git_sum_from_file(&file_path, summer).unwrap();
    }
    println!("");
    if hash == result {
        println!("Hash target: {hash}");
        println!("Hash result: {result}");
        println!("");
        println!("They match");
    } else {
        println!("Hash target: {hash}");
        println!("Hash result: {result}");
        println!("");
        println!("They do not match");
    }
}

pub enum Sha {
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

fn git_sum_from_file( input:&String, summer: Sha) -> Result<String, String> {
    let mut file = match File::open(&input) {
        Ok(x) => x,
        Err(_) => panic!("Couldn't open file {input}"),
    };
    let mut buffer = Vec::new();
    let _x = file.read_to_end(&mut buffer);
    //let mut summer = Sha256::new();
    if let Sha::Sha224 = summer {
        let mut sum = Sha224::new();
        sum.update(&buffer);
        Ok(format!("{:x}", sum.finalize()))
    } else if let Sha::Sha256 = summer {
        let mut sum = Sha256::new();
        sum.update(&buffer);
        Ok(format!("{:x}", sum.finalize()))
    } else if let Sha::Sha384 = summer {
        let mut sum = Sha384::new();
        sum.update(&buffer);
        Ok(format!("{:x}", sum.finalize()))
    } else if let Sha::Sha512 = summer {
        let mut sum = Sha512::new();
        sum.update(&buffer);
        Ok(format!("{:x}", sum.finalize()))
    } else {
        return Ok("".to_string())
    }
}

fn retrieve_hash(input:&String) -> Result<String, String> {
    let mut file_string = String::new();
    let mut file = match File::open(&input) {
        Ok(x) => x,
        Err(_) => panic!("Couldn't open file {input}"),
    };
    let output = match file.read_to_string(&mut file_string) {
        Ok(_) => file_string.split(' ').next().unwrap().to_string(),
        Err(_) => panic!("Couldn't read hash file to string."),
    };
    Ok(output)
}

fn help_message() {
    println!("");
    println!("This is the checkSummer Of Our Disk Content");
    println!("");
    println!("Requires a file to check, and a file containing the valid hash to compare the file to.");
    println!("");
    println!("-h: file containing the provided hash.");
    println!("-f: file to be checked.");
    println!("");
    println!("-sha224, -sha384, -sha512 to use these hashing algorithms as opposed to the default of sha256.");
    println!("");
    println!("");
}
