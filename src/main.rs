mod knet;

use knet::bin::load_knet;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
struct EarthquakeData {}

impl EarthquakeData {
    fn new() -> Self {
        Self {}
    }
}

fn main() {
    let _ = EarthquakeData::new();

    let file = File::open("data/MYG0041103111446.kwin").unwrap();
    let reader = BufReader::new(file);
    println!("{:?}", load_knet(reader).unwrap());
}
