use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
struct EarthquakeData {
    /// 地震発生時刻
    origin_time: String,
    /// 震央北緯
    latitude: f64,
    /// 震央東経
    longitude: f64,
    /// 震源深さ (km)
    depth: f64,
    /// マグニチュード
    magnitude: f64,
    /// 観測点コード
    station_code: String,
    /// 観測点北緯
    station_latitude: f64,
    /// 観測点東経
    station_longitude: f64,
    /// 観測点標高 (m)
    station_height: f64,
    /// 記録開始時刻
    ///
    /// この時刻は強震計の遅延時間”１５秒”の効果を含んでいます。 したがって，真のスタート時刻を求めるためにはこの時刻から１５秒を引いて下さい。
    record_time: String,
    /// サンプリング周波数 (Hz)
    sampling_freq: String,
    /// 計測時間 (s)
    duration_time: f64,
    /// チャンネル
    direction: String,
    /// スケールファクタ
    scale_factor: String,
    /// 最大加速度 (gal)
    ///
    /// データ全長の平均値（オフセット）を引いた値の最大値です。
    max_acc: f64,
    /// 最終校正時刻
    last_correction: String,
    /// 備考 string
    memo: Option<String>,
    /// 強震データ
    acceleration: Vec<f64>,
}

impl EarthquakeData {
    fn new() -> Self {
        Self {
            origin_time: String::new(),
            latitude: 0.0,
            longitude: 0.0,
            depth: 0.0,
            magnitude: 0.0,
            station_code: String::new(),
            station_latitude: 0.0,
            station_longitude: 0.0,
            station_height: 0.0,
            record_time: String::new(),
            sampling_freq: String::new(),
            duration_time: 0.0,
            direction: String::new(),
            scale_factor: String::new(),
            max_acc: 0.0,
            last_correction: String::new(),
            memo: None,
            acceleration: Vec::new(),
        }
    }
}

fn parse_kv<T>(line: &str, key: &str, separator: Option<&str>) -> Option<T>
where
    T: FromStr,
{
    let regex =
        Regex::new(format!(r"{}.*\s*{}\s*(.+)", key, separator.unwrap_or(" ")).as_str()).unwrap();
    if let Some(caps) = regex.captures(line) {
        if let Ok(value) = caps[1].parse::<T>() {
            return Some(value);
        }
    }

    None
}

fn parse_kv_separated_space<T>(line: &str, key: &str) -> Option<T>
where
    T: FromStr,
{
    if let Some(value) = parse_kv::<T>(line, key, None) {
        return Some(value);
    }

    None
}

fn parse_knet(file_path: &str) -> io::Result<EarthquakeData> {
    // https://www.kyoshin.bosai.go.jp/kyoshin/man/knetform.html

    let mut data = EarthquakeData::new();

    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some(value) = parse_kv_separated_space::<String>(&line, "Origin Time") {
                    data.origin_time = value;
                } else if let Some(value) = parse_kv_separated_space::<f64>(&line, "Lat.") {
                    data.latitude = value;
                } else if let Some(value) = parse_kv_separated_space::<f64>(&line, "Long.") {
                    data.longitude = value;
                } else if let Some(value) = parse_kv_separated_space::<f64>(&line, "Depth.") {
                    data.depth = value;
                } else if let Some(value) = parse_kv_separated_space::<f64>(&line, "Mag.") {
                    data.magnitude = value;
                } else if let Some(value) =
                    parse_kv_separated_space::<String>(&line, "Station Code")
                {
                    data.station_code = value;
                } else if let Some(value) = parse_kv_separated_space::<f64>(&line, "Station Lat.") {
                    data.station_latitude = value;
                } else if let Some(value) = parse_kv_separated_space::<f64>(&line, "Station Long.")
                {
                    data.station_longitude = value;
                } else if let Some(value) = parse_kv_separated_space::<f64>(&line, "Station Height")
                {
                    data.station_height = value;
                } else if let Some(value) = parse_kv_separated_space::<String>(&line, "Record Time")
                {
                    data.record_time = value;
                } else if let Some(value) =
                    parse_kv_separated_space::<String>(&line, "Sampling Freq")
                {
                    data.sampling_freq = value;
                } else if let Some(value) = parse_kv_separated_space::<f64>(&line, "Duration Time")
                {
                    data.duration_time = value;
                } else if let Some(value) = parse_kv_separated_space::<String>(&line, "Dir.") {
                    data.direction = value;
                } else if let Some(value) =
                    parse_kv_separated_space::<String>(&line, "Scale Factor")
                {
                    data.scale_factor = value;
                } else if let Some(value) = parse_kv_separated_space::<f64>(&line, "Max. Acc.") {
                    data.max_acc = value;
                } else if let Some(value) =
                    parse_kv_separated_space::<String>(&line, "Last Correction")
                {
                    data.last_correction = value;
                } else if let Some(value) = parse_kv_separated_space::<String>(&line, "Memo.") {
                    data.memo = Some(value);
                } else {
                    let values: Vec<f64> = line
                        .split_whitespace()
                        .map(|v| v.parse().unwrap())
                        .collect();

                    data.acceleration.extend(values);
                }
            }
        }
    }

    Ok(data)
}

fn main() {
    let file_path = "data/MYG0041103111446.EW";
    match parse_knet(file_path) {
        Ok(data) => println!("{:?}", data),
        Err(e) => eprintln!("Error parsing file: {}", e),
    }
}
