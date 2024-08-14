use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct EarthquakeData {
    magnitude: Option<f64>,            // (DIMENSIONLESS)
    depth: Option<f64>,                // KM
    delta: Option<f64>,                // KM
    sensitivity: Option<f64>,          // GAL/MM
    instrument_period: Option<f64>,    // SEC
    damping: Option<f64>,              // PERCENT
    duration_time: Option<f64>,        // SEC
    peak_acceleration: Option<f64>,    // CM/SEC/SEC
    peak_velocity: Option<f64>,        // CM/SEC
    peak_displacement: Option<f64>,    // CM
    initial_velocity: Option<f64>,     // CM/SEC
    initial_displacement: Option<f64>, // CM
}

impl EarthquakeData {
    fn new() -> Self {
        Self {
            magnitude: None,
            depth: None,
            delta: None,
            sensitivity: None,
            instrument_period: None,
            damping: None,
            duration_time: None,
            peak_acceleration: None,
            peak_velocity: None,
            peak_displacement: None,
            initial_velocity: None,
            initial_displacement: None,
        }
    }
}

fn parse_kv_f64(line: &str, key: &str) -> Option<f64> {
    let regex = Regex::new(format!(r"{}\s*=\s*{}", key, r"(-?\d+\.?\d*)").as_str()).unwrap();
    if let Some(caps) = regex.captures(line) {
        if let Ok(value) = caps[1].parse() {
            return Some(value);
        }
    }

    None
}

fn parse_earthquake_data(file_path: &str) -> io::Result<EarthquakeData> {
    let mut data = EarthquakeData::new();

    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if let None = data.magnitude {
                    data.magnitude = parse_kv_f64(&line, "MAGNITUDE");
                }
                if let None = data.depth {
                    data.depth = parse_kv_f64(&line, "DEPTH");
                }
                if let None = data.delta {
                    data.delta = parse_kv_f64(&line, "DELTA");
                }
                if let None = data.sensitivity {
                    data.sensitivity = parse_kv_f64(&line, "SENSITIVITY");
                }
                if let None = data.instrument_period {
                    data.instrument_period = parse_kv_f64(&line, "INSTRUMENT PERIOD");
                }
                if let None = data.damping {
                    data.damping = parse_kv_f64(&line, "DAMPING");
                }
                if let None = data.duration_time {
                    data.duration_time = parse_kv_f64(&line, "DURATION TIME");
                }
                if let None = data.peak_acceleration {
                    data.peak_acceleration = parse_kv_f64(&line, "PEAK ACCELERATION");
                }
                if let None = data.peak_velocity {
                    data.peak_velocity = parse_kv_f64(&line, "PEAK VELOCITY");
                }
                if let None = data.peak_displacement {
                    data.peak_displacement = parse_kv_f64(&line, "PEAK DISPLACEMENT");
                }
                if let None = data.initial_velocity {
                    data.initial_velocity = parse_kv_f64(&line, "INITIAL VELOCITY");
                }
                if let None = data.initial_displacement {
                    data.initial_displacement = parse_kv_f64(&line, "INITIAL DISP.");
                }
            }
        }
    }

    Ok(data)
}

fn main() {
    let file_path = "data/86-Elcentew.txt";
    match parse_earthquake_data(file_path) {
        Ok(data) => println!("{:?}", data),
        Err(e) => eprintln!("Error parsing file: {}", e),
    }
}
