use std::io::prelude::*;
use std::io::BufReader;
use std::fs::{self, File};
use std::error::Error;

fn create_file(s: &str) -> Result<File, Box<Error>> {
    let mut f = File::create(s);
    Ok(f.unwrap())
}

fn write_data(file: &str, data: &str) {
    fs::write(file, data).expect("Unable to write data");
}

fn main() {
    for x in 0..10 {
        let mut base: String = "./db/".to_owned();
        let mut data: String = "hello world".to_owned();
        let mut x_str: &str = &x.to_string();
        base.push_str(x_str);
        create_file(&base);
        write_data(&base, &data)
    }
}
