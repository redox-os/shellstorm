extern crate rand;

use rand::Rng;
use std::{fs, io, path, process, time};

fn bins() -> io::Result<Vec<path::PathBuf>> {
    let mut paths = Vec::new();

    for entry_res in fs::read_dir("/bin")? {
        let entry = entry_res?;
        paths.push(entry.path());
    }

    Ok(paths)
}

fn main() {
    let bins = bins().unwrap();

    let mut rng = rand::ChaChaRng::new_unseeded();

    let duration = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
    rng.set_counter(duration.as_secs(), duration.subsec_nanos() as u64);

    loop {
        let i = rng.gen_range(0, bins.len());
        let bin = bins[i];
        process::Command::new(bin).status().unwrap();
    }
}
