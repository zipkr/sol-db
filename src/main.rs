extern crate rand;
extern crate uuid;

use std::fs::{self, File};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use uuid::Uuid;

use rand::prelude::*;

struct Shard<'a> {
    path: &'a str,
}

struct ShardManager<'a> {
    hot_shards: HashMap<&'a str, &'a Shard<'a>>
}

fn create_file(s: &str) -> Result<File, Box<Error>> {
    let mut f = File::create(s);
    Ok(f.unwrap())
}

fn write_data(shard_map: HashMap<&str, File>, data: &str) {
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
    println!("{}", prefix);

    let mut target_shard = write_hash.to_string().chars().next().unwrap();
    match shard_map.get::<str>(&target_shard.to_string()) {
        Some(target_file) => fs::write(target_file, prefix).expect("Unable to write data"),
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
        shard_map.insert(shards[x], file);
    }
}
