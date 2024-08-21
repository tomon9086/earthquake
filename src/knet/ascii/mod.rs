mod util;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use util::*;

#[derive(Debug)]
pub struct KnetAsciiEarthquakeData {
    /// 地震発生時刻
    pub origin_time: String,
    /// 震央北緯
    pub latitude: f64,
    /// 震央東経
    pub longitude: f64,
    /// 震源深さ (km)
    pub depth: f64,
    /// マグニチュード
    pub magnitude: f64,
    /// 観測点コード
    pub station_code: String,
    /// 観測点北緯
    pub station_latitude: f64,
    /// 観測点東経
    pub station_longitude: f64,
    /// 観測点標高 (m)
    pub station_height: f64,
    /// 記録開始時刻
    ///
    /// この時刻は強震計の遅延時間”１５秒”の効果を含んでいます。 したがって，真のスタート時刻を求めるためにはこの時刻から１５秒を引いて下さい。
    pub record_time: String,
    /// サンプリング周波数 (Hz)
    pub sampling_freq: String,
    /// 計測時間 (s)
    pub duration_time: f64,
    /// チャンネル
    pub direction: String,
    /// スケールファクタ
    pub scale_factor: String,
    /// 最大加速度 (gal)
    ///
    /// データ全長の平均値（オフセット）を引いた値の最大値です。
    pub max_acc: f64,
    /// 最終校正時刻
    pub last_correction: String,
    /// 備考 string
    pub memo: Option<String>,
    /// 強震データ NS
    pub acceleration_ns: Vec<f64>,
    /// 強震データ EW
    pub acceleration_ew: Vec<f64>,
    /// 強震データ UD
    pub acceleration_ud: Vec<f64>,
}

#[allow(dead_code)]
pub fn parse_knet(reader: BufReader<File>) -> io::Result<KnetAsciiEarthquakeData> {
    // https://www.kyoshin.bosai.go.jp/kyoshin/man/knetform.html

    let mut data = KnetAsciiEarthquakeData {
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
        acceleration_ns: Vec::new(),
        acceleration_ew: Vec::new(),
        acceleration_ud: Vec::new(),
    };

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
            } else if let Some(value) = parse_kv_separated_space::<String>(&line, "Station Code") {
                data.station_code = value;
            } else if let Some(value) = parse_kv_separated_space::<f64>(&line, "Station Lat.") {
                data.station_latitude = value;
            } else if let Some(value) = parse_kv_separated_space::<f64>(&line, "Station Long.") {
                data.station_longitude = value;
            } else if let Some(value) = parse_kv_separated_space::<f64>(&line, "Station Height") {
                data.station_height = value;
            } else if let Some(value) = parse_kv_separated_space::<String>(&line, "Record Time") {
                data.record_time = value;
            } else if let Some(value) = parse_kv_separated_space::<String>(&line, "Sampling Freq") {
                data.sampling_freq = value;
            } else if let Some(value) = parse_kv_separated_space::<f64>(&line, "Duration Time") {
                data.duration_time = value;
            } else if let Some(value) = parse_kv_separated_space::<String>(&line, "Dir.") {
                data.direction = value;
            } else if let Some(value) = parse_kv_separated_space::<String>(&line, "Scale Factor") {
                data.scale_factor = value;
            } else if let Some(value) = parse_kv_separated_space::<f64>(&line, "Max. Acc.") {
                data.max_acc = value;
            } else if let Some(value) = parse_kv_separated_space::<String>(&line, "Last Correction")
            {
                data.last_correction = value;
            } else if let Some(value) = parse_kv_separated_space::<String>(&line, "Memo.") {
                data.memo = Some(value);
            } else {
                let values: Vec<f64> = line
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect();

                data.acceleration_ew.extend(values);
            }
        }
    }

    Ok(data)
}
