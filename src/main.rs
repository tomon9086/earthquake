use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "data/86-Elcentew.txt";

    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    }

    Ok(())
}
