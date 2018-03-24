extern crate rand;

use rand::Rng;
use std::{fs, process, time};
use std::io::Read;

fn main() {
    let mut config = String::new();
    fs::File::open("/etc/shellstorm").unwrap().read_to_string(&mut config).unwrap();

    let bins = config.lines().collect::<Vec<&str>>();

    let mut rng = rand::ChaChaRng::new_unseeded();

    let duration = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
    rng.set_counter(duration.as_secs(), duration.subsec_nanos() as u64);

    loop {
        let i = rng.gen_range(0, bins.len());
        let bin = &bins[i];
        eprintln!("shellstorm: {}", bin);
        process::Command::new(bin).status().unwrap();
    }
}
