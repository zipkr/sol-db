extern crate rand;
extern crate uuid;
extern crate stopwatch;

use std::fs::{self, File};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::io::LineWriter;
use std::io::prelude::*;
use std::thread;

use uuid::Uuid;
use rand::prelude::*;
use stopwatch::{Stopwatch};

struct Shard<'a> {
    path: &'a str,
}

struct ShardManager<'a> {
    hot_shards: HashMap<&'a str, &'a Shard<'a>>
}

fn create_file(s: &str) -> Result<File, &'static str> {
    let mut f = File::create(s);
    match f {
        Ok(z) => Ok(z),
        Err(e) => Err("error creating file")
    }
}

fn write_data(shard_map: &HashMap<&str, File>, data: &str) -> Result<bool, &'static str>{
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
    let mut separator = ", ".to_owned();
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
    let shards: [&'static str; 16] = [
        "0", "1", "2", "3",
        "4", "5", "6", "7",
        "8", "9", "a", "b",
        "c", "d", "e", "f"
    ];

    let mut shard_map = HashMap::new();

    for x in 0..16 {
        let mut base: String = "./db/".to_owned();
        let mut x_str: &str = &x.to_string();
        base.push_str(x_str);
        let file = create_file(&base);
        match file {
            Ok(matched_file) => shard_map.insert(shards[x], matched_file),
            Err(e) => None
        };
        
    }


    let sw = Stopwatch::start_new();
    // 1 million writes
    for x in 0..1000000 {
        match write_data(&shard_map, &x.to_string()) {
            Ok(o) => (),
            Err(e) => println!("error writing data") 
        };
    }

    println!("{}", &sw.elapsed_ms().to_string());
}
