extern crate rand;
extern crate uuid;
extern crate stopwatch;

use std::fs::{self, File, OpenOptions};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::io::{BufReader, LineWriter};
use std::io::prelude::*;
use std::thread;

use uuid::Uuid;
use rand::prelude::*;
use stopwatch::{Stopwatch};

struct DataGateway {
    file: String,
    reader: Option<BufReader<File>>,
    writer: Option<LineWriter<File>>,
}


impl DataGateway {
    fn new(file_path: String) -> Option<DataGateway> {
        let fp = file_path.as_str();

        let write_f = OpenOptions::new()
                        .read(true)
                        .append(true)
                        .create(true)
                        .open(fp);

        let read_f = OpenOptions::new()
                        .read(true)
                        .append(true)
                        .create(true)
                        .open(fp);

        let br: Option<BufReader<File>>;
        let lw: Option<LineWriter<File>>;
        br = match read_f {
            Ok(file) => Some(BufReader::new(file)),
            Err(_) => None
        };
        lw = match write_f {
            Ok(file) => Some(LineWriter::new(file)),
            Err(_) => None
        };

        if br.is_some() && lw.is_some() {
            return Some(DataGateway{
                file: file_path,
                reader: br,
                writer: lw
            })
        }
        return None
    }
}

fn new_shard(mut hm: HashMap<&str, DataGateway<>>) {
    let shards: [&'static str; 16] = [
        "0", "1", "2", "3",
        "4", "5", "6", "7",
        "8", "9", "a", "b",
        "c", "d", "e", "f"
    ];
    let mut hmcopy = hm;
    for x in 0..16 {
        let mut base: String = "./db/".to_owned();
        let x_str: &str = &x.to_string();
        base.push_str(x_str);
        let dg_option = DataGateway::new(base);
        match dg_option {
            Some(dgate) => hmcopy.insert(shards[x], dgate),
            None => None
        };
    }
    hm = hmcopy;
}

fn create_file(s: &str) -> Result<File, &'static str> {
    let f = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .create(true)
                    .open(s);
    match f {
        Ok(z) => Ok(z),
        Err(e) => Err("error creating file")
    }
}

fn read_data(shard_map: &HashMap<&str, File>, key: &str) -> Result<&'static str, &'static str> {
    let target_shard = key.to_string().chars().next().unwrap();
    match shard_map.get::<str>(&target_shard.to_string()) {
        Some(target_file) => {
            let mut reader = BufReader::new(target_file);
            let mut line = String::new();
            reader.read_line(&mut line);
            Ok("found and reading from file") 
        }
        None => Err("error reading")
    }
}

fn read_time_series<'a>(
    shard: &HashMap<&'a str, DataGateway>, time_one: SystemTime, time_two: SystemTime
)  -> Vec<&'a str> {
    
}

fn write_data(shard_map: &HashMap<&str, File>, data: &str) -> Result<bool, &'static str> {
    // get write_hash
    let seed: u128 = random();
    let write_hash = Uuid::from_u128(seed);

    // get nano write time
    let write_time = SystemTime::now();
    let nwt = write_time.duration_since(UNIX_EPOCH).
                               expect("Time went backwards").
                               as_nanos();

    // string conversion
    let mut prefix = nwt.to_string().to_owned();
    let separator = ", ".to_owned();
    prefix.push_str(&separator);
    prefix.push_str(&write_hash.to_string());
    prefix.push_str(&separator);
    let mut suffix = data.to_owned();
    prefix.push_str(&suffix);

    let mut target_shard = write_hash.to_string().chars().next().unwrap();
    match shard_map.get::<str>(&target_shard.to_string()) {
        Some(target_file) => {
            let mut writer = LineWriter::new(target_file);
            writer.write_all(prefix.as_bytes());
            writer.write_all(b"\n");
            Ok(true)
        }
        None => Err("error writing")
    }
}

fn main() {
    let mut hmap = HashMap::new();
    let mut shard = new_shard(hmap);
}
